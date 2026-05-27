use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ArtifactRegistryRepositoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cleanup_policy_dry_run: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    format: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    repository_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cleanup_policies: Option<Vec<ArtifactRegistryRepositoryCleanupPoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docker_config: Option<Vec<ArtifactRegistryRepositoryDockerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maven_config: Option<Vec<ArtifactRegistryRepositoryMavenConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_repository_config: Option<Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ArtifactRegistryRepositoryTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_repository_config: Option<Vec<ArtifactRegistryRepositoryVirtualRepositoryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vulnerability_scanning_config:
        Option<Vec<ArtifactRegistryRepositoryVulnerabilityScanningConfigEl>>,
    dynamic: ArtifactRegistryRepositoryDynamic,
}
struct ArtifactRegistryRepository_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ArtifactRegistryRepositoryData>,
}
#[derive(Clone)]
pub struct ArtifactRegistryRepository(Rc<ArtifactRegistryRepository_>);
impl ArtifactRegistryRepository {
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
    #[doc = "Set the field `cleanup_policy_dry_run`.\nIf true, the cleanup pipeline is prevented from deleting versions in this\nrepository."]
    pub fn set_cleanup_policy_dry_run(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cleanup_policy_dry_run = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe user-provided description of the repository."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\nThe Cloud KMS resource name of the customer managed encryption key that’s\nused to encrypt the contents of the Repository. Has the form:\n'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'.\nThis value may not be changed after the Repository has been created."]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels with user-defined metadata.\nThis field may contain up to 64 entries. Label keys and values may be no\nlonger than 63 characters. Label keys must begin with a lowercase letter\nand may only contain lowercase letters, numeric characters, underscores,\nand dashes.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe name of the repository's location. In addition to specific regions,\nspecial values for multi-region locations are 'asia', 'europe', and 'us'.\nSee [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),\nor use the\n[google_artifact_registry_locations](https://registry.terraform.io/providers/hashicorp/google/latest/docs/data-sources/artifact_registry_locations)\ndata source for possible values."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nThe mode configures the repository to serve artifacts from different sources. Default value: \"STANDARD_REPOSITORY\" Possible values: [\"STANDARD_REPOSITORY\", \"VIRTUAL_REPOSITORY\", \"REMOTE_REPOSITORY\"]"]
    pub fn set_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mode = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `cleanup_policies`.\n"]
    pub fn set_cleanup_policies(
        self,
        v: impl Into<BlockAssignable<ArtifactRegistryRepositoryCleanupPoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cleanup_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cleanup_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `docker_config`.\n"]
    pub fn set_docker_config(
        self,
        v: impl Into<BlockAssignable<ArtifactRegistryRepositoryDockerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().docker_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.docker_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `maven_config`.\n"]
    pub fn set_maven_config(
        self,
        v: impl Into<BlockAssignable<ArtifactRegistryRepositoryMavenConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maven_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maven_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remote_repository_config`.\n"]
    pub fn set_remote_repository_config(
        self,
        v: impl Into<BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_repository_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_repository_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ArtifactRegistryRepositoryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_repository_config`.\n"]
    pub fn set_virtual_repository_config(
        self,
        v: impl Into<BlockAssignable<ArtifactRegistryRepositoryVirtualRepositoryConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().virtual_repository_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.virtual_repository_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vulnerability_scanning_config`.\n"]
    pub fn set_vulnerability_scanning_config(
        self,
        v: impl Into<BlockAssignable<ArtifactRegistryRepositoryVulnerabilityScanningConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vulnerability_scanning_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .vulnerability_scanning_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `cleanup_policy_dry_run` after provisioning.\nIf true, the cleanup pipeline is prevented from deleting versions in this\nrepository."]
    pub fn cleanup_policy_dry_run(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cleanup_policy_dry_run", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the repository was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe user-provided description of the repository."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\nThe format of packages that are stored in the repository. Supported formats\ncan be found [here](https://cloud.google.com/artifact-registry/docs/supported-formats).\nYou can only create alpha formats if you are a member of the\n[alpha user group](https://cloud.google.com/artifact-registry/docs/supported-formats#alpha-access)."]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe Cloud KMS resource name of the customer managed encryption key that’s\nused to encrypt the contents of the Repository. Has the form:\n'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'.\nThis value may not be changed after the Repository has been created."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\nThis field may contain up to 64 entries. Label keys and values may be no\nlonger than 63 characters. Label keys must begin with a lowercase letter\nand may only contain lowercase letters, numeric characters, underscores,\nand dashes.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the repository's location. In addition to specific regions,\nspecial values for multi-region locations are 'asia', 'europe', and 'us'.\nSee [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),\nor use the\n[google_artifact_registry_locations](https://registry.terraform.io/providers/hashicorp/google/latest/docs/data-sources/artifact_registry_locations)\ndata source for possible values."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe mode configures the repository to serve artifacts from different sources. Default value: \"STANDARD_REPOSITORY\" Possible values: [\"STANDARD_REPOSITORY\", \"VIRTUAL_REPOSITORY\", \"REMOTE_REPOSITORY\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the repository, for example:\n\"repo1\""]
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
    #[doc = "Get a reference to the value of field `registry_uri` after provisioning.\nThe repository endpoint, for example: us-docker.pkg.dev/my-proj/my-repo."]
    pub fn registry_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe last part of the repository name, for example:\n\"repo1\""]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the repository was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `docker_config` after provisioning.\n"]
    pub fn docker_config(&self) -> ListRef<ArtifactRegistryRepositoryDockerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.docker_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maven_config` after provisioning.\n"]
    pub fn maven_config(&self) -> ListRef<ArtifactRegistryRepositoryMavenConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maven_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remote_repository_config` after provisioning.\n"]
    pub fn remote_repository_config(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryRemoteRepositoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remote_repository_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ArtifactRegistryRepositoryTimeoutsElRef {
        ArtifactRegistryRepositoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_repository_config` after provisioning.\n"]
    pub fn virtual_repository_config(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryVirtualRepositoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_repository_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vulnerability_scanning_config` after provisioning.\n"]
    pub fn vulnerability_scanning_config(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryVulnerabilityScanningConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vulnerability_scanning_config", self.extract_ref()),
        )
    }
}
impl Referable for ArtifactRegistryRepository {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ArtifactRegistryRepository {}
impl ToListMappable for ArtifactRegistryRepository {
    type O = ListRef<ArtifactRegistryRepositoryRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ArtifactRegistryRepository_ {
    fn extract_resource_type(&self) -> String {
        "google_artifact_registry_repository".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildArtifactRegistryRepository {
    pub tf_id: String,
    #[doc = "The format of packages that are stored in the repository. Supported formats\ncan be found [here](https://cloud.google.com/artifact-registry/docs/supported-formats).\nYou can only create alpha formats if you are a member of the\n[alpha user group](https://cloud.google.com/artifact-registry/docs/supported-formats#alpha-access)."]
    pub format: PrimField<String>,
    #[doc = "The last part of the repository name, for example:\n\"repo1\""]
    pub repository_id: PrimField<String>,
}
impl BuildArtifactRegistryRepository {
    pub fn build(self, stack: &mut Stack) -> ArtifactRegistryRepository {
        let out = ArtifactRegistryRepository(Rc::new(ArtifactRegistryRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ArtifactRegistryRepositoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cleanup_policy_dry_run: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                format: self.format,
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                mode: core::default::Default::default(),
                project: core::default::Default::default(),
                repository_id: self.repository_id,
                cleanup_policies: core::default::Default::default(),
                docker_config: core::default::Default::default(),
                maven_config: core::default::Default::default(),
                remote_repository_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                virtual_repository_config: core::default::Default::default(),
                vulnerability_scanning_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ArtifactRegistryRepositoryRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ArtifactRegistryRepositoryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cleanup_policy_dry_run` after provisioning.\nIf true, the cleanup pipeline is prevented from deleting versions in this\nrepository."]
    pub fn cleanup_policy_dry_run(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cleanup_policy_dry_run", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the repository was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe user-provided description of the repository."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\nThe format of packages that are stored in the repository. Supported formats\ncan be found [here](https://cloud.google.com/artifact-registry/docs/supported-formats).\nYou can only create alpha formats if you are a member of the\n[alpha user group](https://cloud.google.com/artifact-registry/docs/supported-formats#alpha-access)."]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe Cloud KMS resource name of the customer managed encryption key that’s\nused to encrypt the contents of the Repository. Has the form:\n'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'.\nThis value may not be changed after the Repository has been created."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\nThis field may contain up to 64 entries. Label keys and values may be no\nlonger than 63 characters. Label keys must begin with a lowercase letter\nand may only contain lowercase letters, numeric characters, underscores,\nand dashes.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the repository's location. In addition to specific regions,\nspecial values for multi-region locations are 'asia', 'europe', and 'us'.\nSee [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),\nor use the\n[google_artifact_registry_locations](https://registry.terraform.io/providers/hashicorp/google/latest/docs/data-sources/artifact_registry_locations)\ndata source for possible values."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe mode configures the repository to serve artifacts from different sources. Default value: \"STANDARD_REPOSITORY\" Possible values: [\"STANDARD_REPOSITORY\", \"VIRTUAL_REPOSITORY\", \"REMOTE_REPOSITORY\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the repository, for example:\n\"repo1\""]
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
    #[doc = "Get a reference to the value of field `registry_uri` after provisioning.\nThe repository endpoint, for example: us-docker.pkg.dev/my-proj/my-repo."]
    pub fn registry_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe last part of the repository name, for example:\n\"repo1\""]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the repository was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `docker_config` after provisioning.\n"]
    pub fn docker_config(&self) -> ListRef<ArtifactRegistryRepositoryDockerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.docker_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maven_config` after provisioning.\n"]
    pub fn maven_config(&self) -> ListRef<ArtifactRegistryRepositoryMavenConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maven_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remote_repository_config` after provisioning.\n"]
    pub fn remote_repository_config(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryRemoteRepositoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remote_repository_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ArtifactRegistryRepositoryTimeoutsElRef {
        ArtifactRegistryRepositoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_repository_config` after provisioning.\n"]
    pub fn virtual_repository_config(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryVirtualRepositoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_repository_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vulnerability_scanning_config` after provisioning.\n"]
    pub fn vulnerability_scanning_config(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryVulnerabilityScanningConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vulnerability_scanning_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryCleanupPoliciesElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    newer_than: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    older_than: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_name_prefixes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_prefixes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_name_prefixes: Option<ListField<PrimField<String>>>,
}
impl ArtifactRegistryRepositoryCleanupPoliciesElConditionEl {
    #[doc = "Set the field `newer_than`.\nMatch versions newer than a duration."]
    pub fn set_newer_than(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.newer_than = Some(v.into());
        self
    }
    #[doc = "Set the field `older_than`.\nMatch versions older than a duration."]
    pub fn set_older_than(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.older_than = Some(v.into());
        self
    }
    #[doc = "Set the field `package_name_prefixes`.\nMatch versions by package prefix. Applied on any prefix match."]
    pub fn set_package_name_prefixes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.package_name_prefixes = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_prefixes`.\nMatch versions by tag prefix. Applied on any prefix match."]
    pub fn set_tag_prefixes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tag_prefixes = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_state`.\nMatch versions by tag status. Default value: \"ANY\" Possible values: [\"TAGGED\", \"UNTAGGED\", \"ANY\"]"]
    pub fn set_tag_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_state = Some(v.into());
        self
    }
    #[doc = "Set the field `version_name_prefixes`.\nMatch versions by version name prefix. Applied on any prefix match."]
    pub fn set_version_name_prefixes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.version_name_prefixes = Some(v.into());
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryCleanupPoliciesElConditionEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryCleanupPoliciesElConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryCleanupPoliciesElConditionEl {}
impl BuildArtifactRegistryRepositoryCleanupPoliciesElConditionEl {
    pub fn build(self) -> ArtifactRegistryRepositoryCleanupPoliciesElConditionEl {
        ArtifactRegistryRepositoryCleanupPoliciesElConditionEl {
            newer_than: core::default::Default::default(),
            older_than: core::default::Default::default(),
            package_name_prefixes: core::default::Default::default(),
            tag_prefixes: core::default::Default::default(),
            tag_state: core::default::Default::default(),
            version_name_prefixes: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryCleanupPoliciesElConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryCleanupPoliciesElConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryCleanupPoliciesElConditionElRef {
        ArtifactRegistryRepositoryCleanupPoliciesElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryCleanupPoliciesElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `newer_than` after provisioning.\nMatch versions newer than a duration."]
    pub fn newer_than(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.newer_than", self.base))
    }
    #[doc = "Get a reference to the value of field `older_than` after provisioning.\nMatch versions older than a duration."]
    pub fn older_than(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.older_than", self.base))
    }
    #[doc = "Get a reference to the value of field `package_name_prefixes` after provisioning.\nMatch versions by package prefix. Applied on any prefix match."]
    pub fn package_name_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.package_name_prefixes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_prefixes` after provisioning.\nMatch versions by tag prefix. Applied on any prefix match."]
    pub fn tag_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tag_prefixes", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_state` after provisioning.\nMatch versions by tag status. Default value: \"ANY\" Possible values: [\"TAGGED\", \"UNTAGGED\", \"ANY\"]"]
    pub fn tag_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_state", self.base))
    }
    #[doc = "Get a reference to the value of field `version_name_prefixes` after provisioning.\nMatch versions by version name prefix. Applied on any prefix match."]
    pub fn version_name_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version_name_prefixes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_name_prefixes: Option<ListField<PrimField<String>>>,
}
impl ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsEl {
    #[doc = "Set the field `keep_count`.\nMinimum number of versions to keep."]
    pub fn set_keep_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.keep_count = Some(v.into());
        self
    }
    #[doc = "Set the field `package_name_prefixes`.\nMatch versions by package prefix. Applied on any prefix match."]
    pub fn set_package_name_prefixes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.package_name_prefixes = Some(v.into());
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsEl {}
impl BuildArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsEl {
    pub fn build(self) -> ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsEl {
        ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsEl {
            keep_count: core::default::Default::default(),
            package_name_prefixes: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsElRef {
        ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `keep_count` after provisioning.\nMinimum number of versions to keep."]
    pub fn keep_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_count", self.base))
    }
    #[doc = "Get a reference to the value of field `package_name_prefixes` after provisioning.\nMatch versions by package prefix. Applied on any prefix match."]
    pub fn package_name_prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.package_name_prefixes", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryCleanupPoliciesElDynamic {
    condition: Option<DynamicBlock<ArtifactRegistryRepositoryCleanupPoliciesElConditionEl>>,
    most_recent_versions:
        Option<DynamicBlock<ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsEl>>,
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryCleanupPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<ArtifactRegistryRepositoryCleanupPoliciesElConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    most_recent_versions:
        Option<Vec<ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsEl>>,
    dynamic: ArtifactRegistryRepositoryCleanupPoliciesElDynamic,
}
impl ArtifactRegistryRepositoryCleanupPoliciesEl {
    #[doc = "Set the field `action`.\nPolicy action. Possible values: [\"DELETE\", \"KEEP\"]"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<ArtifactRegistryRepositoryCleanupPoliciesElConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `most_recent_versions`.\n"]
    pub fn set_most_recent_versions(
        mut self,
        v: impl Into<BlockAssignable<ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.most_recent_versions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.most_recent_versions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryCleanupPoliciesEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryCleanupPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryCleanupPoliciesEl {
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildArtifactRegistryRepositoryCleanupPoliciesEl {
    pub fn build(self) -> ArtifactRegistryRepositoryCleanupPoliciesEl {
        ArtifactRegistryRepositoryCleanupPoliciesEl {
            action: core::default::Default::default(),
            id: self.id,
            condition: core::default::Default::default(),
            most_recent_versions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryCleanupPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryCleanupPoliciesElRef {
    fn new(shared: StackShared, base: String) -> ArtifactRegistryRepositoryCleanupPoliciesElRef {
        ArtifactRegistryRepositoryCleanupPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryCleanupPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nPolicy action. Possible values: [\"DELETE\", \"KEEP\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<ArtifactRegistryRepositoryCleanupPoliciesElConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
    #[doc = "Get a reference to the value of field `most_recent_versions` after provisioning.\n"]
    pub fn most_recent_versions(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryCleanupPoliciesElMostRecentVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.most_recent_versions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryDockerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    immutable_tags: Option<PrimField<bool>>,
}
impl ArtifactRegistryRepositoryDockerConfigEl {
    #[doc = "Set the field `immutable_tags`.\nThe repository which enabled this flag prevents all tags from being modified, moved or deleted. This does not prevent tags from being created."]
    pub fn set_immutable_tags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.immutable_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryDockerConfigEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryDockerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryDockerConfigEl {}
impl BuildArtifactRegistryRepositoryDockerConfigEl {
    pub fn build(self) -> ArtifactRegistryRepositoryDockerConfigEl {
        ArtifactRegistryRepositoryDockerConfigEl {
            immutable_tags: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryDockerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryDockerConfigElRef {
    fn new(shared: StackShared, base: String) -> ArtifactRegistryRepositoryDockerConfigElRef {
        ArtifactRegistryRepositoryDockerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryDockerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `immutable_tags` after provisioning.\nThe repository which enabled this flag prevents all tags from being modified, moved or deleted. This does not prevent tags from being created."]
    pub fn immutable_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.immutable_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryMavenConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_snapshot_overwrites: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_policy: Option<PrimField<String>>,
}
impl ArtifactRegistryRepositoryMavenConfigEl {
    #[doc = "Set the field `allow_snapshot_overwrites`.\nThe repository with this flag will allow publishing the same\nsnapshot versions."]
    pub fn set_allow_snapshot_overwrites(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_snapshot_overwrites = Some(v.into());
        self
    }
    #[doc = "Set the field `version_policy`.\nVersion policy defines the versions that the registry will accept. Default value: \"VERSION_POLICY_UNSPECIFIED\" Possible values: [\"VERSION_POLICY_UNSPECIFIED\", \"RELEASE\", \"SNAPSHOT\"]"]
    pub fn set_version_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_policy = Some(v.into());
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryMavenConfigEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryMavenConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryMavenConfigEl {}
impl BuildArtifactRegistryRepositoryMavenConfigEl {
    pub fn build(self) -> ArtifactRegistryRepositoryMavenConfigEl {
        ArtifactRegistryRepositoryMavenConfigEl {
            allow_snapshot_overwrites: core::default::Default::default(),
            version_policy: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryMavenConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryMavenConfigElRef {
    fn new(shared: StackShared, base: String) -> ArtifactRegistryRepositoryMavenConfigElRef {
        ArtifactRegistryRepositoryMavenConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryMavenConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_snapshot_overwrites` after provisioning.\nThe repository with this flag will allow publishing the same\nsnapshot versions."]
    pub fn allow_snapshot_overwrites(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_snapshot_overwrites", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version_policy` after provisioning.\nVersion policy defines the versions that the registry will accept. Default value: \"VERSION_POLICY_UNSPECIFIED\" Possible values: [\"VERSION_POLICY_UNSPECIFIED\", \"RELEASE\", \"SNAPSHOT\"]"]
    pub fn version_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {
    repository_base: PrimField<String>,
    repository_path: PrimField<String>,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {}
impl ToListMappable
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl
{
    type O = BlockAssignable<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl
{
    #[doc = "A common public repository base for Apt, e.g. '\"debian/dists/stable\"' Possible values: [\"DEBIAN\", \"UBUNTU\", \"DEBIAN_SNAPSHOT\"]"]
    pub repository_base: PrimField<String>,
    #[doc = "Specific repository from the base."]
    pub repository_path: PrimField<String>,
}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {
    pub fn build(
        self,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl {
            repository_base: self.repository_base,
            repository_path: self.repository_path,
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef
    {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository_base` after provisioning.\nA common public repository base for Apt, e.g. '\"debian/dists/stable\"' Possible values: [\"DEBIAN\", \"UBUNTU\", \"DEBIAN_SNAPSHOT\"]"]
    pub fn repository_base(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_base", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `repository_path` after provisioning.\nSpecific repository from the base."]
    pub fn repository_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_path", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElDynamic {
    public_repository: Option<
        DynamicBlock<
            ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    public_repository: Option<
        Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl>,
    >,
    dynamic: ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElDynamic,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(
        mut self,
        v: impl Into<
            BlockAssignable<
                ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.public_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.public_repository = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
    pub fn build(self) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl {
            public_repository: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(
        &self,
    ) -> ListRef<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElPublicRepositoryElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_repository", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryEl {
    uri: PrimField<String>,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryEl {}
impl ToListMappable for ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryEl {
    #[doc = "One of:\na. Artifact Registry Repository resource, e.g. 'projects/UPSTREAM_PROJECT_ID/locations/REGION/repositories/UPSTREAM_REPOSITORY'\nb. URI to the registry, e.g. '\"https://registry-1.docker.io\"'\nc. URI to Artifact Registry Repository, e.g. '\"https://REGION-docker.pkg.dev/UPSTREAM_PROJECT_ID/UPSTREAM_REPOSITORY\"'"]
    pub uri: PrimField<String>,
}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryEl {
    pub fn build(self) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryEl { uri: self.uri }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryElRef {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nOne of:\na. Artifact Registry Repository resource, e.g. 'projects/UPSTREAM_PROJECT_ID/locations/REGION/repositories/UPSTREAM_REPOSITORY'\nb. URI to the registry, e.g. '\"https://registry-1.docker.io\"'\nc. URI to Artifact Registry Repository, e.g. '\"https://REGION-docker.pkg.dev/UPSTREAM_PROJECT_ID/UPSTREAM_REPOSITORY\"'"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryEl {
    #[doc = "Set the field `uri`.\nSpecific uri to the registry, e.g. '\"https://registry-1.docker.io\"'"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryEl
{
    type O = BlockAssignable<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryEl
{}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryEl {
    pub fn build(
        self,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryEl
    {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryElRef
    {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nSpecific uri to the registry, e.g. '\"https://registry-1.docker.io\"'"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElDynamic {
    custom_repository: Option<
        DynamicBlock<
            ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    public_repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_repository: Option<
        Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryEl>,
    >,
    dynamic: ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElDynamic,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
    #[doc = "Set the field `public_repository`.\nAddress of the remote repository. Possible values: [\"DOCKER_HUB\"]"]
    pub fn set_public_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_repository = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v : impl Into < BlockAssignable < ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_repository = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
    pub fn build(self) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl {
            public_repository: core::default::Default::default(),
            custom_repository: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\nAddress of the remote repository. Possible values: [\"DOCKER_HUB\"]"]
    pub fn public_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> ListRef<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElCustomRepositoryElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_repository", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryEl {
    #[doc = "Set the field `uri`.\nSpecific uri to the registry, e.g. '\"https://repo.maven.apache.org/maven2\"'"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryEl
{
    type O = BlockAssignable<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryEl
{}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryEl {
    pub fn build(
        self,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryElRef
    {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nSpecific uri to the registry, e.g. '\"https://repo.maven.apache.org/maven2\"'"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElDynamic {
    custom_repository: Option<
        DynamicBlock<
            ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    public_repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_repository: Option<
        Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryEl>,
    >,
    dynamic: ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElDynamic,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
    #[doc = "Set the field `public_repository`.\nAddress of the remote repository. Possible values: [\"MAVEN_CENTRAL\"]"]
    pub fn set_public_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_repository = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v : impl Into < BlockAssignable < ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_repository = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
    pub fn build(self) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl {
            public_repository: core::default::Default::default(),
            custom_repository: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\nAddress of the remote repository. Possible values: [\"MAVEN_CENTRAL\"]"]
    pub fn public_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> ListRef<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElCustomRepositoryElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_repository", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryEl {
    #[doc = "Set the field `uri`.\nSpecific uri to the registry, e.g. '\"https://registry.npmjs.org\"'"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryEl
{
    type O = BlockAssignable<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryEl
{}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryEl {
    pub fn build(
        self,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryElRef
    {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nSpecific uri to the registry, e.g. '\"https://registry.npmjs.org\"'"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElDynamic {
    custom_repository: Option<
        DynamicBlock<
            ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    public_repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_repository: Option<
        Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryEl>,
    >,
    dynamic: ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElDynamic,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
    #[doc = "Set the field `public_repository`.\nAddress of the remote repository. Possible values: [\"NPMJS\"]"]
    pub fn set_public_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_repository = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v: impl Into<
            BlockAssignable<
                ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_repository = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
    pub fn build(self) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl {
            public_repository: core::default::Default::default(),
            custom_repository: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\nAddress of the remote repository. Possible values: [\"NPMJS\"]"]
    pub fn public_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> ListRef<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElCustomRepositoryElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_repository", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryEl {
    #[doc = "Set the field `uri`.\nSpecific uri to the registry, e.g. '\"https://pypi.io\"'"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryEl
{
    type O = BlockAssignable<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryEl
{}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryEl {
    pub fn build(
        self,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryEl
    {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryElRef
    {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nSpecific uri to the registry, e.g. '\"https://pypi.io\"'"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElDynamic {
    custom_repository: Option<
        DynamicBlock<
            ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    public_repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_repository: Option<
        Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryEl>,
    >,
    dynamic: ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElDynamic,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
    #[doc = "Set the field `public_repository`.\nAddress of the remote repository. Possible values: [\"PYPI\"]"]
    pub fn set_public_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_repository = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_repository`.\n"]
    pub fn set_custom_repository(
        mut self,
        v : impl Into < BlockAssignable < ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_repository = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
    pub fn build(self) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl {
            public_repository: core::default::Default::default(),
            custom_repository: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\nAddress of the remote repository. Possible values: [\"PYPI\"]"]
    pub fn public_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_repository` after provisioning.\n"]
    pub fn custom_repository(
        &self,
    ) -> ListRef<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElCustomRepositoryElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_repository", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    password_secret_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl { # [doc = "Set the field `password_secret_version`.\nThe Secret Manager key version that holds the password to access the\nremote repository. Must be in the format of\n'projects/{project}/secrets/{secret}/versions/{version}'."] pub fn set_password_secret_version (mut self , v : impl Into < PrimField < String > >) -> Self { self . password_secret_version = Some (v . into ()) ; self } # [doc = "Set the field `username`.\nThe username to access the remote repository."] pub fn set_username (mut self , v : impl Into < PrimField < String > >) -> Self { self . username = Some (v . into ()) ; self } }
impl ToListMappable for ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl { type O = BlockAssignable < ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl
{}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl { pub fn build (self) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl { ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl { password_secret_version : core :: default :: Default :: default () , username : core :: default :: Default :: default () , } } }
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef { fn new (shared : StackShared , base : String) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef { ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef { shared : shared , base : base . to_string () , } } }
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `password_secret_version` after provisioning.\nThe Secret Manager key version that holds the password to access the\nremote repository. Must be in the format of\n'projects/{project}/secrets/{secret}/versions/{version}'."] pub fn password_secret_version (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.password_secret_version" , self . base)) } # [doc = "Get a reference to the value of field `username` after provisioning.\nThe username to access the remote repository."] pub fn username (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.username" , self . base)) } }
#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElDynamic { username_password_credentials : Option < DynamicBlock < ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl >> , }
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl { # [serde (skip_serializing_if = "Option::is_none")] username_password_credentials : Option < Vec < ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl > > , dynamic : ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElDynamic , }
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {
    #[doc = "Set the field `username_password_credentials`.\n"]
    pub fn set_username_password_credentials(
        mut self,
        v : impl Into < BlockAssignable < ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.username_password_credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.username_password_credentials = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {
    type O =
        BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {
    pub fn build(self) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl {
            username_password_credentials: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `username_password_credentials` after provisioning.\n"]    pub fn username_password_credentials (& self) -> ListRef < ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElUsernamePasswordCredentialsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.username_password_credentials", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {
    repository_base: PrimField<String>,
    repository_path: PrimField<String>,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {}
impl ToListMappable
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl
{
    type O = BlockAssignable<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl
{
    #[doc = "A common public repository base for Yum. Possible values: [\"CENTOS\", \"CENTOS_DEBUG\", \"CENTOS_VAULT\", \"CENTOS_STREAM\", \"ROCKY\", \"EPEL\"]"]
    pub repository_base: PrimField<String>,
    #[doc = "Specific repository from the base, e.g. '\"pub/rocky/9/BaseOS/x86_64/os\"'"]
    pub repository_path: PrimField<String>,
}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {
    pub fn build(
        self,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl {
            repository_base: self.repository_base,
            repository_path: self.repository_path,
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef
    {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository_base` after provisioning.\nA common public repository base for Yum. Possible values: [\"CENTOS\", \"CENTOS_DEBUG\", \"CENTOS_VAULT\", \"CENTOS_STREAM\", \"ROCKY\", \"EPEL\"]"]
    pub fn repository_base(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_base", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `repository_path` after provisioning.\nSpecific repository from the base, e.g. '\"pub/rocky/9/BaseOS/x86_64/os\"'"]
    pub fn repository_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_path", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElDynamic {
    public_repository: Option<
        DynamicBlock<
            ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    public_repository: Option<
        Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl>,
    >,
    dynamic: ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElDynamic,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
    #[doc = "Set the field `public_repository`.\n"]
    pub fn set_public_repository(
        mut self,
        v: impl Into<
            BlockAssignable<
                ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.public_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.public_repository = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
    pub fn build(self) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl {
            public_repository: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `public_repository` after provisioning.\n"]
    pub fn public_repository(
        &self,
    ) -> ListRef<
        ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElPublicRepositoryElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_repository", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryRemoteRepositoryConfigElDynamic {
    apt_repository:
        Option<DynamicBlock<ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl>>,
    common_repository:
        Option<DynamicBlock<ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryEl>>,
    docker_repository:
        Option<DynamicBlock<ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl>>,
    maven_repository:
        Option<DynamicBlock<ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl>>,
    npm_repository:
        Option<DynamicBlock<ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl>>,
    python_repository:
        Option<DynamicBlock<ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl>>,
    upstream_credentials: Option<
        DynamicBlock<ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl>,
    >,
    yum_repository:
        Option<DynamicBlock<ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl>>,
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_upstream_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apt_repository: Option<Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_repository:
        Option<Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docker_repository:
        Option<Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maven_repository:
        Option<Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    npm_repository: Option<Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_repository:
        Option<Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upstream_credentials:
        Option<Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    yum_repository: Option<Vec<ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl>>,
    dynamic: ArtifactRegistryRepositoryRemoteRepositoryConfigElDynamic,
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigEl {
    #[doc = "Set the field `description`.\nThe description of the remote source."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_upstream_validation`.\nIf true, the remote repository upstream and upstream credentials will\nnot be validated."]
    pub fn set_disable_upstream_validation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_upstream_validation = Some(v.into());
        self
    }
    #[doc = "Set the field `apt_repository`.\n"]
    pub fn set_apt_repository(
        mut self,
        v: impl Into<BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.apt_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.apt_repository = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `common_repository`.\n"]
    pub fn set_common_repository(
        mut self,
        v: impl Into<
            BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.common_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.common_repository = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `docker_repository`.\n"]
    pub fn set_docker_repository(
        mut self,
        v: impl Into<
            BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.docker_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.docker_repository = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `maven_repository`.\n"]
    pub fn set_maven_repository(
        mut self,
        v: impl Into<
            BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.maven_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.maven_repository = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `npm_repository`.\n"]
    pub fn set_npm_repository(
        mut self,
        v: impl Into<BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.npm_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.npm_repository = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `python_repository`.\n"]
    pub fn set_python_repository(
        mut self,
        v: impl Into<
            BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.python_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.python_repository = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `upstream_credentials`.\n"]
    pub fn set_upstream_credentials(
        mut self,
        v: impl Into<
            BlockAssignable<
                ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.upstream_credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.upstream_credentials = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `yum_repository`.\n"]
    pub fn set_yum_repository(
        mut self,
        v: impl Into<BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.yum_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.yum_repository = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryRemoteRepositoryConfigEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryRemoteRepositoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryRemoteRepositoryConfigEl {}
impl BuildArtifactRegistryRepositoryRemoteRepositoryConfigEl {
    pub fn build(self) -> ArtifactRegistryRepositoryRemoteRepositoryConfigEl {
        ArtifactRegistryRepositoryRemoteRepositoryConfigEl {
            description: core::default::Default::default(),
            disable_upstream_validation: core::default::Default::default(),
            apt_repository: core::default::Default::default(),
            common_repository: core::default::Default::default(),
            docker_repository: core::default::Default::default(),
            maven_repository: core::default::Default::default(),
            npm_repository: core::default::Default::default(),
            python_repository: core::default::Default::default(),
            upstream_credentials: core::default::Default::default(),
            yum_repository: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryRemoteRepositoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryRemoteRepositoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryRemoteRepositoryConfigElRef {
        ArtifactRegistryRepositoryRemoteRepositoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryRemoteRepositoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the remote source."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_upstream_validation` after provisioning.\nIf true, the remote repository upstream and upstream credentials will\nnot be validated."]
    pub fn disable_upstream_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_upstream_validation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `apt_repository` after provisioning.\n"]
    pub fn apt_repository(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryRemoteRepositoryConfigElAptRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.apt_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `common_repository` after provisioning.\n"]
    pub fn common_repository(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryRemoteRepositoryConfigElCommonRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.common_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `docker_repository` after provisioning.\n"]
    pub fn docker_repository(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryRemoteRepositoryConfigElDockerRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.docker_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maven_repository` after provisioning.\n"]
    pub fn maven_repository(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryRemoteRepositoryConfigElMavenRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maven_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `npm_repository` after provisioning.\n"]
    pub fn npm_repository(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryRemoteRepositoryConfigElNpmRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.npm_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `python_repository` after provisioning.\n"]
    pub fn python_repository(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryRemoteRepositoryConfigElPythonRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.python_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upstream_credentials` after provisioning.\n"]
    pub fn upstream_credentials(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryRemoteRepositoryConfigElUpstreamCredentialsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upstream_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `yum_repository` after provisioning.\n"]
    pub fn yum_repository(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryRemoteRepositoryConfigElYumRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.yum_repository", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ArtifactRegistryRepositoryTimeoutsEl {
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
impl ToListMappable for ArtifactRegistryRepositoryTimeoutsEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryTimeoutsEl {}
impl BuildArtifactRegistryRepositoryTimeoutsEl {
    pub fn build(self) -> ArtifactRegistryRepositoryTimeoutsEl {
        ArtifactRegistryRepositoryTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ArtifactRegistryRepositoryTimeoutsElRef {
        ArtifactRegistryRepositoryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryTimeoutsElRef {
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
pub struct ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
}
impl ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
    #[doc = "Set the field `id`.\nThe user-provided ID of the upstream policy."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\nEntries with a greater priority value take precedence in the pull order."]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\nA reference to the repository resource, for example:\n\"projects/p1/locations/us-central1/repository/repo1\"."]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {}
impl BuildArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
    pub fn build(self) -> ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
        ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl {
            id: core::default::Default::default(),
            priority: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef {
        ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe user-provided ID of the upstream policy."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nEntries with a greater priority value take precedence in the pull order."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nA reference to the repository resource, for example:\n\"projects/p1/locations/us-central1/repository/repo1\"."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}
#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryVirtualRepositoryConfigElDynamic {
    upstream_policies:
        Option<DynamicBlock<ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl>>,
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryVirtualRepositoryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    upstream_policies:
        Option<Vec<ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl>>,
    dynamic: ArtifactRegistryRepositoryVirtualRepositoryConfigElDynamic,
}
impl ArtifactRegistryRepositoryVirtualRepositoryConfigEl {
    #[doc = "Set the field `upstream_policies`.\n"]
    pub fn set_upstream_policies(
        mut self,
        v: impl Into<
            BlockAssignable<ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.upstream_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.upstream_policies = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryVirtualRepositoryConfigEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryVirtualRepositoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryVirtualRepositoryConfigEl {}
impl BuildArtifactRegistryRepositoryVirtualRepositoryConfigEl {
    pub fn build(self) -> ArtifactRegistryRepositoryVirtualRepositoryConfigEl {
        ArtifactRegistryRepositoryVirtualRepositoryConfigEl {
            upstream_policies: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryVirtualRepositoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryVirtualRepositoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryVirtualRepositoryConfigElRef {
        ArtifactRegistryRepositoryVirtualRepositoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryVirtualRepositoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `upstream_policies` after provisioning.\n"]
    pub fn upstream_policies(
        &self,
    ) -> ListRef<ArtifactRegistryRepositoryVirtualRepositoryConfigElUpstreamPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upstream_policies", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRepositoryVulnerabilityScanningConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enablement_config: Option<PrimField<String>>,
}
impl ArtifactRegistryRepositoryVulnerabilityScanningConfigEl {
    #[doc = "Set the field `enablement_config`.\nThis configures whether vulnerability scanning is automatically performed for artifacts pushed to this repository. Possible values: [\"INHERITED\", \"DISABLED\"]"]
    pub fn set_enablement_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enablement_config = Some(v.into());
        self
    }
}
impl ToListMappable for ArtifactRegistryRepositoryVulnerabilityScanningConfigEl {
    type O = BlockAssignable<ArtifactRegistryRepositoryVulnerabilityScanningConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRepositoryVulnerabilityScanningConfigEl {}
impl BuildArtifactRegistryRepositoryVulnerabilityScanningConfigEl {
    pub fn build(self) -> ArtifactRegistryRepositoryVulnerabilityScanningConfigEl {
        ArtifactRegistryRepositoryVulnerabilityScanningConfigEl {
            enablement_config: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRepositoryVulnerabilityScanningConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRepositoryVulnerabilityScanningConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ArtifactRegistryRepositoryVulnerabilityScanningConfigElRef {
        ArtifactRegistryRepositoryVulnerabilityScanningConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRepositoryVulnerabilityScanningConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enablement_config` after provisioning.\nThis configures whether vulnerability scanning is automatically performed for artifacts pushed to this repository. Possible values: [\"INHERITED\", \"DISABLED\"]"]
    pub fn enablement_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enablement_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enablement_state` after provisioning.\nThis field returns whether scanning is active for this repository."]
    pub fn enablement_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enablement_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enablement_state_reason` after provisioning.\nThis provides an explanation for the state of scanning on this repository."]
    pub fn enablement_state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enablement_state_reason", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ArtifactRegistryRepositoryDynamic {
    cleanup_policies: Option<DynamicBlock<ArtifactRegistryRepositoryCleanupPoliciesEl>>,
    docker_config: Option<DynamicBlock<ArtifactRegistryRepositoryDockerConfigEl>>,
    maven_config: Option<DynamicBlock<ArtifactRegistryRepositoryMavenConfigEl>>,
    remote_repository_config:
        Option<DynamicBlock<ArtifactRegistryRepositoryRemoteRepositoryConfigEl>>,
    virtual_repository_config:
        Option<DynamicBlock<ArtifactRegistryRepositoryVirtualRepositoryConfigEl>>,
    vulnerability_scanning_config:
        Option<DynamicBlock<ArtifactRegistryRepositoryVulnerabilityScanningConfigEl>>,
}
