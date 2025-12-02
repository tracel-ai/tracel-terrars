use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CodebuildProjectData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_retry_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    badge_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrent_build_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queued_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_access_role: Option<PrimField<String>>,
    service_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artifacts: Option<Vec<CodebuildProjectArtifactsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_batch_config: Option<Vec<CodebuildProjectBuildBatchConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache: Option<Vec<CodebuildProjectCacheEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<CodebuildProjectEnvironmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_locations: Option<Vec<CodebuildProjectFileSystemLocationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logs_config: Option<Vec<CodebuildProjectLogsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_artifacts: Option<Vec<CodebuildProjectSecondaryArtifactsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_source_version: Option<Vec<CodebuildProjectSecondarySourceVersionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_sources: Option<Vec<CodebuildProjectSecondarySourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<CodebuildProjectSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<CodebuildProjectVpcConfigEl>>,
    dynamic: CodebuildProjectDynamic,
}
struct CodebuildProject_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodebuildProjectData>,
}
#[derive(Clone)]
pub struct CodebuildProject(Rc<CodebuildProject_>);
impl CodebuildProject {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `auto_retry_limit`.\nMaximum number of additional automatic retries after a failed build. The default value is 0."]
    pub fn set_auto_retry_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().auto_retry_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `badge_enabled`.\n"]
    pub fn set_badge_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().badge_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `build_timeout`.\n"]
    pub fn set_build_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().build_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `concurrent_build_limit`.\n"]
    pub fn set_concurrent_build_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().concurrent_build_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project_visibility`.\n"]
    pub fn set_project_visibility(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project_visibility = Some(v.into());
        self
    }
    #[doc = "Set the field `queued_timeout`.\n"]
    pub fn set_queued_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().queued_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_access_role`.\n"]
    pub fn set_resource_access_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_access_role = Some(v.into());
        self
    }
    #[doc = "Set the field `source_version`.\n"]
    pub fn set_source_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_version = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `artifacts`.\n"]
    pub fn set_artifacts(self, v: impl Into<BlockAssignable<CodebuildProjectArtifactsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().artifacts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.artifacts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `build_batch_config`.\n"]
    pub fn set_build_batch_config(
        self,
        v: impl Into<BlockAssignable<CodebuildProjectBuildBatchConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().build_batch_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.build_batch_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cache`.\n"]
    pub fn set_cache(self, v: impl Into<BlockAssignable<CodebuildProjectCacheEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        self,
        v: impl Into<BlockAssignable<CodebuildProjectEnvironmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `file_system_locations`.\n"]
    pub fn set_file_system_locations(
        self,
        v: impl Into<BlockAssignable<CodebuildProjectFileSystemLocationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().file_system_locations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.file_system_locations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logs_config`.\n"]
    pub fn set_logs_config(
        self,
        v: impl Into<BlockAssignable<CodebuildProjectLogsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logs_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logs_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secondary_artifacts`.\n"]
    pub fn set_secondary_artifacts(
        self,
        v: impl Into<BlockAssignable<CodebuildProjectSecondaryArtifactsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_artifacts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_artifacts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secondary_source_version`.\n"]
    pub fn set_secondary_source_version(
        self,
        v: impl Into<BlockAssignable<CodebuildProjectSecondarySourceVersionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_source_version = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_source_version = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secondary_sources`.\n"]
    pub fn set_secondary_sources(
        self,
        v: impl Into<BlockAssignable<CodebuildProjectSecondarySourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_sources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl Into<BlockAssignable<CodebuildProjectSourceEl>>) -> Self {
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
    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl Into<BlockAssignable<CodebuildProjectVpcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `auto_retry_limit` after provisioning.\nMaximum number of additional automatic retries after a failed build. The default value is 0."]
    pub fn auto_retry_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_retry_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `badge_enabled` after provisioning.\n"]
    pub fn badge_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.badge_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `badge_url` after provisioning.\n"]
    pub fn badge_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.badge_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_timeout` after provisioning.\n"]
    pub fn build_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `concurrent_build_limit` after provisioning.\n"]
    pub fn concurrent_build_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.concurrent_build_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_visibility` after provisioning.\n"]
    pub fn project_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_visibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_project_alias` after provisioning.\n"]
    pub fn public_project_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_project_alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `queued_timeout` after provisioning.\n"]
    pub fn queued_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.queued_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_access_role` after provisioning.\n"]
    pub fn resource_access_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_access_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `artifacts` after provisioning.\n"]
    pub fn artifacts(&self) -> ListRef<CodebuildProjectArtifactsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.artifacts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_batch_config` after provisioning.\n"]
    pub fn build_batch_config(&self) -> ListRef<CodebuildProjectBuildBatchConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_batch_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cache` after provisioning.\n"]
    pub fn cache(&self) -> ListRef<CodebuildProjectCacheElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> ListRef<CodebuildProjectEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_config` after provisioning.\n"]
    pub fn logs_config(&self) -> ListRef<CodebuildProjectLogsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logs_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<CodebuildProjectSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<CodebuildProjectVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
impl Referable for CodebuildProject {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CodebuildProject {}
impl ToListMappable for CodebuildProject {
    type O = ListRef<CodebuildProjectRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CodebuildProject_ {
    fn extract_resource_type(&self) -> String {
        "aws_codebuild_project".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCodebuildProject {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub service_role: PrimField<String>,
}
impl BuildCodebuildProject {
    pub fn build(self, stack: &mut Stack) -> CodebuildProject {
        let out = CodebuildProject(Rc::new(CodebuildProject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodebuildProjectData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_retry_limit: core::default::Default::default(),
                badge_enabled: core::default::Default::default(),
                build_timeout: core::default::Default::default(),
                concurrent_build_limit: core::default::Default::default(),
                description: core::default::Default::default(),
                encryption_key: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project_visibility: core::default::Default::default(),
                queued_timeout: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_access_role: core::default::Default::default(),
                service_role: self.service_role,
                source_version: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                artifacts: core::default::Default::default(),
                build_batch_config: core::default::Default::default(),
                cache: core::default::Default::default(),
                environment: core::default::Default::default(),
                file_system_locations: core::default::Default::default(),
                logs_config: core::default::Default::default(),
                secondary_artifacts: core::default::Default::default(),
                secondary_source_version: core::default::Default::default(),
                secondary_sources: core::default::Default::default(),
                source: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CodebuildProjectRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CodebuildProjectRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `auto_retry_limit` after provisioning.\nMaximum number of additional automatic retries after a failed build. The default value is 0."]
    pub fn auto_retry_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_retry_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `badge_enabled` after provisioning.\n"]
    pub fn badge_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.badge_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `badge_url` after provisioning.\n"]
    pub fn badge_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.badge_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_timeout` after provisioning.\n"]
    pub fn build_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `concurrent_build_limit` after provisioning.\n"]
    pub fn concurrent_build_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.concurrent_build_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_visibility` after provisioning.\n"]
    pub fn project_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_visibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_project_alias` after provisioning.\n"]
    pub fn public_project_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_project_alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `queued_timeout` after provisioning.\n"]
    pub fn queued_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.queued_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_access_role` after provisioning.\n"]
    pub fn resource_access_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_access_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `artifacts` after provisioning.\n"]
    pub fn artifacts(&self) -> ListRef<CodebuildProjectArtifactsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.artifacts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_batch_config` after provisioning.\n"]
    pub fn build_batch_config(&self) -> ListRef<CodebuildProjectBuildBatchConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_batch_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cache` after provisioning.\n"]
    pub fn cache(&self) -> ListRef<CodebuildProjectCacheElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> ListRef<CodebuildProjectEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_config` after provisioning.\n"]
    pub fn logs_config(&self) -> ListRef<CodebuildProjectLogsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logs_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<CodebuildProjectSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<CodebuildProjectVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectArtifactsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_owner_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_artifact_name: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packaging: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl CodebuildProjectArtifactsEl {
    #[doc = "Set the field `artifact_identifier`.\n"]
    pub fn set_artifact_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.artifact_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket_owner_access`.\n"]
    pub fn set_bucket_owner_access(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_owner_access = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_disabled`.\n"]
    pub fn set_encryption_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encryption_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace_type`.\n"]
    pub fn set_namespace_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace_type = Some(v.into());
        self
    }
    #[doc = "Set the field `override_artifact_name`.\n"]
    pub fn set_override_artifact_name(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_artifact_name = Some(v.into());
        self
    }
    #[doc = "Set the field `packaging`.\n"]
    pub fn set_packaging(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.packaging = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectArtifactsEl {
    type O = BlockAssignable<CodebuildProjectArtifactsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectArtifactsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildCodebuildProjectArtifactsEl {
    pub fn build(self) -> CodebuildProjectArtifactsEl {
        CodebuildProjectArtifactsEl {
            artifact_identifier: core::default::Default::default(),
            bucket_owner_access: core::default::Default::default(),
            encryption_disabled: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
            namespace_type: core::default::Default::default(),
            override_artifact_name: core::default::Default::default(),
            packaging: core::default::Default::default(),
            path: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct CodebuildProjectArtifactsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectArtifactsElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectArtifactsElRef {
        CodebuildProjectArtifactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectArtifactsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `artifact_identifier` after provisioning.\n"]
    pub fn artifact_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.artifact_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bucket_owner_access` after provisioning.\n"]
    pub fn bucket_owner_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_owner_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_disabled` after provisioning.\n"]
    pub fn encryption_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_disabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace_type` after provisioning.\n"]
    pub fn namespace_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `override_artifact_name` after provisioning.\n"]
    pub fn override_artifact_name(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_artifact_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `packaging` after provisioning.\n"]
    pub fn packaging(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.packaging", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectBuildBatchConfigElRestrictionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_types_allowed: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_builds_allowed: Option<PrimField<f64>>,
}
impl CodebuildProjectBuildBatchConfigElRestrictionsEl {
    #[doc = "Set the field `compute_types_allowed`.\n"]
    pub fn set_compute_types_allowed(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.compute_types_allowed = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_builds_allowed`.\n"]
    pub fn set_maximum_builds_allowed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_builds_allowed = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectBuildBatchConfigElRestrictionsEl {
    type O = BlockAssignable<CodebuildProjectBuildBatchConfigElRestrictionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectBuildBatchConfigElRestrictionsEl {}
impl BuildCodebuildProjectBuildBatchConfigElRestrictionsEl {
    pub fn build(self) -> CodebuildProjectBuildBatchConfigElRestrictionsEl {
        CodebuildProjectBuildBatchConfigElRestrictionsEl {
            compute_types_allowed: core::default::Default::default(),
            maximum_builds_allowed: core::default::Default::default(),
        }
    }
}
pub struct CodebuildProjectBuildBatchConfigElRestrictionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectBuildBatchConfigElRestrictionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodebuildProjectBuildBatchConfigElRestrictionsElRef {
        CodebuildProjectBuildBatchConfigElRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectBuildBatchConfigElRestrictionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compute_types_allowed` after provisioning.\n"]
    pub fn compute_types_allowed(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_types_allowed", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_builds_allowed` after provisioning.\n"]
    pub fn maximum_builds_allowed(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_builds_allowed", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CodebuildProjectBuildBatchConfigElDynamic {
    restrictions: Option<DynamicBlock<CodebuildProjectBuildBatchConfigElRestrictionsEl>>,
}
#[derive(Serialize)]
pub struct CodebuildProjectBuildBatchConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    combine_artifacts: Option<PrimField<bool>>,
    service_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_in_mins: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<Vec<CodebuildProjectBuildBatchConfigElRestrictionsEl>>,
    dynamic: CodebuildProjectBuildBatchConfigElDynamic,
}
impl CodebuildProjectBuildBatchConfigEl {
    #[doc = "Set the field `combine_artifacts`.\n"]
    pub fn set_combine_artifacts(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.combine_artifacts = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_in_mins`.\n"]
    pub fn set_timeout_in_mins(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_in_mins = Some(v.into());
        self
    }
    #[doc = "Set the field `restrictions`.\n"]
    pub fn set_restrictions(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectBuildBatchConfigElRestrictionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.restrictions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.restrictions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CodebuildProjectBuildBatchConfigEl {
    type O = BlockAssignable<CodebuildProjectBuildBatchConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectBuildBatchConfigEl {
    #[doc = ""]
    pub service_role: PrimField<String>,
}
impl BuildCodebuildProjectBuildBatchConfigEl {
    pub fn build(self) -> CodebuildProjectBuildBatchConfigEl {
        CodebuildProjectBuildBatchConfigEl {
            combine_artifacts: core::default::Default::default(),
            service_role: self.service_role,
            timeout_in_mins: core::default::Default::default(),
            restrictions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CodebuildProjectBuildBatchConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectBuildBatchConfigElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectBuildBatchConfigElRef {
        CodebuildProjectBuildBatchConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectBuildBatchConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `combine_artifacts` after provisioning.\n"]
    pub fn combine_artifacts(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.combine_artifacts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout_in_mins` after provisioning.\n"]
    pub fn timeout_in_mins(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_in_mins", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> ListRef<CodebuildProjectBuildBatchConfigElRestrictionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.restrictions", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectCacheEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modes: Option<ListField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl CodebuildProjectCacheEl {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `modes`.\n"]
    pub fn set_modes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.modes = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectCacheEl {
    type O = BlockAssignable<CodebuildProjectCacheEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectCacheEl {}
impl BuildCodebuildProjectCacheEl {
    pub fn build(self) -> CodebuildProjectCacheEl {
        CodebuildProjectCacheEl {
            location: core::default::Default::default(),
            modes: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CodebuildProjectCacheElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectCacheElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectCacheElRef {
        CodebuildProjectCacheElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectCacheElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `modes` after provisioning.\n"]
    pub fn modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.modes", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectEnvironmentElDockerServerEl {
    compute_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<ListField<PrimField<String>>>,
}
impl CodebuildProjectEnvironmentElDockerServerEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectEnvironmentElDockerServerEl {
    type O = BlockAssignable<CodebuildProjectEnvironmentElDockerServerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectEnvironmentElDockerServerEl {
    #[doc = ""]
    pub compute_type: PrimField<String>,
}
impl BuildCodebuildProjectEnvironmentElDockerServerEl {
    pub fn build(self) -> CodebuildProjectEnvironmentElDockerServerEl {
        CodebuildProjectEnvironmentElDockerServerEl {
            compute_type: self.compute_type,
            security_group_ids: core::default::Default::default(),
        }
    }
}
pub struct CodebuildProjectEnvironmentElDockerServerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectEnvironmentElDockerServerElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectEnvironmentElDockerServerElRef {
        CodebuildProjectEnvironmentElDockerServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectEnvironmentElDockerServerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_type", self.base))
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectEnvironmentElEnvironmentVariableEl {
    name: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    value: PrimField<String>,
}
impl CodebuildProjectEnvironmentElEnvironmentVariableEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectEnvironmentElEnvironmentVariableEl {
    type O = BlockAssignable<CodebuildProjectEnvironmentElEnvironmentVariableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectEnvironmentElEnvironmentVariableEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildCodebuildProjectEnvironmentElEnvironmentVariableEl {
    pub fn build(self) -> CodebuildProjectEnvironmentElEnvironmentVariableEl {
        CodebuildProjectEnvironmentElEnvironmentVariableEl {
            name: self.name,
            type_: core::default::Default::default(),
            value: self.value,
        }
    }
}
pub struct CodebuildProjectEnvironmentElEnvironmentVariableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectEnvironmentElEnvironmentVariableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodebuildProjectEnvironmentElEnvironmentVariableElRef {
        CodebuildProjectEnvironmentElEnvironmentVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectEnvironmentElEnvironmentVariableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectEnvironmentElFleetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_arn: Option<PrimField<String>>,
}
impl CodebuildProjectEnvironmentElFleetEl {
    #[doc = "Set the field `fleet_arn`.\n"]
    pub fn set_fleet_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fleet_arn = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectEnvironmentElFleetEl {
    type O = BlockAssignable<CodebuildProjectEnvironmentElFleetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectEnvironmentElFleetEl {}
impl BuildCodebuildProjectEnvironmentElFleetEl {
    pub fn build(self) -> CodebuildProjectEnvironmentElFleetEl {
        CodebuildProjectEnvironmentElFleetEl {
            fleet_arn: core::default::Default::default(),
        }
    }
}
pub struct CodebuildProjectEnvironmentElFleetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectEnvironmentElFleetElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectEnvironmentElFleetElRef {
        CodebuildProjectEnvironmentElFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectEnvironmentElFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fleet_arn` after provisioning.\n"]
    pub fn fleet_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fleet_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectEnvironmentElRegistryCredentialEl {
    credential: PrimField<String>,
    credential_provider: PrimField<String>,
}
impl CodebuildProjectEnvironmentElRegistryCredentialEl {}
impl ToListMappable for CodebuildProjectEnvironmentElRegistryCredentialEl {
    type O = BlockAssignable<CodebuildProjectEnvironmentElRegistryCredentialEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectEnvironmentElRegistryCredentialEl {
    #[doc = ""]
    pub credential: PrimField<String>,
    #[doc = ""]
    pub credential_provider: PrimField<String>,
}
impl BuildCodebuildProjectEnvironmentElRegistryCredentialEl {
    pub fn build(self) -> CodebuildProjectEnvironmentElRegistryCredentialEl {
        CodebuildProjectEnvironmentElRegistryCredentialEl {
            credential: self.credential,
            credential_provider: self.credential_provider,
        }
    }
}
pub struct CodebuildProjectEnvironmentElRegistryCredentialElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectEnvironmentElRegistryCredentialElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodebuildProjectEnvironmentElRegistryCredentialElRef {
        CodebuildProjectEnvironmentElRegistryCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectEnvironmentElRegistryCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `credential` after provisioning.\n"]
    pub fn credential(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.credential", self.base))
    }
    #[doc = "Get a reference to the value of field `credential_provider` after provisioning.\n"]
    pub fn credential_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_provider", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CodebuildProjectEnvironmentElDynamic {
    docker_server: Option<DynamicBlock<CodebuildProjectEnvironmentElDockerServerEl>>,
    environment_variable: Option<DynamicBlock<CodebuildProjectEnvironmentElEnvironmentVariableEl>>,
    fleet: Option<DynamicBlock<CodebuildProjectEnvironmentElFleetEl>>,
    registry_credential: Option<DynamicBlock<CodebuildProjectEnvironmentElRegistryCredentialEl>>,
}
#[derive(Serialize)]
pub struct CodebuildProjectEnvironmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<String>>,
    compute_type: PrimField<String>,
    image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_pull_credentials_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privileged_mode: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docker_server: Option<Vec<CodebuildProjectEnvironmentElDockerServerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variable: Option<Vec<CodebuildProjectEnvironmentElEnvironmentVariableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet: Option<Vec<CodebuildProjectEnvironmentElFleetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_credential: Option<Vec<CodebuildProjectEnvironmentElRegistryCredentialEl>>,
    dynamic: CodebuildProjectEnvironmentElDynamic,
}
impl CodebuildProjectEnvironmentEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `image_pull_credentials_type`.\n"]
    pub fn set_image_pull_credentials_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_pull_credentials_type = Some(v.into());
        self
    }
    #[doc = "Set the field `privileged_mode`.\n"]
    pub fn set_privileged_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.privileged_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `docker_server`.\n"]
    pub fn set_docker_server(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectEnvironmentElDockerServerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.docker_server = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.docker_server = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `environment_variable`.\n"]
    pub fn set_environment_variable(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectEnvironmentElEnvironmentVariableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.environment_variable = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.environment_variable = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fleet`.\n"]
    pub fn set_fleet(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectEnvironmentElFleetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fleet = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fleet = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `registry_credential`.\n"]
    pub fn set_registry_credential(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectEnvironmentElRegistryCredentialEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.registry_credential = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.registry_credential = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CodebuildProjectEnvironmentEl {
    type O = BlockAssignable<CodebuildProjectEnvironmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectEnvironmentEl {
    #[doc = ""]
    pub compute_type: PrimField<String>,
    #[doc = ""]
    pub image: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildCodebuildProjectEnvironmentEl {
    pub fn build(self) -> CodebuildProjectEnvironmentEl {
        CodebuildProjectEnvironmentEl {
            certificate: core::default::Default::default(),
            compute_type: self.compute_type,
            image: self.image,
            image_pull_credentials_type: core::default::Default::default(),
            privileged_mode: core::default::Default::default(),
            type_: self.type_,
            docker_server: core::default::Default::default(),
            environment_variable: core::default::Default::default(),
            fleet: core::default::Default::default(),
            registry_credential: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CodebuildProjectEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectEnvironmentElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectEnvironmentElRef {
        CodebuildProjectEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }
    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_type", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `image_pull_credentials_type` after provisioning.\n"]
    pub fn image_pull_credentials_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_pull_credentials_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `privileged_mode` after provisioning.\n"]
    pub fn privileged_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.privileged_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `docker_server` after provisioning.\n"]
    pub fn docker_server(&self) -> ListRef<CodebuildProjectEnvironmentElDockerServerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.docker_server", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment_variable` after provisioning.\n"]
    pub fn environment_variable(
        &self,
    ) -> ListRef<CodebuildProjectEnvironmentElEnvironmentVariableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment_variable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> ListRef<CodebuildProjectEnvironmentElFleetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fleet", self.base))
    }
    #[doc = "Get a reference to the value of field `registry_credential` after provisioning.\n"]
    pub fn registry_credential(
        &self,
    ) -> ListRef<CodebuildProjectEnvironmentElRegistryCredentialElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.registry_credential", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectFileSystemLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_point: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl CodebuildProjectFileSystemLocationsEl {
    #[doc = "Set the field `identifier`.\n"]
    pub fn set_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `mount_options`.\n"]
    pub fn set_mount_options(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_options = Some(v.into());
        self
    }
    #[doc = "Set the field `mount_point`.\n"]
    pub fn set_mount_point(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_point = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectFileSystemLocationsEl {
    type O = BlockAssignable<CodebuildProjectFileSystemLocationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectFileSystemLocationsEl {}
impl BuildCodebuildProjectFileSystemLocationsEl {
    pub fn build(self) -> CodebuildProjectFileSystemLocationsEl {
        CodebuildProjectFileSystemLocationsEl {
            identifier: core::default::Default::default(),
            location: core::default::Default::default(),
            mount_options: core::default::Default::default(),
            mount_point: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CodebuildProjectFileSystemLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectFileSystemLocationsElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectFileSystemLocationsElRef {
        CodebuildProjectFileSystemLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectFileSystemLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\n"]
    pub fn mount_options(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mount_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mount_point` after provisioning.\n"]
    pub fn mount_point(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_point", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectLogsConfigElCloudwatchLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_name: Option<PrimField<String>>,
}
impl CodebuildProjectLogsConfigElCloudwatchLogsEl {
    #[doc = "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_name = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `stream_name`.\n"]
    pub fn set_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_name = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectLogsConfigElCloudwatchLogsEl {
    type O = BlockAssignable<CodebuildProjectLogsConfigElCloudwatchLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectLogsConfigElCloudwatchLogsEl {}
impl BuildCodebuildProjectLogsConfigElCloudwatchLogsEl {
    pub fn build(self) -> CodebuildProjectLogsConfigElCloudwatchLogsEl {
        CodebuildProjectLogsConfigElCloudwatchLogsEl {
            group_name: core::default::Default::default(),
            status: core::default::Default::default(),
            stream_name: core::default::Default::default(),
        }
    }
}
pub struct CodebuildProjectLogsConfigElCloudwatchLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectLogsConfigElCloudwatchLogsElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectLogsConfigElCloudwatchLogsElRef {
        CodebuildProjectLogsConfigElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectLogsConfigElCloudwatchLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectLogsConfigElS3LogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_owner_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl CodebuildProjectLogsConfigElS3LogsEl {
    #[doc = "Set the field `bucket_owner_access`.\n"]
    pub fn set_bucket_owner_access(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_owner_access = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_disabled`.\n"]
    pub fn set_encryption_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encryption_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectLogsConfigElS3LogsEl {
    type O = BlockAssignable<CodebuildProjectLogsConfigElS3LogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectLogsConfigElS3LogsEl {}
impl BuildCodebuildProjectLogsConfigElS3LogsEl {
    pub fn build(self) -> CodebuildProjectLogsConfigElS3LogsEl {
        CodebuildProjectLogsConfigElS3LogsEl {
            bucket_owner_access: core::default::Default::default(),
            encryption_disabled: core::default::Default::default(),
            location: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct CodebuildProjectLogsConfigElS3LogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectLogsConfigElS3LogsElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectLogsConfigElS3LogsElRef {
        CodebuildProjectLogsConfigElS3LogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectLogsConfigElS3LogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_owner_access` after provisioning.\n"]
    pub fn bucket_owner_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_owner_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_disabled` after provisioning.\n"]
    pub fn encryption_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_disabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize, Default)]
struct CodebuildProjectLogsConfigElDynamic {
    cloudwatch_logs: Option<DynamicBlock<CodebuildProjectLogsConfigElCloudwatchLogsEl>>,
    s3_logs: Option<DynamicBlock<CodebuildProjectLogsConfigElS3LogsEl>>,
}
#[derive(Serialize)]
pub struct CodebuildProjectLogsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs: Option<Vec<CodebuildProjectLogsConfigElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_logs: Option<Vec<CodebuildProjectLogsConfigElS3LogsEl>>,
    dynamic: CodebuildProjectLogsConfigElDynamic,
}
impl CodebuildProjectLogsConfigEl {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectLogsConfigElCloudwatchLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_logs`.\n"]
    pub fn set_s3_logs(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectLogsConfigElS3LogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_logs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CodebuildProjectLogsConfigEl {
    type O = BlockAssignable<CodebuildProjectLogsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectLogsConfigEl {}
impl BuildCodebuildProjectLogsConfigEl {
    pub fn build(self) -> CodebuildProjectLogsConfigEl {
        CodebuildProjectLogsConfigEl {
            cloudwatch_logs: core::default::Default::default(),
            s3_logs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CodebuildProjectLogsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectLogsConfigElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectLogsConfigElRef {
        CodebuildProjectLogsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectLogsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> ListRef<CodebuildProjectLogsConfigElCloudwatchLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_logs` after provisioning.\n"]
    pub fn s3_logs(&self) -> ListRef<CodebuildProjectLogsConfigElS3LogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_logs", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectSecondaryArtifactsEl {
    artifact_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_owner_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_artifact_name: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packaging: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl CodebuildProjectSecondaryArtifactsEl {
    #[doc = "Set the field `bucket_owner_access`.\n"]
    pub fn set_bucket_owner_access(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_owner_access = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_disabled`.\n"]
    pub fn set_encryption_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encryption_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace_type`.\n"]
    pub fn set_namespace_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace_type = Some(v.into());
        self
    }
    #[doc = "Set the field `override_artifact_name`.\n"]
    pub fn set_override_artifact_name(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_artifact_name = Some(v.into());
        self
    }
    #[doc = "Set the field `packaging`.\n"]
    pub fn set_packaging(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.packaging = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectSecondaryArtifactsEl {
    type O = BlockAssignable<CodebuildProjectSecondaryArtifactsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectSecondaryArtifactsEl {
    #[doc = ""]
    pub artifact_identifier: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildCodebuildProjectSecondaryArtifactsEl {
    pub fn build(self) -> CodebuildProjectSecondaryArtifactsEl {
        CodebuildProjectSecondaryArtifactsEl {
            artifact_identifier: self.artifact_identifier,
            bucket_owner_access: core::default::Default::default(),
            encryption_disabled: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
            namespace_type: core::default::Default::default(),
            override_artifact_name: core::default::Default::default(),
            packaging: core::default::Default::default(),
            path: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct CodebuildProjectSecondaryArtifactsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectSecondaryArtifactsElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectSecondaryArtifactsElRef {
        CodebuildProjectSecondaryArtifactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectSecondaryArtifactsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `artifact_identifier` after provisioning.\n"]
    pub fn artifact_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.artifact_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bucket_owner_access` after provisioning.\n"]
    pub fn bucket_owner_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_owner_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_disabled` after provisioning.\n"]
    pub fn encryption_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_disabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace_type` after provisioning.\n"]
    pub fn namespace_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `override_artifact_name` after provisioning.\n"]
    pub fn override_artifact_name(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_artifact_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `packaging` after provisioning.\n"]
    pub fn packaging(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.packaging", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectSecondarySourceVersionEl {
    source_identifier: PrimField<String>,
    source_version: PrimField<String>,
}
impl CodebuildProjectSecondarySourceVersionEl {}
impl ToListMappable for CodebuildProjectSecondarySourceVersionEl {
    type O = BlockAssignable<CodebuildProjectSecondarySourceVersionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectSecondarySourceVersionEl {
    #[doc = ""]
    pub source_identifier: PrimField<String>,
    #[doc = ""]
    pub source_version: PrimField<String>,
}
impl BuildCodebuildProjectSecondarySourceVersionEl {
    pub fn build(self) -> CodebuildProjectSecondarySourceVersionEl {
        CodebuildProjectSecondarySourceVersionEl {
            source_identifier: self.source_identifier,
            source_version: self.source_version,
        }
    }
}
pub struct CodebuildProjectSecondarySourceVersionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectSecondarySourceVersionElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectSecondarySourceVersionElRef {
        CodebuildProjectSecondarySourceVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectSecondarySourceVersionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_identifier` after provisioning.\n"]
    pub fn source_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectSecondarySourcesElAuthEl {
    resource: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl CodebuildProjectSecondarySourcesElAuthEl {}
impl ToListMappable for CodebuildProjectSecondarySourcesElAuthEl {
    type O = BlockAssignable<CodebuildProjectSecondarySourcesElAuthEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectSecondarySourcesElAuthEl {
    #[doc = ""]
    pub resource: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildCodebuildProjectSecondarySourcesElAuthEl {
    pub fn build(self) -> CodebuildProjectSecondarySourcesElAuthEl {
        CodebuildProjectSecondarySourcesElAuthEl {
            resource: self.resource,
            type_: self.type_,
        }
    }
}
pub struct CodebuildProjectSecondarySourcesElAuthElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectSecondarySourcesElAuthElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectSecondarySourcesElAuthElRef {
        CodebuildProjectSecondarySourcesElAuthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectSecondarySourcesElAuthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectSecondarySourcesElBuildStatusConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_url: Option<PrimField<String>>,
}
impl CodebuildProjectSecondarySourcesElBuildStatusConfigEl {
    #[doc = "Set the field `context`.\n"]
    pub fn set_context(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.context = Some(v.into());
        self
    }
    #[doc = "Set the field `target_url`.\n"]
    pub fn set_target_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_url = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectSecondarySourcesElBuildStatusConfigEl {
    type O = BlockAssignable<CodebuildProjectSecondarySourcesElBuildStatusConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectSecondarySourcesElBuildStatusConfigEl {}
impl BuildCodebuildProjectSecondarySourcesElBuildStatusConfigEl {
    pub fn build(self) -> CodebuildProjectSecondarySourcesElBuildStatusConfigEl {
        CodebuildProjectSecondarySourcesElBuildStatusConfigEl {
            context: core::default::Default::default(),
            target_url: core::default::Default::default(),
        }
    }
}
pub struct CodebuildProjectSecondarySourcesElBuildStatusConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectSecondarySourcesElBuildStatusConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodebuildProjectSecondarySourcesElBuildStatusConfigElRef {
        CodebuildProjectSecondarySourcesElBuildStatusConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectSecondarySourcesElBuildStatusConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.context", self.base))
    }
    #[doc = "Get a reference to the value of field `target_url` after provisioning.\n"]
    pub fn target_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_url", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {
    fetch_submodules: PrimField<bool>,
}
impl CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {}
impl ToListMappable for CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {
    type O = BlockAssignable<CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {
    #[doc = ""]
    pub fetch_submodules: PrimField<bool>,
}
impl BuildCodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {
    pub fn build(self) -> CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {
        CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {
            fetch_submodules: self.fetch_submodules,
        }
    }
}
pub struct CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef {
        CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fetch_submodules` after provisioning.\n"]
    pub fn fetch_submodules(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fetch_submodules", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CodebuildProjectSecondarySourcesElDynamic {
    auth: Option<DynamicBlock<CodebuildProjectSecondarySourcesElAuthEl>>,
    build_status_config:
        Option<DynamicBlock<CodebuildProjectSecondarySourcesElBuildStatusConfigEl>>,
    git_submodules_config:
        Option<DynamicBlock<CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl>>,
}
#[derive(Serialize)]
pub struct CodebuildProjectSecondarySourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    buildspec: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_clone_depth: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_ssl: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    report_build_status: Option<PrimField<bool>>,
    source_identifier: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth: Option<Vec<CodebuildProjectSecondarySourcesElAuthEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_status_config: Option<Vec<CodebuildProjectSecondarySourcesElBuildStatusConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_submodules_config: Option<Vec<CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl>>,
    dynamic: CodebuildProjectSecondarySourcesElDynamic,
}
impl CodebuildProjectSecondarySourcesEl {
    #[doc = "Set the field `buildspec`.\n"]
    pub fn set_buildspec(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.buildspec = Some(v.into());
        self
    }
    #[doc = "Set the field `git_clone_depth`.\n"]
    pub fn set_git_clone_depth(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.git_clone_depth = Some(v.into());
        self
    }
    #[doc = "Set the field `insecure_ssl`.\n"]
    pub fn set_insecure_ssl(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.insecure_ssl = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `report_build_status`.\n"]
    pub fn set_report_build_status(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.report_build_status = Some(v.into());
        self
    }
    #[doc = "Set the field `auth`.\n"]
    pub fn set_auth(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectSecondarySourcesElAuthEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auth = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auth = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `build_status_config`.\n"]
    pub fn set_build_status_config(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectSecondarySourcesElBuildStatusConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.build_status_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.build_status_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `git_submodules_config`.\n"]
    pub fn set_git_submodules_config(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.git_submodules_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.git_submodules_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CodebuildProjectSecondarySourcesEl {
    type O = BlockAssignable<CodebuildProjectSecondarySourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectSecondarySourcesEl {
    #[doc = ""]
    pub source_identifier: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildCodebuildProjectSecondarySourcesEl {
    pub fn build(self) -> CodebuildProjectSecondarySourcesEl {
        CodebuildProjectSecondarySourcesEl {
            buildspec: core::default::Default::default(),
            git_clone_depth: core::default::Default::default(),
            insecure_ssl: core::default::Default::default(),
            location: core::default::Default::default(),
            report_build_status: core::default::Default::default(),
            source_identifier: self.source_identifier,
            type_: self.type_,
            auth: core::default::Default::default(),
            build_status_config: core::default::Default::default(),
            git_submodules_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CodebuildProjectSecondarySourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectSecondarySourcesElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectSecondarySourcesElRef {
        CodebuildProjectSecondarySourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectSecondarySourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `buildspec` after provisioning.\n"]
    pub fn buildspec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.buildspec", self.base))
    }
    #[doc = "Get a reference to the value of field `git_clone_depth` after provisioning.\n"]
    pub fn git_clone_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.git_clone_depth", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `insecure_ssl` after provisioning.\n"]
    pub fn insecure_ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.insecure_ssl", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `report_build_status` after provisioning.\n"]
    pub fn report_build_status(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.report_build_status", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_identifier` after provisioning.\n"]
    pub fn source_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `auth` after provisioning.\n"]
    pub fn auth(&self) -> ListRef<CodebuildProjectSecondarySourcesElAuthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auth", self.base))
    }
    #[doc = "Get a reference to the value of field `build_status_config` after provisioning.\n"]
    pub fn build_status_config(
        &self,
    ) -> ListRef<CodebuildProjectSecondarySourcesElBuildStatusConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_status_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `git_submodules_config` after provisioning.\n"]
    pub fn git_submodules_config(
        &self,
    ) -> ListRef<CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.git_submodules_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectSourceElAuthEl {
    resource: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl CodebuildProjectSourceElAuthEl {}
impl ToListMappable for CodebuildProjectSourceElAuthEl {
    type O = BlockAssignable<CodebuildProjectSourceElAuthEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectSourceElAuthEl {
    #[doc = ""]
    pub resource: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildCodebuildProjectSourceElAuthEl {
    pub fn build(self) -> CodebuildProjectSourceElAuthEl {
        CodebuildProjectSourceElAuthEl {
            resource: self.resource,
            type_: self.type_,
        }
    }
}
pub struct CodebuildProjectSourceElAuthElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectSourceElAuthElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectSourceElAuthElRef {
        CodebuildProjectSourceElAuthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectSourceElAuthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectSourceElBuildStatusConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_url: Option<PrimField<String>>,
}
impl CodebuildProjectSourceElBuildStatusConfigEl {
    #[doc = "Set the field `context`.\n"]
    pub fn set_context(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.context = Some(v.into());
        self
    }
    #[doc = "Set the field `target_url`.\n"]
    pub fn set_target_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_url = Some(v.into());
        self
    }
}
impl ToListMappable for CodebuildProjectSourceElBuildStatusConfigEl {
    type O = BlockAssignable<CodebuildProjectSourceElBuildStatusConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectSourceElBuildStatusConfigEl {}
impl BuildCodebuildProjectSourceElBuildStatusConfigEl {
    pub fn build(self) -> CodebuildProjectSourceElBuildStatusConfigEl {
        CodebuildProjectSourceElBuildStatusConfigEl {
            context: core::default::Default::default(),
            target_url: core::default::Default::default(),
        }
    }
}
pub struct CodebuildProjectSourceElBuildStatusConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectSourceElBuildStatusConfigElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectSourceElBuildStatusConfigElRef {
        CodebuildProjectSourceElBuildStatusConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectSourceElBuildStatusConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.context", self.base))
    }
    #[doc = "Get a reference to the value of field `target_url` after provisioning.\n"]
    pub fn target_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_url", self.base))
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectSourceElGitSubmodulesConfigEl {
    fetch_submodules: PrimField<bool>,
}
impl CodebuildProjectSourceElGitSubmodulesConfigEl {}
impl ToListMappable for CodebuildProjectSourceElGitSubmodulesConfigEl {
    type O = BlockAssignable<CodebuildProjectSourceElGitSubmodulesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectSourceElGitSubmodulesConfigEl {
    #[doc = ""]
    pub fetch_submodules: PrimField<bool>,
}
impl BuildCodebuildProjectSourceElGitSubmodulesConfigEl {
    pub fn build(self) -> CodebuildProjectSourceElGitSubmodulesConfigEl {
        CodebuildProjectSourceElGitSubmodulesConfigEl {
            fetch_submodules: self.fetch_submodules,
        }
    }
}
pub struct CodebuildProjectSourceElGitSubmodulesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectSourceElGitSubmodulesConfigElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectSourceElGitSubmodulesConfigElRef {
        CodebuildProjectSourceElGitSubmodulesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectSourceElGitSubmodulesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fetch_submodules` after provisioning.\n"]
    pub fn fetch_submodules(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fetch_submodules", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CodebuildProjectSourceElDynamic {
    auth: Option<DynamicBlock<CodebuildProjectSourceElAuthEl>>,
    build_status_config: Option<DynamicBlock<CodebuildProjectSourceElBuildStatusConfigEl>>,
    git_submodules_config: Option<DynamicBlock<CodebuildProjectSourceElGitSubmodulesConfigEl>>,
}
#[derive(Serialize)]
pub struct CodebuildProjectSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    buildspec: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_clone_depth: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_ssl: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    report_build_status: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth: Option<Vec<CodebuildProjectSourceElAuthEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_status_config: Option<Vec<CodebuildProjectSourceElBuildStatusConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_submodules_config: Option<Vec<CodebuildProjectSourceElGitSubmodulesConfigEl>>,
    dynamic: CodebuildProjectSourceElDynamic,
}
impl CodebuildProjectSourceEl {
    #[doc = "Set the field `buildspec`.\n"]
    pub fn set_buildspec(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.buildspec = Some(v.into());
        self
    }
    #[doc = "Set the field `git_clone_depth`.\n"]
    pub fn set_git_clone_depth(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.git_clone_depth = Some(v.into());
        self
    }
    #[doc = "Set the field `insecure_ssl`.\n"]
    pub fn set_insecure_ssl(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.insecure_ssl = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `report_build_status`.\n"]
    pub fn set_report_build_status(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.report_build_status = Some(v.into());
        self
    }
    #[doc = "Set the field `auth`.\n"]
    pub fn set_auth(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectSourceElAuthEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auth = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auth = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `build_status_config`.\n"]
    pub fn set_build_status_config(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectSourceElBuildStatusConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.build_status_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.build_status_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `git_submodules_config`.\n"]
    pub fn set_git_submodules_config(
        mut self,
        v: impl Into<BlockAssignable<CodebuildProjectSourceElGitSubmodulesConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.git_submodules_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.git_submodules_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CodebuildProjectSourceEl {
    type O = BlockAssignable<CodebuildProjectSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectSourceEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildCodebuildProjectSourceEl {
    pub fn build(self) -> CodebuildProjectSourceEl {
        CodebuildProjectSourceEl {
            buildspec: core::default::Default::default(),
            git_clone_depth: core::default::Default::default(),
            insecure_ssl: core::default::Default::default(),
            location: core::default::Default::default(),
            report_build_status: core::default::Default::default(),
            type_: self.type_,
            auth: core::default::Default::default(),
            build_status_config: core::default::Default::default(),
            git_submodules_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CodebuildProjectSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectSourceElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectSourceElRef {
        CodebuildProjectSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `buildspec` after provisioning.\n"]
    pub fn buildspec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.buildspec", self.base))
    }
    #[doc = "Get a reference to the value of field `git_clone_depth` after provisioning.\n"]
    pub fn git_clone_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.git_clone_depth", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `insecure_ssl` after provisioning.\n"]
    pub fn insecure_ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.insecure_ssl", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `report_build_status` after provisioning.\n"]
    pub fn report_build_status(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.report_build_status", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `auth` after provisioning.\n"]
    pub fn auth(&self) -> ListRef<CodebuildProjectSourceElAuthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auth", self.base))
    }
    #[doc = "Get a reference to the value of field `build_status_config` after provisioning.\n"]
    pub fn build_status_config(&self) -> ListRef<CodebuildProjectSourceElBuildStatusConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build_status_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `git_submodules_config` after provisioning.\n"]
    pub fn git_submodules_config(
        &self,
    ) -> ListRef<CodebuildProjectSourceElGitSubmodulesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.git_submodules_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CodebuildProjectVpcConfigEl {
    security_group_ids: SetField<PrimField<String>>,
    subnets: SetField<PrimField<String>>,
    vpc_id: PrimField<String>,
}
impl CodebuildProjectVpcConfigEl {}
impl ToListMappable for CodebuildProjectVpcConfigEl {
    type O = BlockAssignable<CodebuildProjectVpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCodebuildProjectVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}
impl BuildCodebuildProjectVpcConfigEl {
    pub fn build(self) -> CodebuildProjectVpcConfigEl {
        CodebuildProjectVpcConfigEl {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
            vpc_id: self.vpc_id,
        }
    }
}
pub struct CodebuildProjectVpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CodebuildProjectVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> CodebuildProjectVpcConfigElRef {
        CodebuildProjectVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CodebuildProjectVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct CodebuildProjectDynamic {
    artifacts: Option<DynamicBlock<CodebuildProjectArtifactsEl>>,
    build_batch_config: Option<DynamicBlock<CodebuildProjectBuildBatchConfigEl>>,
    cache: Option<DynamicBlock<CodebuildProjectCacheEl>>,
    environment: Option<DynamicBlock<CodebuildProjectEnvironmentEl>>,
    file_system_locations: Option<DynamicBlock<CodebuildProjectFileSystemLocationsEl>>,
    logs_config: Option<DynamicBlock<CodebuildProjectLogsConfigEl>>,
    secondary_artifacts: Option<DynamicBlock<CodebuildProjectSecondaryArtifactsEl>>,
    secondary_source_version: Option<DynamicBlock<CodebuildProjectSecondarySourceVersionEl>>,
    secondary_sources: Option<DynamicBlock<CodebuildProjectSecondarySourcesEl>>,
    source: Option<DynamicBlock<CodebuildProjectSourceEl>>,
    vpc_config: Option<DynamicBlock<CodebuildProjectVpcConfigEl>>,
}
