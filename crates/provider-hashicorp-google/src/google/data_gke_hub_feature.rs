use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataGkeHubFeatureData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataGkeHubFeature_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGkeHubFeatureData>,
}
#[derive(Clone)]
pub struct DataGkeHubFeature(Rc<DataGkeHubFeature_>);
impl DataGkeHubFeature {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. When the Feature resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. When the Feature resource was deleted."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_default_member_config` after provisioning.\nOptional. Fleet Default Membership Configuration."]
    pub fn fleet_default_member_config(
        &self,
    ) -> ListRef<DataGkeHubFeatureFleetDefaultMemberConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet_default_member_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nGCP labels for this Feature.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full, unique name of this Feature resource"]
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
    #[doc = "Get a reference to the value of field `resource_state` after provisioning.\nState of the Feature resource itself."]
    pub fn resource_state(&self) -> ListRef<DataGkeHubFeatureResourceStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\nOptional. Hub-wide Feature configuration. If this Feature does not support any Hub-wide configuration, this field may be unused."]
    pub fn spec(&self) -> ListRef<DataGkeHubFeatureSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The Hub-wide Feature state"]
    pub fn state(&self) -> ListRef<DataGkeHubFeatureStateElRef> {
        ListRef::new(
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. When the Feature resource was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
impl Referable for DataGkeHubFeature {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataGkeHubFeature {}
impl ToListMappable for DataGkeHubFeature {
    type O = ListRef<DataGkeHubFeatureRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataGkeHubFeature_ {
    fn extract_datasource_type(&self) -> String {
        "google_gke_hub_feature".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataGkeHubFeature {
    pub tf_id: String,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "The full, unique name of this Feature resource"]
    pub name: PrimField<String>,
}
impl BuildDataGkeHubFeature {
    pub fn build(self, stack: &mut Stack) -> DataGkeHubFeature {
        let out = DataGkeHubFeature(Rc::new(DataGkeHubFeature_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataGkeHubFeatureData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataGkeHubFeatureRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataGkeHubFeatureRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. When the Feature resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. When the Feature resource was deleted."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_default_member_config` after provisioning.\nOptional. Fleet Default Membership Configuration."]
    pub fn fleet_default_member_config(
        &self,
    ) -> ListRef<DataGkeHubFeatureFleetDefaultMemberConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet_default_member_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nGCP labels for this Feature.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full, unique name of this Feature resource"]
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
    #[doc = "Get a reference to the value of field `resource_state` after provisioning.\nState of the Feature resource itself."]
    pub fn resource_state(&self) -> ListRef<DataGkeHubFeatureResourceStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\nOptional. Hub-wide Feature configuration. If this Feature does not support any Hub-wide configuration, this field may be unused."]
    pub fn spec(&self) -> ListRef<DataGkeHubFeatureSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The Hub-wide Feature state"]
    pub fn state(&self) -> ListRef<DataGkeHubFeatureStateElRef> {
        ListRef::new(
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. When the Feature resource was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_proxy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_repo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_rev: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_wait_secs: Option<PrimField<String>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    #[doc = "Set the field `gcp_service_account_email`.\n"]
    pub fn set_gcp_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `https_proxy`.\n"]
    pub fn set_https_proxy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.https_proxy = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_dir`.\n"]
    pub fn set_policy_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_dir = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_type`.\n"]
    pub fn set_secret_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_type = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_branch`.\n"]
    pub fn set_sync_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_branch = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_repo`.\n"]
    pub fn set_sync_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_repo = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_rev`.\n"]
    pub fn set_sync_rev(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_rev = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_wait_secs`.\n"]
    pub fn set_sync_wait_secs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_wait_secs = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl
{
    type O = BlockAssignable<
        DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
    pub fn build(
        self,
    ) -> DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
        DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl {
            gcp_service_account_email: core::default::Default::default(),
            https_proxy: core::default::Default::default(),
            policy_dir: core::default::Default::default(),
            secret_type: core::default::Default::default(),
            sync_branch: core::default::Default::default(),
            sync_repo: core::default::Default::default(),
            sync_rev: core::default::Default::default(),
            sync_wait_secs: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
        DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_service_account_email` after provisioning.\n"]
    pub fn gcp_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `https_proxy` after provisioning.\n"]
    pub fn https_proxy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.https_proxy", self.base))
    }
    #[doc = "Get a reference to the value of field `policy_dir` after provisioning.\n"]
    pub fn policy_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_type` after provisioning.\n"]
    pub fn secret_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_branch` after provisioning.\n"]
    pub fn sync_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_branch", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_repo` after provisioning.\n"]
    pub fn sync_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_rev` after provisioning.\n"]
    pub fn sync_rev(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_rev", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_wait_secs` after provisioning.\n"]
    pub fn sync_wait_secs(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sync_wait_secs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_repo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_wait_secs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    #[doc = "Set the field `gcp_service_account_email`.\n"]
    pub fn set_gcp_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_dir`.\n"]
    pub fn set_policy_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_dir = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_type`.\n"]
    pub fn set_secret_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_type = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_repo`.\n"]
    pub fn set_sync_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_repo = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_wait_secs`.\n"]
    pub fn set_sync_wait_secs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_wait_secs = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl
{
    type O = BlockAssignable<
        DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
    pub fn build(
        self,
    ) -> DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
        DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl {
            gcp_service_account_email: core::default::Default::default(),
            policy_dir: core::default::Default::default(),
            secret_type: core::default::Default::default(),
            sync_repo: core::default::Default::default(),
            sync_wait_secs: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
        DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_service_account_email` after provisioning.\n"]
    pub fn gcp_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_dir` after provisioning.\n"]
    pub fn policy_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_type` after provisioning.\n"]
    pub fn secret_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_repo` after provisioning.\n"]
    pub fn sync_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }
    #[doc = "Get a reference to the value of field `sync_wait_secs` after provisioning.\n"]
    pub fn sync_wait_secs(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sync_wait_secs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<
        ListField<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics_gcp_service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci: Option<
        ListField<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    prevent_drift: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_format: Option<PrimField<String>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `git`.\n"]
    pub fn set_git(
        mut self,
        v: impl Into<
            ListField<
                DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitEl,
            >,
        >,
    ) -> Self {
        self.git = Some(v.into());
        self
    }
    #[doc = "Set the field `metrics_gcp_service_account_email`.\n"]
    pub fn set_metrics_gcp_service_account_email(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.metrics_gcp_service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `oci`.\n"]
    pub fn set_oci(
        mut self,
        v: impl Into<
            ListField<
                DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciEl,
            >,
        >,
    ) -> Self {
        self.oci = Some(v.into());
        self
    }
    #[doc = "Set the field `prevent_drift`.\n"]
    pub fn set_prevent_drift(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.prevent_drift = Some(v.into());
        self
    }
    #[doc = "Set the field `source_format`.\n"]
    pub fn set_source_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_format = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    type O =
        BlockAssignable<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
    pub fn build(
        self,
    ) -> DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
        DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl {
            enabled: core::default::Default::default(),
            git: core::default::Default::default(),
            metrics_gcp_service_account_email: core::default::Default::default(),
            oci: core::default::Default::default(),
            prevent_drift: core::default::Default::default(),
            source_format: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
        DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(
        &self,
    ) -> ListRef<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElGitElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }
    #[doc = "Get a reference to the value of field `metrics_gcp_service_account_email` after provisioning.\n"]
    pub fn metrics_gcp_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metrics_gcp_service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oci` after provisioning.\n"]
    pub fn oci(
        &self,
    ) -> ListRef<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElOciElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.oci", self.base))
    }
    #[doc = "Get a reference to the value of field `prevent_drift` after provisioning.\n"]
    pub fn prevent_drift(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prevent_drift", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_format` after provisioning.\n"]
    pub fn source_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_format", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    config_sync: Option<
        ListField<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    #[doc = "Set the field `config_sync`.\n"]
    pub fn set_config_sync(
        mut self,
        v: impl Into<
            ListField<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncEl>,
        >,
    ) -> Self {
        self.config_sync = Some(v.into());
        self
    }
    #[doc = "Set the field `management`.\n"]
    pub fn set_management(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.management = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    type O = BlockAssignable<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
    pub fn build(self) -> DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
        DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl {
            config_sync: core::default::Default::default(),
            management: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
        DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config_sync` after provisioning.\n"]
    pub fn config_sync(
        &self,
    ) -> ListRef<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElConfigSyncElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config_sync", self.base))
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.management", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<PrimField<String>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    #[doc = "Set the field `management`.\n"]
    pub fn set_management(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.management = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    type O = BlockAssignable<DataGkeHubFeatureFleetDefaultMemberConfigElMeshEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElMeshEl {}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElMeshEl {
    pub fn build(self) -> DataGkeHubFeatureFleetDefaultMemberConfigElMeshEl {
        DataGkeHubFeatureFleetDefaultMemberConfigElMeshEl {
            management: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
        DataGkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElMeshElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.management", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<String>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl { # [doc = "Set the field `cpu`.\n"] pub fn set_cpu (mut self , v : impl Into < PrimField < String > >) -> Self { self . cpu = Some (v . into ()) ; self } # [doc = "Set the field `memory`.\n"] pub fn set_memory (mut self , v : impl Into < PrimField < String > >) -> Self { self . memory = Some (v . into ()) ; self } }
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl { type O = BlockAssignable < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl
{}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl { pub fn build (self) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl { cpu : core :: default :: Default :: default () , memory : core :: default :: Default :: default () , } } }
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef { fn new (shared : StackShared , base : String) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef { shared : shared , base : base . to_string () , } } }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `cpu` after provisioning.\n"] pub fn cpu (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.cpu" , self . base)) } # [doc = "Get a reference to the value of field `memory` after provisioning.\n"] pub fn memory (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.memory" , self . base)) } }
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<String>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl { # [doc = "Set the field `cpu`.\n"] pub fn set_cpu (mut self , v : impl Into < PrimField < String > >) -> Self { self . cpu = Some (v . into ()) ; self } # [doc = "Set the field `memory`.\n"] pub fn set_memory (mut self , v : impl Into < PrimField < String > >) -> Self { self . memory = Some (v . into ()) ; self } }
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl { type O = BlockAssignable < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl
{}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl { pub fn build (self) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl { cpu : core :: default :: Default :: default () , memory : core :: default :: Default :: default () , } } }
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef { fn new (shared : StackShared , base : String) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef { shared : shared , base : base . to_string () , } } }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `cpu` after provisioning.\n"] pub fn cpu (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.cpu" , self . base)) } # [doc = "Get a reference to the value of field `memory` after provisioning.\n"] pub fn memory (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.memory" , self . base)) } }
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { # [serde (skip_serializing_if = "Option::is_none")] limits : Option < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl > > , # [serde (skip_serializing_if = "Option::is_none")] requests : Option < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl > > , }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { # [doc = "Set the field `limits`.\n"] pub fn set_limits (mut self , v : impl Into < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsEl > >) -> Self { self . limits = Some (v . into ()) ; self } # [doc = "Set the field `requests`.\n"] pub fn set_requests (mut self , v : impl Into < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsEl > >) -> Self { self . requests = Some (v . into ()) ; self } }
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { type O = BlockAssignable < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl
{}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { pub fn build (self) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl { limits : core :: default :: Default :: default () , requests : core :: default :: Default :: default () , } } }
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef { fn new (shared : StackShared , base : String) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef { shared : shared , base : base . to_string () , } } }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `limits` after provisioning.\n"] pub fn limits (& self) -> ListRef < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElLimitsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.limits" , self . base)) } # [doc = "Get a reference to the value of field `requests` after provisioning.\n"] pub fn requests (& self) -> ListRef < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRequestsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.requests" , self . base)) } }
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl { # [doc = "Set the field `effect`.\n"] pub fn set_effect (mut self , v : impl Into < PrimField < String > >) -> Self { self . effect = Some (v . into ()) ; self } # [doc = "Set the field `key`.\n"] pub fn set_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . key = Some (v . into ()) ; self } # [doc = "Set the field `operator`.\n"] pub fn set_operator (mut self , v : impl Into < PrimField < String > >) -> Self { self . operator = Some (v . into ()) ; self } # [doc = "Set the field `value`.\n"] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl { type O = BlockAssignable < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl
{}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl { pub fn build (self) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl { effect : core :: default :: Default :: default () , key : core :: default :: Default :: default () , operator : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef { fn new (shared : StackShared , base : String) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef { shared : shared , base : base . to_string () , } } }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `effect` after provisioning.\n"] pub fn effect (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.effect" , self . base)) } # [doc = "Get a reference to the value of field `key` after provisioning.\n"] pub fn key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } # [doc = "Get a reference to the value of field `operator` after provisioning.\n"] pub fn operator (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.operator" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { # [serde (skip_serializing_if = "Option::is_none")] component : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] container_resources : Option < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl > > , # [serde (skip_serializing_if = "Option::is_none")] pod_affinity : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] pod_toleration : Option < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl > > , # [serde (skip_serializing_if = "Option::is_none")] replica_count : Option < PrimField < f64 > > , }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { # [doc = "Set the field `component`.\n"] pub fn set_component (mut self , v : impl Into < PrimField < String > >) -> Self { self . component = Some (v . into ()) ; self } # [doc = "Set the field `container_resources`.\n"] pub fn set_container_resources (mut self , v : impl Into < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesEl > >) -> Self { self . container_resources = Some (v . into ()) ; self } # [doc = "Set the field `pod_affinity`.\n"] pub fn set_pod_affinity (mut self , v : impl Into < PrimField < String > >) -> Self { self . pod_affinity = Some (v . into ()) ; self } # [doc = "Set the field `pod_toleration`.\n"] pub fn set_pod_toleration (mut self , v : impl Into < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationEl > >) -> Self { self . pod_toleration = Some (v . into ()) ; self } # [doc = "Set the field `replica_count`.\n"] pub fn set_replica_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . replica_count = Some (v . into ()) ; self } }
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { type O = BlockAssignable < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl
{}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { pub fn build (self) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl { component : core :: default :: Default :: default () , container_resources : core :: default :: Default :: default () , pod_affinity : core :: default :: Default :: default () , pod_toleration : core :: default :: Default :: default () , replica_count : core :: default :: Default :: default () , } } }
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef { fn new (shared : StackShared , base : String) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef { shared : shared , base : base . to_string () , } } }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `component` after provisioning.\n"] pub fn component (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.component" , self . base)) } # [doc = "Get a reference to the value of field `container_resources` after provisioning.\n"] pub fn container_resources (& self) -> ListRef < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElContainerResourcesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.container_resources" , self . base)) } # [doc = "Get a reference to the value of field `pod_affinity` after provisioning.\n"] pub fn pod_affinity (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pod_affinity" , self . base)) } # [doc = "Get a reference to the value of field `pod_toleration` after provisioning.\n"] pub fn pod_toleration (& self) -> ListRef < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElPodTolerationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.pod_toleration" , self . base)) } # [doc = "Get a reference to the value of field `replica_count` after provisioning.\n"] pub fn replica_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replica_count" , self . base)) } }
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    backends: Option<ListField<PrimField<String>>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl { # [doc = "Set the field `backends`.\n"] pub fn set_backends (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . backends = Some (v . into ()) ; self } }
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl { type O = BlockAssignable < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl
{}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl { pub fn build (self) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl { backends : core :: default :: Default :: default () , } } }
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef { fn new (shared : StackShared , base : String) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef { shared : shared , base : base . to_string () , } } }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `backends` after provisioning.\n"] pub fn backends (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.backends" , self . base)) } }
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    bundle: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exempted_namespaces: Option<ListField<PrimField<String>>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl { # [doc = "Set the field `bundle`.\n"] pub fn set_bundle (mut self , v : impl Into < PrimField < String > >) -> Self { self . bundle = Some (v . into ()) ; self } # [doc = "Set the field `exempted_namespaces`.\n"] pub fn set_exempted_namespaces (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . exempted_namespaces = Some (v . into ()) ; self } }
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl { type O = BlockAssignable < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl
{}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl { pub fn build (self) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl { bundle : core :: default :: Default :: default () , exempted_namespaces : core :: default :: Default :: default () , } } }
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef { fn new (shared : StackShared , base : String) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef { shared : shared , base : base . to_string () , } } }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bundle` after provisioning.\n"] pub fn bundle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bundle" , self . base)) } # [doc = "Get a reference to the value of field `exempted_namespaces` after provisioning.\n"] pub fn exempted_namespaces (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.exempted_namespaces" , self . base)) } }
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    installation: Option<PrimField<String>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl { # [doc = "Set the field `installation`.\n"] pub fn set_installation (mut self , v : impl Into < PrimField < String > >) -> Self { self . installation = Some (v . into ()) ; self } }
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl { type O = BlockAssignable < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl
{}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl { pub fn build (self) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl { installation : core :: default :: Default :: default () , } } }
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef { fn new (shared : StackShared , base : String) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef { shared : shared , base : base . to_string () , } } }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `installation` after provisioning.\n"] pub fn installation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.installation" , self . base)) } }
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { # [serde (skip_serializing_if = "Option::is_none")] bundles : Option < SetField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl > > , # [serde (skip_serializing_if = "Option::is_none")] template_library : Option < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl > > , }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { # [doc = "Set the field `bundles`.\n"] pub fn set_bundles (mut self , v : impl Into < SetField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesEl > >) -> Self { self . bundles = Some (v . into ()) ; self } # [doc = "Set the field `template_library`.\n"] pub fn set_template_library (mut self , v : impl Into < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryEl > >) -> Self { self . template_library = Some (v . into ()) ; self } }
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { type O = BlockAssignable < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl
{}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { pub fn build (self) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl { bundles : core :: default :: Default :: default () , template_library : core :: default :: Default :: default () , } } }
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef { fn new (shared : StackShared , base : String) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef { DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef { shared : shared , base : base . to_string () , } } }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bundles` after provisioning.\n"] pub fn bundles (& self) -> SetRef < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElBundlesElRef > { SetRef :: new (self . shared () . clone () , format ! ("{}.bundles" , self . base)) } # [doc = "Get a reference to the value of field `template_library` after provisioning.\n"] pub fn template_library (& self) -> ListRef < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElTemplateLibraryElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.template_library" , self . base)) } }
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl { # [serde (skip_serializing_if = "Option::is_none")] audit_interval_seconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] constraint_violation_limit : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] deployment_configs : Option < SetField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl > > , # [serde (skip_serializing_if = "Option::is_none")] exemptable_namespaces : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] install_spec : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] log_denies_enabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] monitoring : Option < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl > > , # [serde (skip_serializing_if = "Option::is_none")] mutation_enabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] policy_content : Option < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl > > , # [serde (skip_serializing_if = "Option::is_none")] referential_rules_enabled : Option < PrimField < bool > > , }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
    #[doc = "Set the field `audit_interval_seconds`.\n"]
    pub fn set_audit_interval_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.audit_interval_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `constraint_violation_limit`.\n"]
    pub fn set_constraint_violation_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.constraint_violation_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `deployment_configs`.\n"]
    pub fn set_deployment_configs(
        mut self,
        v : impl Into < SetField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsEl > >,
    ) -> Self {
        self.deployment_configs = Some(v.into());
        self
    }
    #[doc = "Set the field `exemptable_namespaces`.\n"]
    pub fn set_exemptable_namespaces(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exemptable_namespaces = Some(v.into());
        self
    }
    #[doc = "Set the field `install_spec`.\n"]
    pub fn set_install_spec(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.install_spec = Some(v.into());
        self
    }
    #[doc = "Set the field `log_denies_enabled`.\n"]
    pub fn set_log_denies_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.log_denies_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(
        mut self,
        v : impl Into < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringEl > >,
    ) -> Self {
        self.monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `mutation_enabled`.\n"]
    pub fn set_mutation_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mutation_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_content`.\n"]
    pub fn set_policy_content(
        mut self,
        v : impl Into < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentEl > >,
    ) -> Self {
        self.policy_content = Some(v.into());
        self
    }
    #[doc = "Set the field `referential_rules_enabled`.\n"]
    pub fn set_referential_rules_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.referential_rules_enabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl
{
    type O = BlockAssignable<
        DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl
{}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
    pub fn build(
        self,
    ) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl
    {
        DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl {
            audit_interval_seconds: core::default::Default::default(),
            constraint_violation_limit: core::default::Default::default(),
            deployment_configs: core::default::Default::default(),
            exemptable_namespaces: core::default::Default::default(),
            install_spec: core::default::Default::default(),
            log_denies_enabled: core::default::Default::default(),
            monitoring: core::default::Default::default(),
            mutation_enabled: core::default::Default::default(),
            policy_content: core::default::Default::default(),
            referential_rules_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef
    {
        DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audit_interval_seconds` after provisioning.\n"]
    pub fn audit_interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audit_interval_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `constraint_violation_limit` after provisioning.\n"]
    pub fn constraint_violation_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.constraint_violation_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_configs` after provisioning.\n"]    pub fn deployment_configs (& self) -> SetRef < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElDeploymentConfigsElRef >{
        SetRef::new(
            self.shared().clone(),
            format!("{}.deployment_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exemptable_namespaces` after provisioning.\n"]
    pub fn exemptable_namespaces(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exemptable_namespaces", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `install_spec` after provisioning.\n"]
    pub fn install_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.install_spec", self.base))
    }
    #[doc = "Get a reference to the value of field `log_denies_enabled` after provisioning.\n"]
    pub fn log_denies_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_denies_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]    pub fn monitoring (& self) -> ListRef < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElMonitoringElRef >{
        ListRef::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }
    #[doc = "Get a reference to the value of field `mutation_enabled` after provisioning.\n"]
    pub fn mutation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mutation_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_content` after provisioning.\n"]    pub fn policy_content (& self) -> ListRef < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElPolicyContentElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_content", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `referential_rules_enabled` after provisioning.\n"]
    pub fn referential_rules_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.referential_rules_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl { # [serde (skip_serializing_if = "Option::is_none")] policy_controller_hub_config : Option < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] version : Option < PrimField < String > > , }
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
    #[doc = "Set the field `policy_controller_hub_config`.\n"]
    pub fn set_policy_controller_hub_config(
        mut self,
        v : impl Into < ListField < DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigEl > >,
    ) -> Self {
        self.policy_controller_hub_config = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
    type O = BlockAssignable<DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
    pub fn build(self) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
        DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl {
            policy_controller_hub_config: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
        DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy_controller_hub_config` after provisioning.\n"]
    pub fn policy_controller_hub_config(
        &self,
    ) -> ListRef<
        DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElPolicyControllerHubConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_controller_hub_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureFleetDefaultMemberConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    configmanagement:
        Option<ListField<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mesh: Option<ListField<DataGkeHubFeatureFleetDefaultMemberConfigElMeshEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policycontroller:
        Option<ListField<DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl>>,
}
impl DataGkeHubFeatureFleetDefaultMemberConfigEl {
    #[doc = "Set the field `configmanagement`.\n"]
    pub fn set_configmanagement(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementEl>>,
    ) -> Self {
        self.configmanagement = Some(v.into());
        self
    }
    #[doc = "Set the field `mesh`.\n"]
    pub fn set_mesh(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureFleetDefaultMemberConfigElMeshEl>>,
    ) -> Self {
        self.mesh = Some(v.into());
        self
    }
    #[doc = "Set the field `policycontroller`.\n"]
    pub fn set_policycontroller(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerEl>>,
    ) -> Self {
        self.policycontroller = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureFleetDefaultMemberConfigEl {
    type O = BlockAssignable<DataGkeHubFeatureFleetDefaultMemberConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureFleetDefaultMemberConfigEl {}
impl BuildDataGkeHubFeatureFleetDefaultMemberConfigEl {
    pub fn build(self) -> DataGkeHubFeatureFleetDefaultMemberConfigEl {
        DataGkeHubFeatureFleetDefaultMemberConfigEl {
            configmanagement: core::default::Default::default(),
            mesh: core::default::Default::default(),
            policycontroller: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureFleetDefaultMemberConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureFleetDefaultMemberConfigElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubFeatureFleetDefaultMemberConfigElRef {
        DataGkeHubFeatureFleetDefaultMemberConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureFleetDefaultMemberConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `configmanagement` after provisioning.\n"]
    pub fn configmanagement(
        &self,
    ) -> ListRef<DataGkeHubFeatureFleetDefaultMemberConfigElConfigmanagementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configmanagement", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mesh` after provisioning.\n"]
    pub fn mesh(&self) -> ListRef<DataGkeHubFeatureFleetDefaultMemberConfigElMeshElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mesh", self.base))
    }
    #[doc = "Get a reference to the value of field `policycontroller` after provisioning.\n"]
    pub fn policycontroller(
        &self,
    ) -> ListRef<DataGkeHubFeatureFleetDefaultMemberConfigElPolicycontrollerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policycontroller", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureResourceStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    has_resources: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataGkeHubFeatureResourceStateEl {
    #[doc = "Set the field `has_resources`.\n"]
    pub fn set_has_resources(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.has_resources = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureResourceStateEl {
    type O = BlockAssignable<DataGkeHubFeatureResourceStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureResourceStateEl {}
impl BuildDataGkeHubFeatureResourceStateEl {
    pub fn build(self) -> DataGkeHubFeatureResourceStateEl {
        DataGkeHubFeatureResourceStateEl {
            has_resources: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureResourceStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureResourceStateElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubFeatureResourceStateElRef {
        DataGkeHubFeatureResourceStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureResourceStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `has_resources` after provisioning.\n"]
    pub fn has_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.has_resources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    soaking: Option<PrimField<String>>,
}
impl DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {
    #[doc = "Set the field `soaking`.\n"]
    pub fn set_soaking(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.soaking = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl
{
    type O = BlockAssignable<
        DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {}
impl BuildDataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {
    pub fn build(
        self,
    ) -> DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {
        DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl {
            soaking: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef {
        DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `soaking` after provisioning.\n"]
    pub fn soaking(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.soaking", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
    type O = BlockAssignable<DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {}
impl BuildDataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
    pub fn build(self) -> DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
        DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl {
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef {
        DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    post_conditions: Option<
        ListField<DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade:
        Option<ListField<DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl>>,
}
impl DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
    #[doc = "Set the field `post_conditions`.\n"]
    pub fn set_post_conditions(
        mut self,
        v: impl Into<
            ListField<DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsEl>,
        >,
    ) -> Self {
        self.post_conditions = Some(v.into());
        self
    }
    #[doc = "Set the field `upgrade`.\n"]
    pub fn set_upgrade(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeEl>>,
    ) -> Self {
        self.upgrade = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
    type O = BlockAssignable<DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {}
impl BuildDataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
    pub fn build(self) -> DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
        DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl {
            post_conditions: core::default::Default::default(),
            upgrade: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef {
        DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `post_conditions` after provisioning.\n"]
    pub fn post_conditions(
        &self,
    ) -> ListRef<DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElPostConditionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.post_conditions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade` after provisioning.\n"]
    pub fn upgrade(
        &self,
    ) -> ListRef<DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElUpgradeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upgrade", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    soaking: Option<PrimField<String>>,
}
impl DataGkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
    #[doc = "Set the field `soaking`.\n"]
    pub fn set_soaking(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.soaking = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
    type O = BlockAssignable<DataGkeHubFeatureSpecElClusterupgradeElPostConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElClusterupgradeElPostConditionsEl {}
impl BuildDataGkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
    pub fn build(self) -> DataGkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
        DataGkeHubFeatureSpecElClusterupgradeElPostConditionsEl {
            soaking: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElClusterupgradeElPostConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElClusterupgradeElPostConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureSpecElClusterupgradeElPostConditionsElRef {
        DataGkeHubFeatureSpecElClusterupgradeElPostConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElClusterupgradeElPostConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `soaking` after provisioning.\n"]
    pub fn soaking(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.soaking", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElClusterupgradeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_upgrade_overrides:
        Option<ListField<DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_conditions: Option<ListField<DataGkeHubFeatureSpecElClusterupgradeElPostConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upstream_fleets: Option<ListField<PrimField<String>>>,
}
impl DataGkeHubFeatureSpecElClusterupgradeEl {
    #[doc = "Set the field `gke_upgrade_overrides`.\n"]
    pub fn set_gke_upgrade_overrides(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesEl>>,
    ) -> Self {
        self.gke_upgrade_overrides = Some(v.into());
        self
    }
    #[doc = "Set the field `post_conditions`.\n"]
    pub fn set_post_conditions(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureSpecElClusterupgradeElPostConditionsEl>>,
    ) -> Self {
        self.post_conditions = Some(v.into());
        self
    }
    #[doc = "Set the field `upstream_fleets`.\n"]
    pub fn set_upstream_fleets(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.upstream_fleets = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureSpecElClusterupgradeEl {
    type O = BlockAssignable<DataGkeHubFeatureSpecElClusterupgradeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElClusterupgradeEl {}
impl BuildDataGkeHubFeatureSpecElClusterupgradeEl {
    pub fn build(self) -> DataGkeHubFeatureSpecElClusterupgradeEl {
        DataGkeHubFeatureSpecElClusterupgradeEl {
            gke_upgrade_overrides: core::default::Default::default(),
            post_conditions: core::default::Default::default(),
            upstream_fleets: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElClusterupgradeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElClusterupgradeElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubFeatureSpecElClusterupgradeElRef {
        DataGkeHubFeatureSpecElClusterupgradeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElClusterupgradeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gke_upgrade_overrides` after provisioning.\n"]
    pub fn gke_upgrade_overrides(
        &self,
    ) -> ListRef<DataGkeHubFeatureSpecElClusterupgradeElGkeUpgradeOverridesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gke_upgrade_overrides", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `post_conditions` after provisioning.\n"]
    pub fn post_conditions(
        &self,
    ) -> ListRef<DataGkeHubFeatureSpecElClusterupgradeElPostConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.post_conditions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upstream_fleets` after provisioning.\n"]
    pub fn upstream_fleets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upstream_fleets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    type O =
        BlockAssignable<DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {}
impl BuildDataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
    pub fn build(
        self,
    ) -> DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
        DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
        DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl
{
    type O = BlockAssignable<
        DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
}
impl BuildDataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
    pub fn build(
        self,
    ) -> DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
        DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
        DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_config: Option<
        ListField<DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_scope_logs_config: Option<
        ListField<DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl>,
    >,
}
impl DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    #[doc = "Set the field `default_config`.\n"]
    pub fn set_default_config(
        mut self,
        v: impl Into<
            ListField<DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigEl>,
        >,
    ) -> Self {
        self.default_config = Some(v.into());
        self
    }
    #[doc = "Set the field `fleet_scope_logs_config`.\n"]
    pub fn set_fleet_scope_logs_config(
        mut self,
        v: impl Into<
            ListField<
                DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigEl,
            >,
        >,
    ) -> Self {
        self.fleet_scope_logs_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    type O = BlockAssignable<DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {}
impl BuildDataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
    pub fn build(self) -> DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
        DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl {
            default_config: core::default::Default::default(),
            fleet_scope_logs_config: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
        DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_config` after provisioning.\n"]
    pub fn default_config(
        &self,
    ) -> ListRef<DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElDefaultConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_scope_logs_config` after provisioning.\n"]
    pub fn fleet_scope_logs_config(
        &self,
    ) -> ListRef<DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElFleetScopeLogsConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet_scope_logs_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElFleetobservabilityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<ListField<DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl>>,
}
impl DataGkeHubFeatureSpecElFleetobservabilityEl {
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigEl>>,
    ) -> Self {
        self.logging_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureSpecElFleetobservabilityEl {
    type O = BlockAssignable<DataGkeHubFeatureSpecElFleetobservabilityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElFleetobservabilityEl {}
impl BuildDataGkeHubFeatureSpecElFleetobservabilityEl {
    pub fn build(self) -> DataGkeHubFeatureSpecElFleetobservabilityEl {
        DataGkeHubFeatureSpecElFleetobservabilityEl {
            logging_config: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElFleetobservabilityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElFleetobservabilityElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubFeatureSpecElFleetobservabilityElRef {
        DataGkeHubFeatureSpecElFleetobservabilityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElFleetobservabilityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(
        &self,
    ) -> ListRef<DataGkeHubFeatureSpecElFleetobservabilityElLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElMulticlusteringressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    config_membership: Option<PrimField<String>>,
}
impl DataGkeHubFeatureSpecElMulticlusteringressEl {
    #[doc = "Set the field `config_membership`.\n"]
    pub fn set_config_membership(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.config_membership = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureSpecElMulticlusteringressEl {
    type O = BlockAssignable<DataGkeHubFeatureSpecElMulticlusteringressEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElMulticlusteringressEl {}
impl BuildDataGkeHubFeatureSpecElMulticlusteringressEl {
    pub fn build(self) -> DataGkeHubFeatureSpecElMulticlusteringressEl {
        DataGkeHubFeatureSpecElMulticlusteringressEl {
            config_membership: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElMulticlusteringressElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElMulticlusteringressElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubFeatureSpecElMulticlusteringressElRef {
        DataGkeHubFeatureSpecElMulticlusteringressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElMulticlusteringressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config_membership` after provisioning.\n"]
    pub fn config_membership(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_membership", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElRbacrolebindingactuationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_custom_roles: Option<ListField<PrimField<String>>>,
}
impl DataGkeHubFeatureSpecElRbacrolebindingactuationEl {
    #[doc = "Set the field `allowed_custom_roles`.\n"]
    pub fn set_allowed_custom_roles(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_custom_roles = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureSpecElRbacrolebindingactuationEl {
    type O = BlockAssignable<DataGkeHubFeatureSpecElRbacrolebindingactuationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElRbacrolebindingactuationEl {}
impl BuildDataGkeHubFeatureSpecElRbacrolebindingactuationEl {
    pub fn build(self) -> DataGkeHubFeatureSpecElRbacrolebindingactuationEl {
        DataGkeHubFeatureSpecElRbacrolebindingactuationEl {
            allowed_custom_roles: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElRbacrolebindingactuationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElRbacrolebindingactuationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGkeHubFeatureSpecElRbacrolebindingactuationElRef {
        DataGkeHubFeatureSpecElRbacrolebindingactuationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElRbacrolebindingactuationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_custom_roles` after provisioning.\n"]
    pub fn allowed_custom_roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_custom_roles", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecElWorkloadidentityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope_tenancy_pool: Option<PrimField<String>>,
}
impl DataGkeHubFeatureSpecElWorkloadidentityEl {
    #[doc = "Set the field `scope_tenancy_pool`.\n"]
    pub fn set_scope_tenancy_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope_tenancy_pool = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureSpecElWorkloadidentityEl {
    type O = BlockAssignable<DataGkeHubFeatureSpecElWorkloadidentityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecElWorkloadidentityEl {}
impl BuildDataGkeHubFeatureSpecElWorkloadidentityEl {
    pub fn build(self) -> DataGkeHubFeatureSpecElWorkloadidentityEl {
        DataGkeHubFeatureSpecElWorkloadidentityEl {
            scope_tenancy_pool: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElWorkloadidentityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElWorkloadidentityElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubFeatureSpecElWorkloadidentityElRef {
        DataGkeHubFeatureSpecElWorkloadidentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElWorkloadidentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scope_tenancy_pool` after provisioning.\n"]
    pub fn scope_tenancy_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope_tenancy_pool", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    clusterupgrade: Option<ListField<DataGkeHubFeatureSpecElClusterupgradeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleetobservability: Option<ListField<DataGkeHubFeatureSpecElFleetobservabilityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiclusteringress: Option<ListField<DataGkeHubFeatureSpecElMulticlusteringressEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rbacrolebindingactuation: Option<ListField<DataGkeHubFeatureSpecElRbacrolebindingactuationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workloadidentity: Option<ListField<DataGkeHubFeatureSpecElWorkloadidentityEl>>,
}
impl DataGkeHubFeatureSpecEl {
    #[doc = "Set the field `clusterupgrade`.\n"]
    pub fn set_clusterupgrade(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureSpecElClusterupgradeEl>>,
    ) -> Self {
        self.clusterupgrade = Some(v.into());
        self
    }
    #[doc = "Set the field `fleetobservability`.\n"]
    pub fn set_fleetobservability(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureSpecElFleetobservabilityEl>>,
    ) -> Self {
        self.fleetobservability = Some(v.into());
        self
    }
    #[doc = "Set the field `multiclusteringress`.\n"]
    pub fn set_multiclusteringress(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureSpecElMulticlusteringressEl>>,
    ) -> Self {
        self.multiclusteringress = Some(v.into());
        self
    }
    #[doc = "Set the field `rbacrolebindingactuation`.\n"]
    pub fn set_rbacrolebindingactuation(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureSpecElRbacrolebindingactuationEl>>,
    ) -> Self {
        self.rbacrolebindingactuation = Some(v.into());
        self
    }
    #[doc = "Set the field `workloadidentity`.\n"]
    pub fn set_workloadidentity(
        mut self,
        v: impl Into<ListField<DataGkeHubFeatureSpecElWorkloadidentityEl>>,
    ) -> Self {
        self.workloadidentity = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureSpecEl {
    type O = BlockAssignable<DataGkeHubFeatureSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureSpecEl {}
impl BuildDataGkeHubFeatureSpecEl {
    pub fn build(self) -> DataGkeHubFeatureSpecEl {
        DataGkeHubFeatureSpecEl {
            clusterupgrade: core::default::Default::default(),
            fleetobservability: core::default::Default::default(),
            multiclusteringress: core::default::Default::default(),
            rbacrolebindingactuation: core::default::Default::default(),
            workloadidentity: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureSpecElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubFeatureSpecElRef {
        DataGkeHubFeatureSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `clusterupgrade` after provisioning.\n"]
    pub fn clusterupgrade(&self) -> ListRef<DataGkeHubFeatureSpecElClusterupgradeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.clusterupgrade", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fleetobservability` after provisioning.\n"]
    pub fn fleetobservability(&self) -> ListRef<DataGkeHubFeatureSpecElFleetobservabilityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleetobservability", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `multiclusteringress` after provisioning.\n"]
    pub fn multiclusteringress(&self) -> ListRef<DataGkeHubFeatureSpecElMulticlusteringressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiclusteringress", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rbacrolebindingactuation` after provisioning.\n"]
    pub fn rbacrolebindingactuation(
        &self,
    ) -> ListRef<DataGkeHubFeatureSpecElRbacrolebindingactuationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rbacrolebindingactuation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workloadidentity` after provisioning.\n"]
    pub fn workloadidentity(&self) -> ListRef<DataGkeHubFeatureSpecElWorkloadidentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workloadidentity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureStateElStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataGkeHubFeatureStateElStateEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureStateElStateEl {
    type O = BlockAssignable<DataGkeHubFeatureStateElStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureStateElStateEl {}
impl BuildDataGkeHubFeatureStateElStateEl {
    pub fn build(self) -> DataGkeHubFeatureStateElStateEl {
        DataGkeHubFeatureStateElStateEl {
            code: core::default::Default::default(),
            description: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureStateElStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureStateElStateElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubFeatureStateElStateElRef {
        DataGkeHubFeatureStateElStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureStateElStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubFeatureStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<ListField<DataGkeHubFeatureStateElStateEl>>,
}
impl DataGkeHubFeatureStateEl {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<ListField<DataGkeHubFeatureStateElStateEl>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubFeatureStateEl {
    type O = BlockAssignable<DataGkeHubFeatureStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubFeatureStateEl {}
impl BuildDataGkeHubFeatureStateEl {
    pub fn build(self) -> DataGkeHubFeatureStateEl {
        DataGkeHubFeatureStateEl {
            state: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubFeatureStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubFeatureStateElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubFeatureStateElRef {
        DataGkeHubFeatureStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubFeatureStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> ListRef<DataGkeHubFeatureStateElStateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
