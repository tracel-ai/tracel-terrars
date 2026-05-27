use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DeveloperConnectConnectionData {
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
    connection_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket_cloud_config: Option<Vec<DeveloperConnectConnectionBitbucketCloudConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket_data_center_config:
        Option<Vec<DeveloperConnectConnectionBitbucketDataCenterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crypto_key_config: Option<Vec<DeveloperConnectConnectionCryptoKeyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_config: Option<Vec<DeveloperConnectConnectionGithubConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_enterprise_config: Option<Vec<DeveloperConnectConnectionGithubEnterpriseConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab_config: Option<Vec<DeveloperConnectConnectionGitlabConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab_enterprise_config: Option<Vec<DeveloperConnectConnectionGitlabEnterpriseConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_config: Option<Vec<DeveloperConnectConnectionHttpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DeveloperConnectConnectionTimeoutsEl>,
    dynamic: DeveloperConnectConnectionDynamic,
}
struct DeveloperConnectConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DeveloperConnectConnectionData>,
}
#[derive(Clone)]
pub struct DeveloperConnectConnection(Rc<DeveloperConnectConnection_>);
impl DeveloperConnectConnection {
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
    #[doc = "Set the field `annotations`.\nOptional. Allows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nOptional. If disabled is set to true, functionality is disabled for this connection.\nRepository based API methods and webhooks processing for repositories in\nthis connection will be disabled."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\nOptional. This checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
    pub fn set_etag(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().etag = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Labels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
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
        v: impl Into<BlockAssignable<DeveloperConnectConnectionBitbucketCloudConfigEl>>,
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
        v: impl Into<BlockAssignable<DeveloperConnectConnectionBitbucketDataCenterConfigEl>>,
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
    #[doc = "Set the field `crypto_key_config`.\n"]
    pub fn set_crypto_key_config(
        self,
        v: impl Into<BlockAssignable<DeveloperConnectConnectionCryptoKeyConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().crypto_key_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.crypto_key_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `github_config`.\n"]
    pub fn set_github_config(
        self,
        v: impl Into<BlockAssignable<DeveloperConnectConnectionGithubConfigEl>>,
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
        v: impl Into<BlockAssignable<DeveloperConnectConnectionGithubEnterpriseConfigEl>>,
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
        v: impl Into<BlockAssignable<DeveloperConnectConnectionGitlabConfigEl>>,
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
    #[doc = "Set the field `gitlab_enterprise_config`.\n"]
    pub fn set_gitlab_enterprise_config(
        self,
        v: impl Into<BlockAssignable<DeveloperConnectConnectionGitlabEnterpriseConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gitlab_enterprise_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gitlab_enterprise_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_config`.\n"]
    pub fn set_http_config(
        self,
        v: impl Into<BlockAssignable<DeveloperConnectConnectionHttpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DeveloperConnectConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. Allows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\nRequired. Id of the requesting object\nIf auto-generating Id server-side, remove this field and\nconnection_id from the method_signature of Create RPC"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. [Output only] Create timestamp"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. [Output only] Delete timestamp"]
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
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nOptional. If disabled is set to true, functionality is disabled for this connection.\nRepository based API methods and webhooks processing for repositories in\nthis connection will be disabled."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. This checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `installation_state` after provisioning.\nDescribes stage and necessary actions to be taken by the\nuser to complete the installation. Used for GitHub and GitHub Enterprise\nbased connections."]
    pub fn installation_state(&self) -> ListRef<DeveloperConnectConnectionInstallationStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.installation_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the connection, in the format\n'projects/{project}/locations/{location}/connections/{connection_id}'."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true when the connection is being set up or updated in the\nbackground."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A system-assigned unique identifier for a the GitRepositoryLink."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. [Output only] Update timestamp"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_cloud_config` after provisioning.\n"]
    pub fn bitbucket_cloud_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionBitbucketCloudConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_cloud_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_data_center_config` after provisioning.\n"]
    pub fn bitbucket_data_center_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionBitbucketDataCenterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_data_center_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key_config` after provisioning.\n"]
    pub fn crypto_key_config(&self) -> ListRef<DeveloperConnectConnectionCryptoKeyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.crypto_key_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github_config` after provisioning.\n"]
    pub fn github_config(&self) -> ListRef<DeveloperConnectConnectionGithubConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionGithubEnterpriseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github_enterprise_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gitlab_config` after provisioning.\n"]
    pub fn gitlab_config(&self) -> ListRef<DeveloperConnectConnectionGitlabConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gitlab_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gitlab_enterprise_config` after provisioning.\n"]
    pub fn gitlab_enterprise_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionGitlabEnterpriseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gitlab_enterprise_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_config` after provisioning.\n"]
    pub fn http_config(&self) -> ListRef<DeveloperConnectConnectionHttpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectConnectionTimeoutsElRef {
        DeveloperConnectConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DeveloperConnectConnection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DeveloperConnectConnection {}
impl ToListMappable for DeveloperConnectConnection {
    type O = ListRef<DeveloperConnectConnectionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DeveloperConnectConnection_ {
    fn extract_resource_type(&self) -> String {
        "google_developer_connect_connection".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDeveloperConnectConnection {
    pub tf_id: String,
    #[doc = "Required. Id of the requesting object\nIf auto-generating Id server-side, remove this field and\nconnection_id from the method_signature of Create RPC"]
    pub connection_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildDeveloperConnectConnection {
    pub fn build(self, stack: &mut Stack) -> DeveloperConnectConnection {
        let out = DeveloperConnectConnection(Rc::new(DeveloperConnectConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DeveloperConnectConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                connection_id: self.connection_id,
                deletion_policy: core::default::Default::default(),
                disabled: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                bitbucket_cloud_config: core::default::Default::default(),
                bitbucket_data_center_config: core::default::Default::default(),
                crypto_key_config: core::default::Default::default(),
                github_config: core::default::Default::default(),
                github_enterprise_config: core::default::Default::default(),
                gitlab_config: core::default::Default::default(),
                gitlab_enterprise_config: core::default::Default::default(),
                http_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DeveloperConnectConnectionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DeveloperConnectConnectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. Allows clients to store small amounts of arbitrary data.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\nRequired. Id of the requesting object\nIf auto-generating Id server-side, remove this field and\nconnection_id from the method_signature of Create RPC"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. [Output only] Create timestamp"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. [Output only] Delete timestamp"]
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
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nOptional. If disabled is set to true, functionality is disabled for this connection.\nRepository based API methods and webhooks processing for repositories in\nthis connection will be disabled."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. This checksum is computed by the server based on the value of other\nfields, and may be sent on update and delete requests to ensure the\nclient has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `installation_state` after provisioning.\nDescribes stage and necessary actions to be taken by the\nuser to complete the installation. Used for GitHub and GitHub Enterprise\nbased connections."]
    pub fn installation_state(&self) -> ListRef<DeveloperConnectConnectionInstallationStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.installation_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the connection, in the format\n'projects/{project}/locations/{location}/connections/{connection_id}'."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Set to true when the connection is being set up or updated in the\nbackground."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A system-assigned unique identifier for a the GitRepositoryLink."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. [Output only] Update timestamp"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_cloud_config` after provisioning.\n"]
    pub fn bitbucket_cloud_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionBitbucketCloudConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_cloud_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_data_center_config` after provisioning.\n"]
    pub fn bitbucket_data_center_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionBitbucketDataCenterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_data_center_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `crypto_key_config` after provisioning.\n"]
    pub fn crypto_key_config(&self) -> ListRef<DeveloperConnectConnectionCryptoKeyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.crypto_key_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github_config` after provisioning.\n"]
    pub fn github_config(&self) -> ListRef<DeveloperConnectConnectionGithubConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionGithubEnterpriseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github_enterprise_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gitlab_config` after provisioning.\n"]
    pub fn gitlab_config(&self) -> ListRef<DeveloperConnectConnectionGitlabConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gitlab_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gitlab_enterprise_config` after provisioning.\n"]
    pub fn gitlab_enterprise_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionGitlabEnterpriseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gitlab_enterprise_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_config` after provisioning.\n"]
    pub fn http_config(&self) -> ListRef<DeveloperConnectConnectionHttpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectConnectionTimeoutsElRef {
        DeveloperConnectConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionInstallationStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<PrimField<String>>,
}
impl DeveloperConnectConnectionInstallationStateEl {
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
impl ToListMappable for DeveloperConnectConnectionInstallationStateEl {
    type O = BlockAssignable<DeveloperConnectConnectionInstallationStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionInstallationStateEl {}
impl BuildDeveloperConnectConnectionInstallationStateEl {
    pub fn build(self) -> DeveloperConnectConnectionInstallationStateEl {
        DeveloperConnectConnectionInstallationStateEl {
            action_uri: core::default::Default::default(),
            message: core::default::Default::default(),
            stage: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectConnectionInstallationStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionInstallationStateElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectConnectionInstallationStateElRef {
        DeveloperConnectConnectionInstallationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionInstallationStateElRef {
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
pub struct DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialEl {}
impl ToListMappable for DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
    type O =
        BlockAssignable<DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
    pub fn build(self) -> DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
        DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialElRef {
        DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {}
impl ToListMappable for DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
    type O =
        BlockAssignable<DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
    pub fn build(
        self,
    ) -> DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
        DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef {
        DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct DeveloperConnectConnectionBitbucketCloudConfigElDynamic {
    authorizer_credential: Option<
        DynamicBlock<DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialEl>,
    >,
    read_authorizer_credential: Option<
        DynamicBlock<DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl>,
    >,
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionBitbucketCloudConfigEl {
    webhook_secret_secret_version: PrimField<String>,
    workspace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_credential:
        Option<Vec<DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_authorizer_credential:
        Option<Vec<DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl>>,
    dynamic: DeveloperConnectConnectionBitbucketCloudConfigElDynamic,
}
impl DeveloperConnectConnectionBitbucketCloudConfigEl {
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v: impl Into<
            BlockAssignable<DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialEl>,
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
                DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialEl,
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
}
impl ToListMappable for DeveloperConnectConnectionBitbucketCloudConfigEl {
    type O = BlockAssignable<DeveloperConnectConnectionBitbucketCloudConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionBitbucketCloudConfigEl {
    #[doc = "Required. Immutable. SecretManager resource containing the webhook secret used to verify webhook\nevents, formatted as 'projects/*/secrets/*/versions/*'. This is used to\nvalidate and create webhooks."]
    pub webhook_secret_secret_version: PrimField<String>,
    #[doc = "Required. The Bitbucket Cloud Workspace ID to be connected to Google Cloud Platform."]
    pub workspace: PrimField<String>,
}
impl BuildDeveloperConnectConnectionBitbucketCloudConfigEl {
    pub fn build(self) -> DeveloperConnectConnectionBitbucketCloudConfigEl {
        DeveloperConnectConnectionBitbucketCloudConfigEl {
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            workspace: self.workspace,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DeveloperConnectConnectionBitbucketCloudConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionBitbucketCloudConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionBitbucketCloudConfigElRef {
        DeveloperConnectConnectionBitbucketCloudConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionBitbucketCloudConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret used to verify webhook\nevents, formatted as 'projects/*/secrets/*/versions/*'. This is used to\nvalidate and create webhooks."]
    pub fn webhook_secret_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_secret_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workspace` after provisioning.\nRequired. The Bitbucket Cloud Workspace ID to be connected to Google Cloud Platform."]
    pub fn workspace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace", self.base))
    }
    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> ListRef<DeveloperConnectConnectionBitbucketCloudConfigElAuthorizerCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_credential", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> ListRef<DeveloperConnectConnectionBitbucketCloudConfigElReadAuthorizerCredentialElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_authorizer_credential", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {}
impl ToListMappable
    for DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl
{
    type O = BlockAssignable<
        DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {
    pub fn build(
        self,
    ) -> DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {
        DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef {
        DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {}
impl ToListMappable
    for DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl
{
    type O = BlockAssignable<
        DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {
    pub fn build(
        self,
    ) -> DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {
        DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef {
        DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {}
impl ToListMappable
    for DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl
{
    type O = BlockAssignable<
        DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {
    #[doc = "Required. The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: PrimField<String>,
}
impl BuildDeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {
    pub fn build(
        self,
    ) -> DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {
        DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef {
        DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nRequired. The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize, Default)]
struct DeveloperConnectConnectionBitbucketDataCenterConfigElDynamic {
    authorizer_credential: Option<
        DynamicBlock<DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl>,
    >,
    read_authorizer_credential: Option<
        DynamicBlock<
            DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl,
        >,
    >,
    service_directory_config: Option<
        DynamicBlock<DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionBitbucketDataCenterConfigEl {
    host_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca_certificate: Option<PrimField<String>>,
    webhook_secret_secret_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_credential:
        Option<Vec<DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_authorizer_credential: Option<
        Vec<DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config:
        Option<Vec<DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl>>,
    dynamic: DeveloperConnectConnectionBitbucketDataCenterConfigElDynamic,
}
impl DeveloperConnectConnectionBitbucketDataCenterConfigEl {
    #[doc = "Set the field `ssl_ca_certificate`.\nOptional. SSL certificate authority to trust when making requests to Bitbucket Data\nCenter."]
    pub fn set_ssl_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v: impl Into<
            BlockAssignable<
                DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialEl,
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
                DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialEl,
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
                DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigEl,
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
impl ToListMappable for DeveloperConnectConnectionBitbucketDataCenterConfigEl {
    type O = BlockAssignable<DeveloperConnectConnectionBitbucketDataCenterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionBitbucketDataCenterConfigEl {
    #[doc = "Required. The URI of the Bitbucket Data Center host this connection is for."]
    pub host_uri: PrimField<String>,
    #[doc = "Required. Immutable. SecretManager resource containing the webhook secret used to verify webhook\nevents, formatted as 'projects/*/secrets/*/versions/*'. This is used to\nvalidate webhooks."]
    pub webhook_secret_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionBitbucketDataCenterConfigEl {
    pub fn build(self) -> DeveloperConnectConnectionBitbucketDataCenterConfigEl {
        DeveloperConnectConnectionBitbucketDataCenterConfigEl {
            host_uri: self.host_uri,
            ssl_ca_certificate: core::default::Default::default(),
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DeveloperConnectConnectionBitbucketDataCenterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionBitbucketDataCenterConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionBitbucketDataCenterConfigElRef {
        DeveloperConnectConnectionBitbucketDataCenterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionBitbucketDataCenterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_uri` after provisioning.\nRequired. The URI of the Bitbucket Data Center host this connection is for."]
    pub fn host_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `server_version` after provisioning.\nOutput only. Version of the Bitbucket Data Center server running on the 'host_uri'."]
    pub fn server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_ca_certificate` after provisioning.\nOptional. SSL certificate authority to trust when making requests to Bitbucket Data\nCenter."]
    pub fn ssl_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret used to verify webhook\nevents, formatted as 'projects/*/secrets/*/versions/*'. This is used to\nvalidate webhooks."]
    pub fn webhook_secret_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_secret_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> ListRef<DeveloperConnectConnectionBitbucketDataCenterConfigElAuthorizerCredentialElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_credential", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> ListRef<DeveloperConnectConnectionBitbucketDataCenterConfigElReadAuthorizerCredentialElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_authorizer_credential", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionBitbucketDataCenterConfigElServiceDirectoryConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionCryptoKeyConfigEl {
    key_reference: PrimField<String>,
}
impl DeveloperConnectConnectionCryptoKeyConfigEl {}
impl ToListMappable for DeveloperConnectConnectionCryptoKeyConfigEl {
    type O = BlockAssignable<DeveloperConnectConnectionCryptoKeyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionCryptoKeyConfigEl {
    #[doc = "Required. The name of the key which is used to encrypt/decrypt customer data. For key\nin Cloud KMS, the key should be in the format of\n'projects/*/locations/*/keyRings/*/cryptoKeys/*'."]
    pub key_reference: PrimField<String>,
}
impl BuildDeveloperConnectConnectionCryptoKeyConfigEl {
    pub fn build(self) -> DeveloperConnectConnectionCryptoKeyConfigEl {
        DeveloperConnectConnectionCryptoKeyConfigEl {
            key_reference: self.key_reference,
        }
    }
}
pub struct DeveloperConnectConnectionCryptoKeyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionCryptoKeyConfigElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectConnectionCryptoKeyConfigElRef {
        DeveloperConnectConnectionCryptoKeyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionCryptoKeyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key_reference` after provisioning.\nRequired. The name of the key which is used to encrypt/decrypt customer data. For key\nin Cloud KMS, the key should be in the format of\n'projects/*/locations/*/keyRings/*/cryptoKeys/*'."]
    pub fn key_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_reference", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionGithubConfigElAuthorizerCredentialEl {
    oauth_token_secret_version: PrimField<String>,
}
impl DeveloperConnectConnectionGithubConfigElAuthorizerCredentialEl {}
impl ToListMappable for DeveloperConnectConnectionGithubConfigElAuthorizerCredentialEl {
    type O = BlockAssignable<DeveloperConnectConnectionGithubConfigElAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionGithubConfigElAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the OAuth token that authorizes\nthe connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub oauth_token_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionGithubConfigElAuthorizerCredentialEl {
    pub fn build(self) -> DeveloperConnectConnectionGithubConfigElAuthorizerCredentialEl {
        DeveloperConnectConnectionGithubConfigElAuthorizerCredentialEl {
            oauth_token_secret_version: self.oauth_token_secret_version,
        }
    }
}
pub struct DeveloperConnectConnectionGithubConfigElAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionGithubConfigElAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionGithubConfigElAuthorizerCredentialElRef {
        DeveloperConnectConnectionGithubConfigElAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionGithubConfigElAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `oauth_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the OAuth token that authorizes\nthe connection. Format: 'projects/*/secrets/*/versions/*'."]
    pub fn oauth_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct DeveloperConnectConnectionGithubConfigElDynamic {
    authorizer_credential:
        Option<DynamicBlock<DeveloperConnectConnectionGithubConfigElAuthorizerCredentialEl>>,
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionGithubConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_installation_id: Option<PrimField<String>>,
    github_app: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_credential:
        Option<Vec<DeveloperConnectConnectionGithubConfigElAuthorizerCredentialEl>>,
    dynamic: DeveloperConnectConnectionGithubConfigElDynamic,
}
impl DeveloperConnectConnectionGithubConfigEl {
    #[doc = "Set the field `app_installation_id`.\nOptional. GitHub App installation id."]
    pub fn set_app_installation_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.app_installation_id = Some(v.into());
        self
    }
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v: impl Into<BlockAssignable<DeveloperConnectConnectionGithubConfigElAuthorizerCredentialEl>>,
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
impl ToListMappable for DeveloperConnectConnectionGithubConfigEl {
    type O = BlockAssignable<DeveloperConnectConnectionGithubConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionGithubConfigEl {
    #[doc = "Required. Immutable. The GitHub Application that was installed to the GitHub user or\norganization.\nPossible values:\nGIT_HUB_APP_UNSPECIFIED\nDEVELOPER_CONNECT\nFIREBASE"]
    pub github_app: PrimField<String>,
}
impl BuildDeveloperConnectConnectionGithubConfigEl {
    pub fn build(self) -> DeveloperConnectConnectionGithubConfigEl {
        DeveloperConnectConnectionGithubConfigEl {
            app_installation_id: core::default::Default::default(),
            github_app: self.github_app,
            authorizer_credential: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DeveloperConnectConnectionGithubConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionGithubConfigElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectConnectionGithubConfigElRef {
        DeveloperConnectConnectionGithubConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionGithubConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_installation_id` after provisioning.\nOptional. GitHub App installation id."]
    pub fn app_installation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_installation_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `github_app` after provisioning.\nRequired. Immutable. The GitHub Application that was installed to the GitHub user or\norganization.\nPossible values:\nGIT_HUB_APP_UNSPECIFIED\nDEVELOPER_CONNECT\nFIREBASE"]
    pub fn github_app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.github_app", self.base))
    }
    #[doc = "Get a reference to the value of field `installation_uri` after provisioning.\nOutput only. The URI to navigate to in order to manage the installation associated\nwith this GitHubConfig."]
    pub fn installation_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.installation_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> ListRef<DeveloperConnectConnectionGithubConfigElAuthorizerCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_credential", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {}
impl ToListMappable for DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    type O =
        BlockAssignable<DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    #[doc = "Required. The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: PrimField<String>,
}
impl BuildDeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
    pub fn build(
        self,
    ) -> DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
        DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
        DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nRequired. The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize, Default)]
struct DeveloperConnectConnectionGithubEnterpriseConfigElDynamic {
    service_directory_config: Option<
        DynamicBlock<DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionGithubEnterpriseConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_installation_id: Option<PrimField<String>>,
    host_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key_secret_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook_secret_secret_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config:
        Option<Vec<DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl>>,
    dynamic: DeveloperConnectConnectionGithubEnterpriseConfigElDynamic,
}
impl DeveloperConnectConnectionGithubEnterpriseConfigEl {
    #[doc = "Set the field `app_id`.\nOptional. ID of the GitHub App created from the manifest."]
    pub fn set_app_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.app_id = Some(v.into());
        self
    }
    #[doc = "Set the field `app_installation_id`.\nOptional. ID of the installation of the GitHub App."]
    pub fn set_app_installation_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.app_installation_id = Some(v.into());
        self
    }
    #[doc = "Set the field `private_key_secret_version`.\nOptional. SecretManager resource containing the private key of the GitHub App,\nformatted as 'projects/*/secrets/*/versions/*'."]
    pub fn set_private_key_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_key_secret_version = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_ca_certificate`.\nOptional. SSL certificate to use for requests to GitHub Enterprise."]
    pub fn set_ssl_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `webhook_secret_secret_version`.\nOptional. SecretManager resource containing the webhook secret of the GitHub App,\nformatted as 'projects/*/secrets/*/versions/*'."]
    pub fn set_webhook_secret_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webhook_secret_secret_version = Some(v.into());
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigEl,
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
impl ToListMappable for DeveloperConnectConnectionGithubEnterpriseConfigEl {
    type O = BlockAssignable<DeveloperConnectConnectionGithubEnterpriseConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionGithubEnterpriseConfigEl {
    #[doc = "Required. The URI of the GitHub Enterprise host this connection is for."]
    pub host_uri: PrimField<String>,
}
impl BuildDeveloperConnectConnectionGithubEnterpriseConfigEl {
    pub fn build(self) -> DeveloperConnectConnectionGithubEnterpriseConfigEl {
        DeveloperConnectConnectionGithubEnterpriseConfigEl {
            app_id: core::default::Default::default(),
            app_installation_id: core::default::Default::default(),
            host_uri: self.host_uri,
            private_key_secret_version: core::default::Default::default(),
            ssl_ca_certificate: core::default::Default::default(),
            webhook_secret_secret_version: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DeveloperConnectConnectionGithubEnterpriseConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionGithubEnterpriseConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionGithubEnterpriseConfigElRef {
        DeveloperConnectConnectionGithubEnterpriseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionGithubEnterpriseConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nOptional. ID of the GitHub App created from the manifest."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.base))
    }
    #[doc = "Get a reference to the value of field `app_installation_id` after provisioning.\nOptional. ID of the installation of the GitHub App."]
    pub fn app_installation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_installation_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `app_slug` after provisioning.\nOutput only. The URL-friendly name of the GitHub App."]
    pub fn app_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_slug", self.base))
    }
    #[doc = "Get a reference to the value of field `host_uri` after provisioning.\nRequired. The URI of the GitHub Enterprise host this connection is for."]
    pub fn host_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `installation_uri` after provisioning.\nOutput only. The URI to navigate to in order to manage the installation associated\nwith this GitHubEnterpriseConfig."]
    pub fn installation_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.installation_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_key_secret_version` after provisioning.\nOptional. SecretManager resource containing the private key of the GitHub App,\nformatted as 'projects/*/secrets/*/versions/*'."]
    pub fn private_key_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_key_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `server_version` after provisioning.\nOutput only. GitHub Enterprise version installed at the host_uri."]
    pub fn server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_ca_certificate` after provisioning.\nOptional. SSL certificate to use for requests to GitHub Enterprise."]
    pub fn ssl_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nOptional. SecretManager resource containing the webhook secret of the GitHub App,\nformatted as 'projects/*/secrets/*/versions/*'."]
    pub fn webhook_secret_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_secret_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionGithubEnterpriseConfigElServiceDirectoryConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialEl {}
impl ToListMappable for DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialEl {
    type O = BlockAssignable<DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionGitlabConfigElAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionGitlabConfigElAuthorizerCredentialEl {
    pub fn build(self) -> DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialEl {
        DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialElRef {
        DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialEl {}
impl ToListMappable for DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialEl {
    type O = BlockAssignable<DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialEl {
    pub fn build(self) -> DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialEl {
        DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialElRef {
        DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct DeveloperConnectConnectionGitlabConfigElDynamic {
    authorizer_credential:
        Option<DynamicBlock<DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialEl>>,
    read_authorizer_credential:
        Option<DynamicBlock<DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialEl>>,
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionGitlabConfigEl {
    webhook_secret_secret_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_credential:
        Option<Vec<DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_authorizer_credential:
        Option<Vec<DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialEl>>,
    dynamic: DeveloperConnectConnectionGitlabConfigElDynamic,
}
impl DeveloperConnectConnectionGitlabConfigEl {
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v: impl Into<BlockAssignable<DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialEl>>,
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
            BlockAssignable<DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialEl>,
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
impl ToListMappable for DeveloperConnectConnectionGitlabConfigEl {
    type O = BlockAssignable<DeveloperConnectConnectionGitlabConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionGitlabConfigEl {
    #[doc = "Required. Immutable. SecretManager resource containing the webhook secret of a GitLab project,\nformatted as 'projects/*/secrets/*/versions/*'. This is used to validate\nwebhooks."]
    pub webhook_secret_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionGitlabConfigEl {
    pub fn build(self) -> DeveloperConnectConnectionGitlabConfigEl {
        DeveloperConnectConnectionGitlabConfigEl {
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DeveloperConnectConnectionGitlabConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionGitlabConfigElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectConnectionGitlabConfigElRef {
        DeveloperConnectConnectionGitlabConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionGitlabConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret of a GitLab project,\nformatted as 'projects/*/secrets/*/versions/*'. This is used to validate\nwebhooks."]
    pub fn webhook_secret_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_secret_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> ListRef<DeveloperConnectConnectionGitlabConfigElAuthorizerCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_credential", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> ListRef<DeveloperConnectConnectionGitlabConfigElReadAuthorizerCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_authorizer_credential", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialEl {}
impl ToListMappable for DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialEl {
    type O =
        BlockAssignable<DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialEl {
    pub fn build(self) -> DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialEl {
        DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialElRef {
        DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialEl {
    user_token_secret_version: PrimField<String>,
}
impl DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialEl {}
impl ToListMappable
    for DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialEl
{
    type O = BlockAssignable<
        DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialEl {
    #[doc = "Required. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub user_token_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialEl {
    pub fn build(
        self,
    ) -> DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialEl {
        DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialEl {
            user_token_secret_version: self.user_token_secret_version,
        }
    }
}
pub struct DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialElRef {
        DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_token_secret_version` after provisioning.\nRequired. A SecretManager resource containing the user token that authorizes\nthe Developer Connect connection. Format:\n'projects/*/secrets/*/versions/*'."]
    pub fn user_token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_token_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nOutput only. The username associated with this token."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigEl {}
impl ToListMappable for DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigEl {
    type O =
        BlockAssignable<DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigEl {
    #[doc = "Required. The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: PrimField<String>,
}
impl BuildDeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigEl {
    pub fn build(
        self,
    ) -> DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigEl {
        DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigElRef {
        DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nRequired. The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize, Default)]
struct DeveloperConnectConnectionGitlabEnterpriseConfigElDynamic {
    authorizer_credential: Option<
        DynamicBlock<DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialEl>,
    >,
    read_authorizer_credential: Option<
        DynamicBlock<DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialEl>,
    >,
    service_directory_config: Option<
        DynamicBlock<DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionGitlabEnterpriseConfigEl {
    host_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca_certificate: Option<PrimField<String>>,
    webhook_secret_secret_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_credential:
        Option<Vec<DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_authorizer_credential:
        Option<Vec<DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config:
        Option<Vec<DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigEl>>,
    dynamic: DeveloperConnectConnectionGitlabEnterpriseConfigElDynamic,
}
impl DeveloperConnectConnectionGitlabEnterpriseConfigEl {
    #[doc = "Set the field `ssl_ca_certificate`.\nOptional. SSL Certificate Authority certificate to use for requests to GitLab\nEnterprise instance."]
    pub fn set_ssl_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `authorizer_credential`.\n"]
    pub fn set_authorizer_credential(
        mut self,
        v: impl Into<
            BlockAssignable<
                DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialEl,
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
                DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialEl,
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
                DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigEl,
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
impl ToListMappable for DeveloperConnectConnectionGitlabEnterpriseConfigEl {
    type O = BlockAssignable<DeveloperConnectConnectionGitlabEnterpriseConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionGitlabEnterpriseConfigEl {
    #[doc = "Required. The URI of the GitLab Enterprise host this connection is for."]
    pub host_uri: PrimField<String>,
    #[doc = "Required. Immutable. SecretManager resource containing the webhook secret of a GitLab project,\nformatted as 'projects/*/secrets/*/versions/*'. This is used to validate\nwebhooks."]
    pub webhook_secret_secret_version: PrimField<String>,
}
impl BuildDeveloperConnectConnectionGitlabEnterpriseConfigEl {
    pub fn build(self) -> DeveloperConnectConnectionGitlabEnterpriseConfigEl {
        DeveloperConnectConnectionGitlabEnterpriseConfigEl {
            host_uri: self.host_uri,
            ssl_ca_certificate: core::default::Default::default(),
            webhook_secret_secret_version: self.webhook_secret_secret_version,
            authorizer_credential: core::default::Default::default(),
            read_authorizer_credential: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DeveloperConnectConnectionGitlabEnterpriseConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionGitlabEnterpriseConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionGitlabEnterpriseConfigElRef {
        DeveloperConnectConnectionGitlabEnterpriseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionGitlabEnterpriseConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_uri` after provisioning.\nRequired. The URI of the GitLab Enterprise host this connection is for."]
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
    #[doc = "Get a reference to the value of field `ssl_ca_certificate` after provisioning.\nOptional. SSL Certificate Authority certificate to use for requests to GitLab\nEnterprise instance."]
    pub fn ssl_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_secret_secret_version` after provisioning.\nRequired. Immutable. SecretManager resource containing the webhook secret of a GitLab project,\nformatted as 'projects/*/secrets/*/versions/*'. This is used to validate\nwebhooks."]
    pub fn webhook_secret_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_secret_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorizer_credential` after provisioning.\n"]
    pub fn authorizer_credential(
        &self,
    ) -> ListRef<DeveloperConnectConnectionGitlabEnterpriseConfigElAuthorizerCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_credential", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_authorizer_credential` after provisioning.\n"]
    pub fn read_authorizer_credential(
        &self,
    ) -> ListRef<DeveloperConnectConnectionGitlabEnterpriseConfigElReadAuthorizerCredentialElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_authorizer_credential", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionGitlabEnterpriseConfigElServiceDirectoryConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionHttpConfigElBasicAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password_secret_version: Option<PrimField<String>>,
    username: PrimField<String>,
}
impl DeveloperConnectConnectionHttpConfigElBasicAuthenticationEl {
    #[doc = "Set the field `password_secret_version`.\nThe password SecretManager secret version to authenticate as."]
    pub fn set_password_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_secret_version = Some(v.into());
        self
    }
}
impl ToListMappable for DeveloperConnectConnectionHttpConfigElBasicAuthenticationEl {
    type O = BlockAssignable<DeveloperConnectConnectionHttpConfigElBasicAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionHttpConfigElBasicAuthenticationEl {
    #[doc = "The username to authenticate as."]
    pub username: PrimField<String>,
}
impl BuildDeveloperConnectConnectionHttpConfigElBasicAuthenticationEl {
    pub fn build(self) -> DeveloperConnectConnectionHttpConfigElBasicAuthenticationEl {
        DeveloperConnectConnectionHttpConfigElBasicAuthenticationEl {
            password_secret_version: core::default::Default::default(),
            username: self.username,
        }
    }
}
pub struct DeveloperConnectConnectionHttpConfigElBasicAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionHttpConfigElBasicAuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionHttpConfigElBasicAuthenticationElRef {
        DeveloperConnectConnectionHttpConfigElBasicAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionHttpConfigElBasicAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password_secret_version` after provisioning.\nThe password SecretManager secret version to authenticate as."]
    pub fn password_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe username to authenticate as."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    token_secret_version: Option<PrimField<String>>,
}
impl DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationEl {
    #[doc = "Set the field `token_secret_version`.\nThe token SecretManager secret version to authenticate as."]
    pub fn set_token_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_secret_version = Some(v.into());
        self
    }
}
impl ToListMappable for DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationEl {
    type O = BlockAssignable<DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationEl {}
impl BuildDeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationEl {
    pub fn build(self) -> DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationEl {
        DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationEl {
            token_secret_version: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationElRef {
        DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `token_secret_version` after provisioning.\nThe token SecretManager secret version to authenticate as."]
    pub fn token_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_secret_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigEl {}
impl ToListMappable for DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigEl {
    type O = BlockAssignable<DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionHttpConfigElServiceDirectoryConfigEl {
    #[doc = "The Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub service: PrimField<String>,
}
impl BuildDeveloperConnectConnectionHttpConfigElServiceDirectoryConfigEl {
    pub fn build(self) -> DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigEl {
        DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigElRef {
        DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe Service Directory service name.\nFormat:\nprojects/{project}/locations/{location}/namespaces/{namespace}/services/{service}."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize, Default)]
struct DeveloperConnectConnectionHttpConfigElDynamic {
    basic_authentication:
        Option<DynamicBlock<DeveloperConnectConnectionHttpConfigElBasicAuthenticationEl>>,
    bearer_token_authentication:
        Option<DynamicBlock<DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationEl>>,
    service_directory_config:
        Option<DynamicBlock<DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigEl>>,
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionHttpConfigEl {
    host_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_authentication: Option<Vec<DeveloperConnectConnectionHttpConfigElBasicAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bearer_token_authentication:
        Option<Vec<DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config:
        Option<Vec<DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigEl>>,
    dynamic: DeveloperConnectConnectionHttpConfigElDynamic,
}
impl DeveloperConnectConnectionHttpConfigEl {
    #[doc = "Set the field `ssl_ca_certificate`.\nThe SSL certificate to use for requests to the HTTP service provider."]
    pub fn set_ssl_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `basic_authentication`.\n"]
    pub fn set_basic_authentication(
        mut self,
        v: impl Into<BlockAssignable<DeveloperConnectConnectionHttpConfigElBasicAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.basic_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_authentication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `bearer_token_authentication`.\n"]
    pub fn set_bearer_token_authentication(
        mut self,
        v: impl Into<BlockAssignable<DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bearer_token_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bearer_token_authentication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<BlockAssignable<DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigEl>>,
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
impl ToListMappable for DeveloperConnectConnectionHttpConfigEl {
    type O = BlockAssignable<DeveloperConnectConnectionHttpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionHttpConfigEl {
    #[doc = "The service provider's https endpoint."]
    pub host_uri: PrimField<String>,
}
impl BuildDeveloperConnectConnectionHttpConfigEl {
    pub fn build(self) -> DeveloperConnectConnectionHttpConfigEl {
        DeveloperConnectConnectionHttpConfigEl {
            host_uri: self.host_uri,
            ssl_ca_certificate: core::default::Default::default(),
            basic_authentication: core::default::Default::default(),
            bearer_token_authentication: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DeveloperConnectConnectionHttpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionHttpConfigElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectConnectionHttpConfigElRef {
        DeveloperConnectConnectionHttpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionHttpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_uri` after provisioning.\nThe service provider's https endpoint."]
    pub fn host_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `ssl_ca_certificate` after provisioning.\nThe SSL certificate to use for requests to the HTTP service provider."]
    pub fn ssl_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `basic_authentication` after provisioning.\n"]
    pub fn basic_authentication(
        &self,
    ) -> ListRef<DeveloperConnectConnectionHttpConfigElBasicAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basic_authentication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bearer_token_authentication` after provisioning.\n"]
    pub fn bearer_token_authentication(
        &self,
    ) -> ListRef<DeveloperConnectConnectionHttpConfigElBearerTokenAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bearer_token_authentication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<DeveloperConnectConnectionHttpConfigElServiceDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DeveloperConnectConnectionTimeoutsEl {
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
impl ToListMappable for DeveloperConnectConnectionTimeoutsEl {
    type O = BlockAssignable<DeveloperConnectConnectionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectConnectionTimeoutsEl {}
impl BuildDeveloperConnectConnectionTimeoutsEl {
    pub fn build(self) -> DeveloperConnectConnectionTimeoutsEl {
        DeveloperConnectConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectConnectionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectConnectionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectConnectionTimeoutsElRef {
        DeveloperConnectConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectConnectionTimeoutsElRef {
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
struct DeveloperConnectConnectionDynamic {
    bitbucket_cloud_config: Option<DynamicBlock<DeveloperConnectConnectionBitbucketCloudConfigEl>>,
    bitbucket_data_center_config:
        Option<DynamicBlock<DeveloperConnectConnectionBitbucketDataCenterConfigEl>>,
    crypto_key_config: Option<DynamicBlock<DeveloperConnectConnectionCryptoKeyConfigEl>>,
    github_config: Option<DynamicBlock<DeveloperConnectConnectionGithubConfigEl>>,
    github_enterprise_config:
        Option<DynamicBlock<DeveloperConnectConnectionGithubEnterpriseConfigEl>>,
    gitlab_config: Option<DynamicBlock<DeveloperConnectConnectionGitlabConfigEl>>,
    gitlab_enterprise_config:
        Option<DynamicBlock<DeveloperConnectConnectionGitlabEnterpriseConfigEl>>,
    http_config: Option<DynamicBlock<DeveloperConnectConnectionHttpConfigEl>>,
}
