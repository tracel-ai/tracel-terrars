use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudbuildTriggerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignored_files: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_build_logs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_files: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    substitutions: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_config: Option<Vec<CloudbuildTriggerApprovalConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket_server_trigger_config: Option<Vec<CloudbuildTriggerBitbucketServerTriggerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build: Option<Vec<CloudbuildTriggerBuildEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_connect_event_config: Option<Vec<CloudbuildTriggerDeveloperConnectEventConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_file_source: Option<Vec<CloudbuildTriggerGitFileSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<CloudbuildTriggerGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_config: Option<Vec<CloudbuildTriggerPubsubConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_event_config: Option<Vec<CloudbuildTriggerRepositoryEventConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_to_build: Option<Vec<CloudbuildTriggerSourceToBuildEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudbuildTriggerTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_template: Option<Vec<CloudbuildTriggerTriggerTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook_config: Option<Vec<CloudbuildTriggerWebhookConfigEl>>,
    dynamic: CloudbuildTriggerDynamic,
}
struct CloudbuildTrigger_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudbuildTriggerData>,
}
#[derive(Clone)]
pub struct CloudbuildTrigger(Rc<CloudbuildTrigger_>);
impl CloudbuildTrigger {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nHuman-readable description of the trigger."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the trigger is disabled or not. If true, the trigger will never result in a build."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `filename`.\nPath, from the source root, to a file whose contents is used for the template.\nEither a filename or build template must be provided. Set this only when using trigger_template or github.\nWhen using Pub/Sub, Webhook or Manual set the file name using git_file_source instead."]
    pub fn set_filename(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filename = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\nA Common Expression Language string. Used only with Pub/Sub and Webhook."]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ignored_files`.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf ignoredFiles and changed files are both empty, then they are not\nused to determine whether or not to trigger a build.\n\nIf ignoredFiles is not empty, then we ignore any files that match any\nof the ignored_file globs. If the change has no files that are outside\nof the ignoredFiles globs, then we do not trigger a build."]
    pub fn set_ignored_files(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ignored_files = Some(v.into());
        self
    }
    #[doc = "Set the field `include_build_logs`.\nBuild logs will be sent back to GitHub as part of the checkrun\nresult.  Values can be INCLUDE_BUILD_LOGS_UNSPECIFIED or\nINCLUDE_BUILD_LOGS_WITH_STATUS Possible values: [\"INCLUDE_BUILD_LOGS_UNSPECIFIED\", \"INCLUDE_BUILD_LOGS_WITH_STATUS\"]"]
    pub fn set_include_build_logs(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().include_build_logs = Some(v.into());
        self
    }
    #[doc = "Set the field `included_files`.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is empty, then as far as this filter is concerned, we\nshould trigger the build.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is not empty, then we make sure that at least one of\nthose files matches a includedFiles glob. If not, then we do not trigger\na build."]
    pub fn set_included_files(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().included_files = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.\nIf not specified, \"global\" is used."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nName of the trigger. Must be unique within the project."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn set_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `substitutions`.\nSubstitutions data for Build resource."]
    pub fn set_substitutions(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().substitutions = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nTags for annotation of a BuildTrigger"]
    pub fn set_tags(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `approval_config`.\n"]
    pub fn set_approval_config(
        self,
        v: impl Into<BlockAssignable<CloudbuildTriggerApprovalConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().approval_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.approval_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `bitbucket_server_trigger_config`.\n"]
    pub fn set_bitbucket_server_trigger_config(
        self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBitbucketServerTriggerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bitbucket_server_trigger_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .bitbucket_server_trigger_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `build`.\n"]
    pub fn set_build(self, v: impl Into<BlockAssignable<CloudbuildTriggerBuildEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().build = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.build = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `developer_connect_event_config`.\n"]
    pub fn set_developer_connect_event_config(
        self,
        v: impl Into<BlockAssignable<CloudbuildTriggerDeveloperConnectEventConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().developer_connect_event_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .developer_connect_event_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `git_file_source`.\n"]
    pub fn set_git_file_source(
        self,
        v: impl Into<BlockAssignable<CloudbuildTriggerGitFileSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().git_file_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.git_file_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `github`.\n"]
    pub fn set_github(self, v: impl Into<BlockAssignable<CloudbuildTriggerGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().github = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.github = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pubsub_config`.\n"]
    pub fn set_pubsub_config(
        self,
        v: impl Into<BlockAssignable<CloudbuildTriggerPubsubConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pubsub_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pubsub_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `repository_event_config`.\n"]
    pub fn set_repository_event_config(
        self,
        v: impl Into<BlockAssignable<CloudbuildTriggerRepositoryEventConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().repository_event_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.repository_event_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_to_build`.\n"]
    pub fn set_source_to_build(
        self,
        v: impl Into<BlockAssignable<CloudbuildTriggerSourceToBuildEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_to_build = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_to_build = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudbuildTriggerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger_template`.\n"]
    pub fn set_trigger_template(
        self,
        v: impl Into<BlockAssignable<CloudbuildTriggerTriggerTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger_template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger_template = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `webhook_config`.\n"]
    pub fn set_webhook_config(
        self,
        v: impl Into<BlockAssignable<CloudbuildTriggerWebhookConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().webhook_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.webhook_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when the trigger was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the trigger."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the trigger is disabled or not. If true, the trigger will never result in a build."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filename` after provisioning.\nPath, from the source root, to a file whose contents is used for the template.\nEither a filename or build template must be provided. Set this only when using trigger_template or github.\nWhen using Pub/Sub, Webhook or Manual set the file name using git_file_source instead."]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filename", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nA Common Expression Language string. Used only with Pub/Sub and Webhook."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignored_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf ignoredFiles and changed files are both empty, then they are not\nused to determine whether or not to trigger a build.\n\nIf ignoredFiles is not empty, then we ignore any files that match any\nof the ignored_file globs. If the change has no files that are outside\nof the ignoredFiles globs, then we do not trigger a build."]
    pub fn ignored_files(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ignored_files", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_build_logs` after provisioning.\nBuild logs will be sent back to GitHub as part of the checkrun\nresult.  Values can be INCLUDE_BUILD_LOGS_UNSPECIFIED or\nINCLUDE_BUILD_LOGS_WITH_STATUS Possible values: [\"INCLUDE_BUILD_LOGS_UNSPECIFIED\", \"INCLUDE_BUILD_LOGS_WITH_STATUS\"]"]
    pub fn include_build_logs(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_build_logs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `included_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is empty, then as far as this filter is concerned, we\nshould trigger the build.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is not empty, then we make sure that at least one of\nthose files matches a includedFiles glob. If not, then we do not trigger\na build."]
    pub fn included_files(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_files", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.\nIf not specified, \"global\" is used."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the trigger. Must be unique within the project."]
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
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `substitutions` after provisioning.\nSubstitutions data for Build resource."]
    pub fn substitutions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.substitutions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags for annotation of a BuildTrigger"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_id` after provisioning.\nThe unique identifier for the trigger."]
    pub fn trigger_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approval_config` after provisioning.\n"]
    pub fn approval_config(&self) -> ListRef<CloudbuildTriggerApprovalConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_server_trigger_config` after provisioning.\n"]
    pub fn bitbucket_server_trigger_config(
        &self,
    ) -> ListRef<CloudbuildTriggerBitbucketServerTriggerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_trigger_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build` after provisioning.\n"]
    pub fn build(&self) -> ListRef<CloudbuildTriggerBuildElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `developer_connect_event_config` after provisioning.\n"]
    pub fn developer_connect_event_config(
        &self,
    ) -> ListRef<CloudbuildTriggerDeveloperConnectEventConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.developer_connect_event_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `git_file_source` after provisioning.\n"]
    pub fn git_file_source(&self) -> ListRef<CloudbuildTriggerGitFileSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.git_file_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<CloudbuildTriggerGithubElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pubsub_config` after provisioning.\n"]
    pub fn pubsub_config(&self) -> ListRef<CloudbuildTriggerPubsubConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_event_config` after provisioning.\n"]
    pub fn repository_event_config(&self) -> ListRef<CloudbuildTriggerRepositoryEventConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repository_event_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_to_build` after provisioning.\n"]
    pub fn source_to_build(&self) -> ListRef<CloudbuildTriggerSourceToBuildElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_to_build", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudbuildTriggerTimeoutsElRef {
        CloudbuildTriggerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_template` after provisioning.\n"]
    pub fn trigger_template(&self) -> ListRef<CloudbuildTriggerTriggerTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_config` after provisioning.\n"]
    pub fn webhook_config(&self) -> ListRef<CloudbuildTriggerWebhookConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.webhook_config", self.extract_ref()),
        )
    }
}
impl Referable for CloudbuildTrigger {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudbuildTrigger {}
impl ToListMappable for CloudbuildTrigger {
    type O = ListRef<CloudbuildTriggerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudbuildTrigger_ {
    fn extract_resource_type(&self) -> String {
        "google_cloudbuild_trigger".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudbuildTrigger {
    pub tf_id: String,
}
impl BuildCloudbuildTrigger {
    pub fn build(self, stack: &mut Stack) -> CloudbuildTrigger {
        let out = CloudbuildTrigger(Rc::new(CloudbuildTrigger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudbuildTriggerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                filename: core::default::Default::default(),
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                ignored_files: core::default::Default::default(),
                include_build_logs: core::default::Default::default(),
                included_files: core::default::Default::default(),
                location: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                service_account: core::default::Default::default(),
                substitutions: core::default::Default::default(),
                tags: core::default::Default::default(),
                approval_config: core::default::Default::default(),
                bitbucket_server_trigger_config: core::default::Default::default(),
                build: core::default::Default::default(),
                developer_connect_event_config: core::default::Default::default(),
                git_file_source: core::default::Default::default(),
                github: core::default::Default::default(),
                pubsub_config: core::default::Default::default(),
                repository_event_config: core::default::Default::default(),
                source_to_build: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                trigger_template: core::default::Default::default(),
                webhook_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudbuildTriggerRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudbuildTriggerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when the trigger was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the trigger."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the trigger is disabled or not. If true, the trigger will never result in a build."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filename` after provisioning.\nPath, from the source root, to a file whose contents is used for the template.\nEither a filename or build template must be provided. Set this only when using trigger_template or github.\nWhen using Pub/Sub, Webhook or Manual set the file name using git_file_source instead."]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filename", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nA Common Expression Language string. Used only with Pub/Sub and Webhook."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignored_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf ignoredFiles and changed files are both empty, then they are not\nused to determine whether or not to trigger a build.\n\nIf ignoredFiles is not empty, then we ignore any files that match any\nof the ignored_file globs. If the change has no files that are outside\nof the ignoredFiles globs, then we do not trigger a build."]
    pub fn ignored_files(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ignored_files", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_build_logs` after provisioning.\nBuild logs will be sent back to GitHub as part of the checkrun\nresult.  Values can be INCLUDE_BUILD_LOGS_UNSPECIFIED or\nINCLUDE_BUILD_LOGS_WITH_STATUS Possible values: [\"INCLUDE_BUILD_LOGS_UNSPECIFIED\", \"INCLUDE_BUILD_LOGS_WITH_STATUS\"]"]
    pub fn include_build_logs(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_build_logs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `included_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is empty, then as far as this filter is concerned, we\nshould trigger the build.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is not empty, then we make sure that at least one of\nthose files matches a includedFiles glob. If not, then we do not trigger\na build."]
    pub fn included_files(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_files", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.\nIf not specified, \"global\" is used."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the trigger. Must be unique within the project."]
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
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `substitutions` after provisioning.\nSubstitutions data for Build resource."]
    pub fn substitutions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.substitutions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags for annotation of a BuildTrigger"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_id` after provisioning.\nThe unique identifier for the trigger."]
    pub fn trigger_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approval_config` after provisioning.\n"]
    pub fn approval_config(&self) -> ListRef<CloudbuildTriggerApprovalConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_server_trigger_config` after provisioning.\n"]
    pub fn bitbucket_server_trigger_config(
        &self,
    ) -> ListRef<CloudbuildTriggerBitbucketServerTriggerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_trigger_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build` after provisioning.\n"]
    pub fn build(&self) -> ListRef<CloudbuildTriggerBuildElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `developer_connect_event_config` after provisioning.\n"]
    pub fn developer_connect_event_config(
        &self,
    ) -> ListRef<CloudbuildTriggerDeveloperConnectEventConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.developer_connect_event_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `git_file_source` after provisioning.\n"]
    pub fn git_file_source(&self) -> ListRef<CloudbuildTriggerGitFileSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.git_file_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<CloudbuildTriggerGithubElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pubsub_config` after provisioning.\n"]
    pub fn pubsub_config(&self) -> ListRef<CloudbuildTriggerPubsubConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_event_config` after provisioning.\n"]
    pub fn repository_event_config(&self) -> ListRef<CloudbuildTriggerRepositoryEventConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repository_event_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_to_build` after provisioning.\n"]
    pub fn source_to_build(&self) -> ListRef<CloudbuildTriggerSourceToBuildElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_to_build", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudbuildTriggerTimeoutsElRef {
        CloudbuildTriggerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_template` after provisioning.\n"]
    pub fn trigger_template(&self) -> ListRef<CloudbuildTriggerTriggerTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_config` after provisioning.\n"]
    pub fn webhook_config(&self) -> ListRef<CloudbuildTriggerWebhookConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.webhook_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerApprovalConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_required: Option<PrimField<bool>>,
}
impl CloudbuildTriggerApprovalConfigEl {
    #[doc = "Set the field `approval_required`.\nWhether or not approval is needed. If this is set on a build, it will become pending when run,\nand will need to be explicitly approved to start."]
    pub fn set_approval_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.approval_required = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerApprovalConfigEl {
    type O = BlockAssignable<CloudbuildTriggerApprovalConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerApprovalConfigEl {}
impl BuildCloudbuildTriggerApprovalConfigEl {
    pub fn build(self) -> CloudbuildTriggerApprovalConfigEl {
        CloudbuildTriggerApprovalConfigEl {
            approval_required: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerApprovalConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerApprovalConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerApprovalConfigElRef {
        CloudbuildTriggerApprovalConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerApprovalConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `approval_required` after provisioning.\nWhether or not approval is needed. If this is set on a build, it will become pending when run,\nand will need to be explicitly approved to start."]
    pub fn approval_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approval_required", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    branch: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
}
impl CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    #[doc = "Set the field `comment_control`.\nConfigure builds to run whether a repository owner or collaborator need to comment /gcbrun. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn set_comment_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment_control = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    type O = BlockAssignable<CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    #[doc = "Regex of branches to match.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub branch: PrimField<String>,
}
impl BuildCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    pub fn build(self) -> CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
        CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
            branch: self.branch,
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
        CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `comment_control` after provisioning.\nConfigure builds to run whether a repository owner or collaborator need to comment /gcbrun. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn comment_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl CloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
    #[doc = "Set the field `branch`.\nRegex of branches to match.  Specify only one of branch or tag."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\nWhen true, only trigger a build if the revision regex does NOT match the gitRef regex."]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nRegex of tags to match.  Specify only one of branch or tag."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
    type O = BlockAssignable<CloudbuildTriggerBitbucketServerTriggerConfigElPushEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {}
impl BuildCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
    pub fn build(self) -> CloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
        CloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
        CloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match.  Specify only one of branch or tag."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\nWhen true, only trigger a build if the revision regex does NOT match the gitRef regex."]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nRegex of tags to match.  Specify only one of branch or tag."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudbuildTriggerBitbucketServerTriggerConfigElDynamic {
    pull_request:
        Option<DynamicBlock<CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl>>,
    push: Option<DynamicBlock<CloudbuildTriggerBitbucketServerTriggerConfigElPushEl>>,
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBitbucketServerTriggerConfigEl {
    bitbucket_server_config_resource: PrimField<String>,
    project_key: PrimField<String>,
    repo_slug: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<Vec<CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<Vec<CloudbuildTriggerBitbucketServerTriggerConfigElPushEl>>,
    dynamic: CloudbuildTriggerBitbucketServerTriggerConfigElDynamic,
}
impl CloudbuildTriggerBitbucketServerTriggerConfigEl {
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pull_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pull_request = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBitbucketServerTriggerConfigElPushEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.push = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.push = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudbuildTriggerBitbucketServerTriggerConfigEl {
    type O = BlockAssignable<CloudbuildTriggerBitbucketServerTriggerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBitbucketServerTriggerConfigEl {
    #[doc = "The Bitbucket server config resource that this trigger config maps to."]
    pub bitbucket_server_config_resource: PrimField<String>,
    #[doc = "Key of the project that the repo is in. For example: The key for https://mybitbucket.server/projects/TEST/repos/test-repo is \"TEST\"."]
    pub project_key: PrimField<String>,
    #[doc = "Slug of the repository. A repository slug is a URL-friendly version of a repository name, automatically generated by Bitbucket for use in the URL.\nFor example, if the repository name is 'test repo', in the URL it would become 'test-repo' as in https://mybitbucket.server/projects/TEST/repos/test-repo."]
    pub repo_slug: PrimField<String>,
}
impl BuildCloudbuildTriggerBitbucketServerTriggerConfigEl {
    pub fn build(self) -> CloudbuildTriggerBitbucketServerTriggerConfigEl {
        CloudbuildTriggerBitbucketServerTriggerConfigEl {
            bitbucket_server_config_resource: self.bitbucket_server_config_resource,
            project_key: self.project_key,
            repo_slug: self.repo_slug,
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBitbucketServerTriggerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBitbucketServerTriggerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerBitbucketServerTriggerConfigElRef {
        CloudbuildTriggerBitbucketServerTriggerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBitbucketServerTriggerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitbucket_server_config_resource` after provisioning.\nThe Bitbucket server config resource that this trigger config maps to."]
    pub fn bitbucket_server_config_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_config_resource", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project_key` after provisioning.\nKey of the project that the repo is in. For example: The key for https://mybitbucket.server/projects/TEST/repos/test-repo is \"TEST\"."]
    pub fn project_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_key", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_slug` after provisioning.\nSlug of the repository. A repository slug is a URL-friendly version of a repository name, automatically generated by Bitbucket for use in the URL.\nFor example, if the repository name is 'test repo', in the URL it would become 'test-repo' as in https://mybitbucket.server/projects/TEST/repos/test-repo."]
    pub fn repo_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_slug", self.base))
    }
    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(
        &self,
    ) -> ListRef<CloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }
    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> ListRef<CloudbuildTriggerBitbucketServerTriggerConfigElPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl CloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
    #[doc = "Set the field `artifact_id`.\nMaven artifactId value used when uploading the artifact to Artifact Registry."]
    pub fn set_artifact_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.artifact_id = Some(v.into());
        self
    }
    #[doc = "Set the field `group_id`.\nMaven groupId value used when uploading the artifact to Artifact Registry."]
    pub fn set_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nPath to an artifact in the build's workspace to be uploaded to Artifact Registry. This can be either an absolute path, e.g. /workspace/my-app/target/my-app-1.0.SNAPSHOT.jar or a relative path from /workspace, e.g. my-app/target/my-app-1.0.SNAPSHOT.jar."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\nArtifact Registry repository, in the form \"https://$REGION-maven.pkg.dev/$PROJECT/$REPOSITORY\"\n\nArtifact in the workspace specified by path will be uploaded to Artifact Registry with this location as a prefix."]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nMaven version value used when uploading the artifact to Artifact Registry."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElArtifactsElMavenArtifactsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {}
impl BuildCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
    pub fn build(self) -> CloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
        CloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
            artifact_id: core::default::Default::default(),
            group_id: core::default::Default::default(),
            path: core::default::Default::default(),
            repository: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
        CloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `artifact_id` after provisioning.\nMaven artifactId value used when uploading the artifact to Artifact Registry."]
    pub fn artifact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.artifact_id", self.base))
    }
    #[doc = "Get a reference to the value of field `group_id` after provisioning.\nMaven groupId value used when uploading the artifact to Artifact Registry."]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to an artifact in the build's workspace to be uploaded to Artifact Registry. This can be either an absolute path, e.g. /workspace/my-app/target/my-app-1.0.SNAPSHOT.jar or a relative path from /workspace, e.g. my-app/target/my-app-1.0.SNAPSHOT.jar."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nArtifact Registry repository, in the form \"https://$REGION-maven.pkg.dev/$PROJECT/$REPOSITORY\"\n\nArtifact in the workspace specified by path will be uploaded to Artifact Registry with this location as a prefix."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nMaven version value used when uploading the artifact to Artifact Registry."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    package_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
}
impl CloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
    #[doc = "Set the field `package_path`.\nPath to the package.json. e.g. workspace/path/to/package"]
    pub fn set_package_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.package_path = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\nArtifact Registry repository, in the form \"https://$REGION-npm.pkg.dev/$PROJECT/$REPOSITORY\"\n\nNpm package in the workspace specified by path will be zipped and uploaded to Artifact Registry with this location as a prefix."]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElArtifactsElNpmPackagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {}
impl BuildCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
    pub fn build(self) -> CloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
        CloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
            package_path: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
        CloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `package_path` after provisioning.\nPath to the package.json. e.g. workspace/path/to/package"]
    pub fn package_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.package_path", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nArtifact Registry repository, in the form \"https://$REGION-npm.pkg.dev/$PROJECT/$REPOSITORY\"\n\nNpm package in the workspace specified by path will be zipped and uploaded to Artifact Registry with this location as a prefix."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl CloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElArtifactsElObjectsElTimingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {}
impl BuildCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
    pub fn build(self) -> CloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
        CloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
        CloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElArtifactsElObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paths: Option<ListField<PrimField<String>>>,
}
impl CloudbuildTriggerBuildElArtifactsElObjectsEl {
    #[doc = "Set the field `location`.\nCloud Storage bucket and optional object path, in the form \"gs://bucket/path/to/somewhere/\".\n\nFiles in the workspace matching any path pattern will be uploaded to Cloud Storage with\nthis location as a prefix."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `paths`.\nPath globs used to match files in the build's workspace."]
    pub fn set_paths(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.paths = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElArtifactsElObjectsEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElArtifactsElObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElArtifactsElObjectsEl {}
impl BuildCloudbuildTriggerBuildElArtifactsElObjectsEl {
    pub fn build(self) -> CloudbuildTriggerBuildElArtifactsElObjectsEl {
        CloudbuildTriggerBuildElArtifactsElObjectsEl {
            location: core::default::Default::default(),
            paths: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElArtifactsElObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElArtifactsElObjectsElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerBuildElArtifactsElObjectsElRef {
        CloudbuildTriggerBuildElArtifactsElObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElArtifactsElObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nCloud Storage bucket and optional object path, in the form \"gs://bucket/path/to/somewhere/\".\n\nFiles in the workspace matching any path pattern will be uploaded to Cloud Storage with\nthis location as a prefix."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `paths` after provisioning.\nPath globs used to match files in the build's workspace."]
    pub fn paths(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
    #[doc = "Get a reference to the value of field `timing` after provisioning.\nOutput only. Stores timing information for pushing all artifact objects."]
    pub fn timing(&self) -> ListRef<CloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timing", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    paths: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
}
impl CloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
    #[doc = "Set the field `paths`.\nPath globs used to match files in the build's workspace. For Python/ Twine, this is usually dist/*, and sometimes additionally an .asc file."]
    pub fn set_paths(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.paths = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\nArtifact Registry repository, in the form \"https://$REGION-python.pkg.dev/$PROJECT/$REPOSITORY\"\n\nFiles in the workspace matching any path pattern will be uploaded to Artifact Registry with this location as a prefix."]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElArtifactsElPythonPackagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {}
impl BuildCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
    pub fn build(self) -> CloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
        CloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
            paths: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
        CloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `paths` after provisioning.\nPath globs used to match files in the build's workspace. For Python/ Twine, this is usually dist/*, and sometimes additionally an .asc file."]
    pub fn paths(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nArtifact Registry repository, in the form \"https://$REGION-python.pkg.dev/$PROJECT/$REPOSITORY\"\n\nFiles in the workspace matching any path pattern will be uploaded to Artifact Registry with this location as a prefix."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudbuildTriggerBuildElArtifactsElDynamic {
    maven_artifacts: Option<DynamicBlock<CloudbuildTriggerBuildElArtifactsElMavenArtifactsEl>>,
    npm_packages: Option<DynamicBlock<CloudbuildTriggerBuildElArtifactsElNpmPackagesEl>>,
    objects: Option<DynamicBlock<CloudbuildTriggerBuildElArtifactsElObjectsEl>>,
    python_packages: Option<DynamicBlock<CloudbuildTriggerBuildElArtifactsElPythonPackagesEl>>,
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElArtifactsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maven_artifacts: Option<Vec<CloudbuildTriggerBuildElArtifactsElMavenArtifactsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    npm_packages: Option<Vec<CloudbuildTriggerBuildElArtifactsElNpmPackagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    objects: Option<Vec<CloudbuildTriggerBuildElArtifactsElObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_packages: Option<Vec<CloudbuildTriggerBuildElArtifactsElPythonPackagesEl>>,
    dynamic: CloudbuildTriggerBuildElArtifactsElDynamic,
}
impl CloudbuildTriggerBuildElArtifactsEl {
    #[doc = "Set the field `images`.\nA list of images to be pushed upon the successful completion of all build steps.\n\nThe images will be pushed using the builder service account's credentials.\n\nThe digests of the pushed images will be stored in the Build resource's results field.\n\nIf any of the images fail to be pushed, the build is marked FAILURE."]
    pub fn set_images(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.images = Some(v.into());
        self
    }
    #[doc = "Set the field `maven_artifacts`.\n"]
    pub fn set_maven_artifacts(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElArtifactsElMavenArtifactsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.maven_artifacts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.maven_artifacts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `npm_packages`.\n"]
    pub fn set_npm_packages(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElArtifactsElNpmPackagesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.npm_packages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.npm_packages = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `objects`.\n"]
    pub fn set_objects(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElArtifactsElObjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.objects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.objects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `python_packages`.\n"]
    pub fn set_python_packages(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElArtifactsElPythonPackagesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.python_packages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.python_packages = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElArtifactsEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElArtifactsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElArtifactsEl {}
impl BuildCloudbuildTriggerBuildElArtifactsEl {
    pub fn build(self) -> CloudbuildTriggerBuildElArtifactsEl {
        CloudbuildTriggerBuildElArtifactsEl {
            images: core::default::Default::default(),
            maven_artifacts: core::default::Default::default(),
            npm_packages: core::default::Default::default(),
            objects: core::default::Default::default(),
            python_packages: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElArtifactsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElArtifactsElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerBuildElArtifactsElRef {
        CloudbuildTriggerBuildElArtifactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElArtifactsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `images` after provisioning.\nA list of images to be pushed upon the successful completion of all build steps.\n\nThe images will be pushed using the builder service account's credentials.\n\nThe digests of the pushed images will be stored in the Build resource's results field.\n\nIf any of the images fail to be pushed, the build is marked FAILURE."]
    pub fn images(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.images", self.base))
    }
    #[doc = "Get a reference to the value of field `maven_artifacts` after provisioning.\n"]
    pub fn maven_artifacts(
        &self,
    ) -> ListRef<CloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maven_artifacts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `npm_packages` after provisioning.\n"]
    pub fn npm_packages(&self) -> ListRef<CloudbuildTriggerBuildElArtifactsElNpmPackagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.npm_packages", self.base))
    }
    #[doc = "Get a reference to the value of field `objects` after provisioning.\n"]
    pub fn objects(&self) -> ListRef<CloudbuildTriggerBuildElArtifactsElObjectsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }
    #[doc = "Get a reference to the value of field `python_packages` after provisioning.\n"]
    pub fn python_packages(
        &self,
    ) -> ListRef<CloudbuildTriggerBuildElArtifactsElPythonPackagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.python_packages", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
    env: PrimField<String>,
    version_name: PrimField<String>,
}
impl CloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {}
impl ToListMappable for CloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
    #[doc = "Environment variable name to associate with the secret. Secret environment\nvariables must be unique across all of a build's secrets, and must be used\nby at least one build step."]
    pub env: PrimField<String>,
    #[doc = "Resource name of the SecretVersion. In format: projects/*/secrets/*/versions/*"]
    pub version_name: PrimField<String>,
}
impl BuildCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
    pub fn build(self) -> CloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
        CloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
            env: self.env,
            version_name: self.version_name,
        }
    }
}
pub struct CloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
        CloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\nEnvironment variable name to associate with the secret. Secret environment\nvariables must be unique across all of a build's secrets, and must be used\nby at least one build step."]
    pub fn env(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `version_name` after provisioning.\nResource name of the SecretVersion. In format: projects/*/secrets/*/versions/*"]
    pub fn version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudbuildTriggerBuildElAvailableSecretsElDynamic {
    secret_manager: Option<DynamicBlock<CloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl>>,
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElAvailableSecretsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager: Option<Vec<CloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl>>,
    dynamic: CloudbuildTriggerBuildElAvailableSecretsElDynamic,
}
impl CloudbuildTriggerBuildElAvailableSecretsEl {
    #[doc = "Set the field `secret_manager`.\n"]
    pub fn set_secret_manager(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_manager = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_manager = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElAvailableSecretsEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElAvailableSecretsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElAvailableSecretsEl {}
impl BuildCloudbuildTriggerBuildElAvailableSecretsEl {
    pub fn build(self) -> CloudbuildTriggerBuildElAvailableSecretsEl {
        CloudbuildTriggerBuildElAvailableSecretsEl {
            secret_manager: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElAvailableSecretsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElAvailableSecretsElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerBuildElAvailableSecretsElRef {
        CloudbuildTriggerBuildElAvailableSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElAvailableSecretsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_manager` after provisioning.\n"]
    pub fn secret_manager(
        &self,
    ) -> ListRef<CloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_manager", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElOptionsElVolumesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl CloudbuildTriggerBuildElOptionsElVolumesEl {
    #[doc = "Set the field `name`.\nName of the volume to mount.\n\nVolume names must be unique per build step and must be valid names for Docker volumes.\nEach named volume must be used by at least two build steps."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nPath at which to mount the volume.\n\nPaths must be absolute and cannot conflict with other volume paths on the same\nbuild step or with certain reserved volume paths."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElOptionsElVolumesEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElOptionsElVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElOptionsElVolumesEl {}
impl BuildCloudbuildTriggerBuildElOptionsElVolumesEl {
    pub fn build(self) -> CloudbuildTriggerBuildElOptionsElVolumesEl {
        CloudbuildTriggerBuildElOptionsElVolumesEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElOptionsElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElOptionsElVolumesElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerBuildElOptionsElVolumesElRef {
        CloudbuildTriggerBuildElOptionsElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElOptionsElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the volume to mount.\n\nVolume names must be unique per build step and must be valid names for Docker volumes.\nEach named volume must be used by at least two build steps."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath at which to mount the volume.\n\nPaths must be absolute and cannot conflict with other volume paths on the same\nbuild step or with certain reserved volume paths."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudbuildTriggerBuildElOptionsElDynamic {
    volumes: Option<DynamicBlock<CloudbuildTriggerBuildElOptionsElVolumesEl>>,
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic_substitutions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_streaming_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_verify_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_env: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_provenance_hash: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    substitution_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_pool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<Vec<CloudbuildTriggerBuildElOptionsElVolumesEl>>,
    dynamic: CloudbuildTriggerBuildElOptionsElDynamic,
}
impl CloudbuildTriggerBuildElOptionsEl {
    #[doc = "Set the field `disk_size_gb`.\nRequested disk size for the VM that runs the build. Note that this is NOT \"disk free\";\nsome of the space will be used by the operating system and build utilities.\nAlso note that this is the minimum disk size that will be allocated for the build --\nthe build may run with a larger disk than requested. At present, the maximum disk size\nis 1000GB; builds that request more than the maximum are rejected with an error."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `dynamic_substitutions`.\nOption to specify whether or not to apply bash style string operations to the substitutions.\n\nNOTE this is always enabled for triggered builds and cannot be overridden in the build configuration file."]
    pub fn set_dynamic_substitutions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dynamic_substitutions = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\nA list of global environment variable definitions that will exist for all build steps\nin this build. If a variable is defined in both globally and in a build step,\nthe variable will use the build step value.\n\nThe elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."]
    pub fn set_env(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.env = Some(v.into());
        self
    }
    #[doc = "Set the field `log_streaming_option`.\nOption to define build log streaming behavior to Google Cloud Storage. Possible values: [\"STREAM_DEFAULT\", \"STREAM_ON\", \"STREAM_OFF\"]"]
    pub fn set_log_streaming_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_streaming_option = Some(v.into());
        self
    }
    #[doc = "Set the field `logging`.\nOption to specify the logging mode, which determines if and where build logs are stored. Possible values: [\"LOGGING_UNSPECIFIED\", \"LEGACY\", \"GCS_ONLY\", \"STACKDRIVER_ONLY\", \"CLOUD_LOGGING_ONLY\", \"NONE\"]"]
    pub fn set_logging(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nCompute Engine machine type on which to run the build."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `requested_verify_option`.\nRequested verifiability options. Possible values: [\"NOT_VERIFIED\", \"VERIFIED\"]"]
    pub fn set_requested_verify_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.requested_verify_option = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_env`.\nA list of global environment variables, which are encrypted using a Cloud Key Management\nService crypto key. These values must be specified in the build's Secret. These variables\nwill be available to all build steps in this build."]
    pub fn set_secret_env(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.secret_env = Some(v.into());
        self
    }
    #[doc = "Set the field `source_provenance_hash`.\nRequested hash for SourceProvenance. Possible values: [\"NONE\", \"SHA256\", \"MD5\"]"]
    pub fn set_source_provenance_hash(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.source_provenance_hash = Some(v.into());
        self
    }
    #[doc = "Set the field `substitution_option`.\nOption to specify behavior when there is an error in the substitution checks.\n\nNOTE this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden\nin the build configuration file. Possible values: [\"MUST_MATCH\", \"ALLOW_LOOSE\"]"]
    pub fn set_substitution_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.substitution_option = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_pool`.\nOption to specify a WorkerPool for the build. Format projects/{project}/workerPools/{workerPool}\n\nThis field is experimental."]
    pub fn set_worker_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.worker_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElOptionsElVolumesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volumes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volumes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElOptionsEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElOptionsEl {}
impl BuildCloudbuildTriggerBuildElOptionsEl {
    pub fn build(self) -> CloudbuildTriggerBuildElOptionsEl {
        CloudbuildTriggerBuildElOptionsEl {
            disk_size_gb: core::default::Default::default(),
            dynamic_substitutions: core::default::Default::default(),
            env: core::default::Default::default(),
            log_streaming_option: core::default::Default::default(),
            logging: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            requested_verify_option: core::default::Default::default(),
            secret_env: core::default::Default::default(),
            source_provenance_hash: core::default::Default::default(),
            substitution_option: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
            volumes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElOptionsElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerBuildElOptionsElRef {
        CloudbuildTriggerBuildElOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nRequested disk size for the VM that runs the build. Note that this is NOT \"disk free\";\nsome of the space will be used by the operating system and build utilities.\nAlso note that this is the minimum disk size that will be allocated for the build --\nthe build may run with a larger disk than requested. At present, the maximum disk size\nis 1000GB; builds that request more than the maximum are rejected with an error."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `dynamic_substitutions` after provisioning.\nOption to specify whether or not to apply bash style string operations to the substitutions.\n\nNOTE this is always enabled for triggered builds and cannot be overridden in the build configuration file."]
    pub fn dynamic_substitutions(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dynamic_substitutions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\nA list of global environment variable definitions that will exist for all build steps\nin this build. If a variable is defined in both globally and in a build step,\nthe variable will use the build step value.\n\nThe elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."]
    pub fn env(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `log_streaming_option` after provisioning.\nOption to define build log streaming behavior to Google Cloud Storage. Possible values: [\"STREAM_DEFAULT\", \"STREAM_ON\", \"STREAM_OFF\"]"]
    pub fn log_streaming_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_streaming_option", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\nOption to specify the logging mode, which determines if and where build logs are stored. Possible values: [\"LOGGING_UNSPECIFIED\", \"LEGACY\", \"GCS_ONLY\", \"STACKDRIVER_ONLY\", \"CLOUD_LOGGING_ONLY\", \"NONE\"]"]
    pub fn logging(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging", self.base))
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nCompute Engine machine type on which to run the build."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `requested_verify_option` after provisioning.\nRequested verifiability options. Possible values: [\"NOT_VERIFIED\", \"VERIFIED\"]"]
    pub fn requested_verify_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requested_verify_option", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_env` after provisioning.\nA list of global environment variables, which are encrypted using a Cloud Key Management\nService crypto key. These values must be specified in the build's Secret. These variables\nwill be available to all build steps in this build."]
    pub fn secret_env(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }
    #[doc = "Get a reference to the value of field `source_provenance_hash` after provisioning.\nRequested hash for SourceProvenance. Possible values: [\"NONE\", \"SHA256\", \"MD5\"]"]
    pub fn source_provenance_hash(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_provenance_hash", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `substitution_option` after provisioning.\nOption to specify behavior when there is an error in the substitution checks.\n\nNOTE this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden\nin the build configuration file. Possible values: [\"MUST_MATCH\", \"ALLOW_LOOSE\"]"]
    pub fn substitution_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.substitution_option", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `worker_pool` after provisioning.\nOption to specify a WorkerPool for the build. Format projects/{project}/workerPools/{workerPool}\n\nThis field is experimental."]
    pub fn worker_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> ListRef<CloudbuildTriggerBuildElOptionsElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElSecretEl {
    kms_key_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_env: Option<RecField<PrimField<String>>>,
}
impl CloudbuildTriggerBuildElSecretEl {
    #[doc = "Set the field `secret_env`.\nMap of environment variable name to its encrypted value.\nSecret environment variables must be unique across all of a build's secrets,\nand must be used by at least one build step. Values can be at most 64 KB in size.\nThere can be at most 100 secret values across all of a build's secrets."]
    pub fn set_secret_env(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.secret_env = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElSecretEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElSecretEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElSecretEl {
    #[doc = "Cloud KMS key name to use to decrypt these envs."]
    pub kms_key_name: PrimField<String>,
}
impl BuildCloudbuildTriggerBuildElSecretEl {
    pub fn build(self) -> CloudbuildTriggerBuildElSecretEl {
        CloudbuildTriggerBuildElSecretEl {
            kms_key_name: self.kms_key_name,
            secret_env: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElSecretElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerBuildElSecretElRef {
        CloudbuildTriggerBuildElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nCloud KMS key name to use to decrypt these envs."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_env` after provisioning.\nMap of environment variable name to its encrypted value.\nSecret environment variables must be unique across all of a build's secrets,\nand must be used by at least one build step. Values can be at most 64 KB in size.\nThere can be at most 100 secret values across all of a build's secrets."]
    pub fn secret_env(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElSourceElRepoSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_sha: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    repo_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    substitutions: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name: Option<PrimField<String>>,
}
impl CloudbuildTriggerBuildElSourceElRepoSourceEl {
    #[doc = "Set the field `branch_name`.\nRegex matching branches to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and\ndescribed at https://github.com/google/re2/wiki/Syntax"]
    pub fn set_branch_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch_name = Some(v.into());
        self
    }
    #[doc = "Set the field `commit_sha`.\nExplicit commit SHA to build. Exactly one a of branch name, tag, or commit SHA must be provided."]
    pub fn set_commit_sha(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_sha = Some(v.into());
        self
    }
    #[doc = "Set the field `dir`.\nDirectory, relative to the source root, in which to run the build.\nThis must be a relative path. If a step's dir is specified and is an absolute path,\nthis value is ignored for that step's execution."]
    pub fn set_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dir = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\nOnly trigger a build if the revision regex does NOT match the revision regex."]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nID of the project that owns the Cloud Source Repository.\nIf omitted, the project ID requesting the build is assumed."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `substitutions`.\nSubstitutions to use in a triggered build. Should only be used with triggers.run"]
    pub fn set_substitutions(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.substitutions = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_name`.\nRegex matching tags to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and\ndescribed at https://github.com/google/re2/wiki/Syntax"]
    pub fn set_tag_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_name = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElSourceElRepoSourceEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElSourceElRepoSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElSourceElRepoSourceEl {
    #[doc = "Name of the Cloud Source Repository."]
    pub repo_name: PrimField<String>,
}
impl BuildCloudbuildTriggerBuildElSourceElRepoSourceEl {
    pub fn build(self) -> CloudbuildTriggerBuildElSourceElRepoSourceEl {
        CloudbuildTriggerBuildElSourceElRepoSourceEl {
            branch_name: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            dir: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            project_id: core::default::Default::default(),
            repo_name: self.repo_name,
            substitutions: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElSourceElRepoSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElSourceElRepoSourceElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerBuildElSourceElRepoSourceElRef {
        CloudbuildTriggerBuildElSourceElRepoSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElSourceElRepoSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\nRegex matching branches to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and\ndescribed at https://github.com/google/re2/wiki/Syntax"]
    pub fn branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }
    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\nExplicit commit SHA to build. Exactly one a of branch name, tag, or commit SHA must be provided."]
    pub fn commit_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }
    #[doc = "Get a reference to the value of field `dir` after provisioning.\nDirectory, relative to the source root, in which to run the build.\nThis must be a relative path. If a step's dir is specified and is an absolute path,\nthis value is ignored for that step's execution."]
    pub fn dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\nOnly trigger a build if the revision regex does NOT match the revision regex."]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nID of the project that owns the Cloud Source Repository.\nIf omitted, the project ID requesting the build is assumed."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_name` after provisioning.\nName of the Cloud Source Repository."]
    pub fn repo_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }
    #[doc = "Get a reference to the value of field `substitutions` after provisioning.\nSubstitutions to use in a triggered build. Should only be used with triggers.run"]
    pub fn substitutions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.substitutions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_name` after provisioning.\nRegex matching tags to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThe syntax of the regular expressions accepted is the syntax accepted by RE2 and\ndescribed at https://github.com/google/re2/wiki/Syntax"]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElSourceElStorageSourceEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    object: PrimField<String>,
}
impl CloudbuildTriggerBuildElSourceElStorageSourceEl {
    #[doc = "Set the field `generation`.\nGoogle Cloud Storage generation for the object.\nIf the generation is omitted, the latest generation will be used"]
    pub fn set_generation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.generation = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElSourceElStorageSourceEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElSourceElStorageSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElSourceElStorageSourceEl {
    #[doc = "Google Cloud Storage bucket containing the source."]
    pub bucket: PrimField<String>,
    #[doc = "Google Cloud Storage object containing the source.\nThis object must be a gzipped archive file (.tar.gz) containing source to build."]
    pub object: PrimField<String>,
}
impl BuildCloudbuildTriggerBuildElSourceElStorageSourceEl {
    pub fn build(self) -> CloudbuildTriggerBuildElSourceElStorageSourceEl {
        CloudbuildTriggerBuildElSourceElStorageSourceEl {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}
pub struct CloudbuildTriggerBuildElSourceElStorageSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElSourceElStorageSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerBuildElSourceElStorageSourceElRef {
        CloudbuildTriggerBuildElSourceElStorageSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElSourceElStorageSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nGoogle Cloud Storage bucket containing the source."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nGoogle Cloud Storage generation for the object.\nIf the generation is omitted, the latest generation will be used"]
    pub fn generation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\nGoogle Cloud Storage object containing the source.\nThis object must be a gzipped archive file (.tar.gz) containing source to build."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudbuildTriggerBuildElSourceElDynamic {
    repo_source: Option<DynamicBlock<CloudbuildTriggerBuildElSourceElRepoSourceEl>>,
    storage_source: Option<DynamicBlock<CloudbuildTriggerBuildElSourceElStorageSourceEl>>,
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_source: Option<Vec<CloudbuildTriggerBuildElSourceElRepoSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_source: Option<Vec<CloudbuildTriggerBuildElSourceElStorageSourceEl>>,
    dynamic: CloudbuildTriggerBuildElSourceElDynamic,
}
impl CloudbuildTriggerBuildElSourceEl {
    #[doc = "Set the field `repo_source`.\n"]
    pub fn set_repo_source(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElSourceElRepoSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.repo_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.repo_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage_source`.\n"]
    pub fn set_storage_source(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElSourceElStorageSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElSourceEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElSourceEl {}
impl BuildCloudbuildTriggerBuildElSourceEl {
    pub fn build(self) -> CloudbuildTriggerBuildElSourceEl {
        CloudbuildTriggerBuildElSourceEl {
            repo_source: core::default::Default::default(),
            storage_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElSourceElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerBuildElSourceElRef {
        CloudbuildTriggerBuildElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repo_source` after provisioning.\n"]
    pub fn repo_source(&self) -> ListRef<CloudbuildTriggerBuildElSourceElRepoSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.repo_source", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_source` after provisioning.\n"]
    pub fn storage_source(&self) -> ListRef<CloudbuildTriggerBuildElSourceElStorageSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_source", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElStepElVolumesEl {
    name: PrimField<String>,
    path: PrimField<String>,
}
impl CloudbuildTriggerBuildElStepElVolumesEl {}
impl ToListMappable for CloudbuildTriggerBuildElStepElVolumesEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElStepElVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElStepElVolumesEl {
    #[doc = "Name of the volume to mount.\n\nVolume names must be unique per build step and must be valid names for\nDocker volumes. Each named volume must be used by at least two build steps."]
    pub name: PrimField<String>,
    #[doc = "Path at which to mount the volume.\n\nPaths must be absolute and cannot conflict with other volume paths on\nthe same build step or with certain reserved volume paths."]
    pub path: PrimField<String>,
}
impl BuildCloudbuildTriggerBuildElStepElVolumesEl {
    pub fn build(self) -> CloudbuildTriggerBuildElStepElVolumesEl {
        CloudbuildTriggerBuildElStepElVolumesEl {
            name: self.name,
            path: self.path,
        }
    }
}
pub struct CloudbuildTriggerBuildElStepElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElStepElVolumesElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerBuildElStepElVolumesElRef {
        CloudbuildTriggerBuildElStepElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElStepElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the volume to mount.\n\nVolume names must be unique per build step and must be valid names for\nDocker volumes. Each named volume must be used by at least two build steps."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath at which to mount the volume.\n\nPaths must be absolute and cannot conflict with other volume paths on\nthe same build step or with certain reserved volume paths."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudbuildTriggerBuildElStepElDynamic {
    volumes: Option<DynamicBlock<CloudbuildTriggerBuildElStepElVolumesEl>>,
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildElStepEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_exit_codes: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_failure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entrypoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_env: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<Vec<CloudbuildTriggerBuildElStepElVolumesEl>>,
    dynamic: CloudbuildTriggerBuildElStepElDynamic,
}
impl CloudbuildTriggerBuildElStepEl {
    #[doc = "Set the field `allow_exit_codes`.\nAllow this build step to fail without failing the entire build if and\nonly if the exit code is one of the specified codes.\n\nIf 'allowFailure' is also specified, this field will take precedence."]
    pub fn set_allow_exit_codes(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.allow_exit_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_failure`.\nAllow this build step to fail without failing the entire build.\nIf false, the entire build will fail if this step fails. Otherwise, the\nbuild will succeed, but this step will still have a failure status.\nError information will be reported in the 'failureDetail' field.\n\n'allowExitCodes' takes precedence over this field."]
    pub fn set_allow_failure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `args`.\nA list of arguments that will be presented to the step when it is started.\n\nIf the image used to run the step's container has an entrypoint, the args\nare used as arguments to that entrypoint. If the image does not define an\nentrypoint, the first element in args is used as the entrypoint, and the\nremainder will be used as arguments."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `dir`.\nWorking directory to use when running this step's container.\n\nIf this value is a relative path, it is relative to the build's working\ndirectory. If this value is absolute, it may be outside the build's working\ndirectory, in which case the contents of the path may not be persisted\nacross build step executions, unless a 'volume' for that path is specified.\n\nIf the build specifies a 'RepoSource' with 'dir' and a step with a\n'dir',\nwhich specifies an absolute path, the 'RepoSource' 'dir' is ignored\nfor the step's execution."]
    pub fn set_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dir = Some(v.into());
        self
    }
    #[doc = "Set the field `entrypoint`.\nEntrypoint to be used instead of the build step image's\ndefault entrypoint.\nIf unset, the image's default entrypoint is used"]
    pub fn set_entrypoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entrypoint = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\nA list of environment variable definitions to be used when\nrunning a step.\n\nThe elements are of the form \"KEY=VALUE\" for the environment variable\n\"KEY\" being given the value \"VALUE\"."]
    pub fn set_env(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.env = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\nUnique identifier for this build step, used in 'wait_for' to\nreference this build step as a dependency."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `script`.\nA shell script to be executed in the step.\nWhen script is provided, the user cannot specify the entrypoint or args."]
    pub fn set_script(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.script = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_env`.\nA list of environment variables which are encrypted using\na Cloud Key\nManagement Service crypto key. These values must be specified in\nthe build's 'Secret'."]
    pub fn set_secret_env(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.secret_env = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\nTime limit for executing this build step. If not defined,\nthe step has no\ntime limit and will be allowed to continue to run until either it\ncompletes or the build itself times out."]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `timing`.\nOutput only. Stores timing information for executing this\nbuild step."]
    pub fn set_timing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timing = Some(v.into());
        self
    }
    #[doc = "Set the field `wait_for`.\nThe ID(s) of the step(s) that this build step depends on.\n\nThis build step will not start until all the build steps in 'wait_for'\nhave completed successfully. If 'wait_for' is empty, this build step\nwill start when all previous build steps in the 'Build.Steps' list\nhave completed successfully."]
    pub fn set_wait_for(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.wait_for = Some(v.into());
        self
    }
    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElStepElVolumesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volumes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volumes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildElStepEl {
    type O = BlockAssignable<CloudbuildTriggerBuildElStepEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildElStepEl {
    #[doc = "The name of the container image that will run this particular build step.\n\nIf the image is available in the host's Docker daemon's cache, it will be\nrun directly. If not, the host will attempt to pull the image first, using\nthe builder service account's credentials if necessary.\n\nThe Docker daemon's cache will already have the latest versions of all of\nthe officially supported build steps (see https://github.com/GoogleCloudPlatform/cloud-builders\nfor images and examples).\nThe Docker daemon will also have cached many of the layers for some popular\nimages, like \"ubuntu\", \"debian\", but they will be refreshed at the time\nyou attempt to use them.\n\nIf you built an image in a previous build step, it will be stored in the\nhost's Docker daemon's cache and is available to use as the name for a\nlater build step."]
    pub name: PrimField<String>,
}
impl BuildCloudbuildTriggerBuildElStepEl {
    pub fn build(self) -> CloudbuildTriggerBuildElStepEl {
        CloudbuildTriggerBuildElStepEl {
            allow_exit_codes: core::default::Default::default(),
            allow_failure: core::default::Default::default(),
            args: core::default::Default::default(),
            dir: core::default::Default::default(),
            entrypoint: core::default::Default::default(),
            env: core::default::Default::default(),
            id: core::default::Default::default(),
            name: self.name,
            script: core::default::Default::default(),
            secret_env: core::default::Default::default(),
            timeout: core::default::Default::default(),
            timing: core::default::Default::default(),
            wait_for: core::default::Default::default(),
            volumes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElStepElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElStepElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerBuildElStepElRef {
        CloudbuildTriggerBuildElStepElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElStepElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_exit_codes` after provisioning.\nAllow this build step to fail without failing the entire build if and\nonly if the exit code is one of the specified codes.\n\nIf 'allowFailure' is also specified, this field will take precedence."]
    pub fn allow_exit_codes(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_exit_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_failure` after provisioning.\nAllow this build step to fail without failing the entire build.\nIf false, the entire build will fail if this step fails. Otherwise, the\nbuild will succeed, but this step will still have a failure status.\nError information will be reported in the 'failureDetail' field.\n\n'allowExitCodes' takes precedence over this field."]
    pub fn allow_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nA list of arguments that will be presented to the step when it is started.\n\nIf the image used to run the step's container has an entrypoint, the args\nare used as arguments to that entrypoint. If the image does not define an\nentrypoint, the first element in args is used as the entrypoint, and the\nremainder will be used as arguments."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `dir` after provisioning.\nWorking directory to use when running this step's container.\n\nIf this value is a relative path, it is relative to the build's working\ndirectory. If this value is absolute, it may be outside the build's working\ndirectory, in which case the contents of the path may not be persisted\nacross build step executions, unless a 'volume' for that path is specified.\n\nIf the build specifies a 'RepoSource' with 'dir' and a step with a\n'dir',\nwhich specifies an absolute path, the 'RepoSource' 'dir' is ignored\nfor the step's execution."]
    pub fn dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }
    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\nEntrypoint to be used instead of the build step image's\ndefault entrypoint.\nIf unset, the image's default entrypoint is used"]
    pub fn entrypoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entrypoint", self.base))
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\nA list of environment variable definitions to be used when\nrunning a step.\n\nThe elements are of the form \"KEY=VALUE\" for the environment variable\n\"KEY\" being given the value \"VALUE\"."]
    pub fn env(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for this build step, used in 'wait_for' to\nreference this build step as a dependency."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the container image that will run this particular build step.\n\nIf the image is available in the host's Docker daemon's cache, it will be\nrun directly. If not, the host will attempt to pull the image first, using\nthe builder service account's credentials if necessary.\n\nThe Docker daemon's cache will already have the latest versions of all of\nthe officially supported build steps (see https://github.com/GoogleCloudPlatform/cloud-builders\nfor images and examples).\nThe Docker daemon will also have cached many of the layers for some popular\nimages, like \"ubuntu\", \"debian\", but they will be refreshed at the time\nyou attempt to use them.\n\nIf you built an image in a previous build step, it will be stored in the\nhost's Docker daemon's cache and is available to use as the name for a\nlater build step."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `script` after provisioning.\nA shell script to be executed in the step.\nWhen script is provided, the user cannot specify the entrypoint or args."]
    pub fn script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_env` after provisioning.\nA list of environment variables which are encrypted using\na Cloud Key\nManagement Service crypto key. These values must be specified in\nthe build's 'Secret'."]
    pub fn secret_env(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nTime limit for executing this build step. If not defined,\nthe step has no\ntime limit and will be allowed to continue to run until either it\ncompletes or the build itself times out."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `timing` after provisioning.\nOutput only. Stores timing information for executing this\nbuild step."]
    pub fn timing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timing", self.base))
    }
    #[doc = "Get a reference to the value of field `wait_for` after provisioning.\nThe ID(s) of the step(s) that this build step depends on.\n\nThis build step will not start until all the build steps in 'wait_for'\nhave completed successfully. If 'wait_for' is empty, this build step\nwill start when all previous build steps in the 'Build.Steps' list\nhave completed successfully."]
    pub fn wait_for(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.wait_for", self.base))
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> ListRef<CloudbuildTriggerBuildElStepElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudbuildTriggerBuildElDynamic {
    artifacts: Option<DynamicBlock<CloudbuildTriggerBuildElArtifactsEl>>,
    available_secrets: Option<DynamicBlock<CloudbuildTriggerBuildElAvailableSecretsEl>>,
    options: Option<DynamicBlock<CloudbuildTriggerBuildElOptionsEl>>,
    secret: Option<DynamicBlock<CloudbuildTriggerBuildElSecretEl>>,
    source: Option<DynamicBlock<CloudbuildTriggerBuildElSourceEl>>,
    step: Option<DynamicBlock<CloudbuildTriggerBuildElStepEl>>,
}
#[derive(Serialize)]
pub struct CloudbuildTriggerBuildEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logs_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    substitutions: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artifacts: Option<Vec<CloudbuildTriggerBuildElArtifactsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_secrets: Option<Vec<CloudbuildTriggerBuildElAvailableSecretsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<CloudbuildTriggerBuildElOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<Vec<CloudbuildTriggerBuildElSecretEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<CloudbuildTriggerBuildElSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<Vec<CloudbuildTriggerBuildElStepEl>>,
    dynamic: CloudbuildTriggerBuildElDynamic,
}
impl CloudbuildTriggerBuildEl {
    #[doc = "Set the field `images`.\nA list of images to be pushed upon the successful completion of all build steps.\nThe images are pushed using the builder service account's credentials.\nThe digests of the pushed images will be stored in the Build resource's results field.\nIf any of the images fail to be pushed, the build status is marked FAILURE."]
    pub fn set_images(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.images = Some(v.into());
        self
    }
    #[doc = "Set the field `logs_bucket`.\nGoogle Cloud Storage bucket where logs should be written.\nLogs file names will be of the format ${logsBucket}/log-${build_id}.txt."]
    pub fn set_logs_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logs_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `queue_ttl`.\nTTL in queue for this build. If provided and the build is enqueued longer than this value,\nthe build will expire and the build status will be EXPIRED.\nThe TTL starts ticking from createTime.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_queue_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.queue_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `substitutions`.\nSubstitutions data for Build resource."]
    pub fn set_substitutions(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.substitutions = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nTags for annotation of a Build. These are not docker tags."]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\nAmount of time that this build should be allowed to run, to second granularity.\nIf this amount of time elapses, work on the build will cease and the build status will be TIMEOUT.\nThis timeout must be equal to or greater than the sum of the timeouts for build steps within the build.\nThe expected format is the number of seconds followed by s.\nDefault time is ten minutes (600s)."]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `artifacts`.\n"]
    pub fn set_artifacts(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElArtifactsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.artifacts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.artifacts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `available_secrets`.\n"]
    pub fn set_available_secrets(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElAvailableSecretsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.available_secrets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.available_secrets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElSecretEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `step`.\n"]
    pub fn set_step(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerBuildElStepEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.step = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.step = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudbuildTriggerBuildEl {
    type O = BlockAssignable<CloudbuildTriggerBuildEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerBuildEl {}
impl BuildCloudbuildTriggerBuildEl {
    pub fn build(self) -> CloudbuildTriggerBuildEl {
        CloudbuildTriggerBuildEl {
            images: core::default::Default::default(),
            logs_bucket: core::default::Default::default(),
            queue_ttl: core::default::Default::default(),
            substitutions: core::default::Default::default(),
            tags: core::default::Default::default(),
            timeout: core::default::Default::default(),
            artifacts: core::default::Default::default(),
            available_secrets: core::default::Default::default(),
            options: core::default::Default::default(),
            secret: core::default::Default::default(),
            source: core::default::Default::default(),
            step: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudbuildTriggerBuildElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerBuildElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerBuildElRef {
        CloudbuildTriggerBuildElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerBuildElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `images` after provisioning.\nA list of images to be pushed upon the successful completion of all build steps.\nThe images are pushed using the builder service account's credentials.\nThe digests of the pushed images will be stored in the Build resource's results field.\nIf any of the images fail to be pushed, the build status is marked FAILURE."]
    pub fn images(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.images", self.base))
    }
    #[doc = "Get a reference to the value of field `logs_bucket` after provisioning.\nGoogle Cloud Storage bucket where logs should be written.\nLogs file names will be of the format ${logsBucket}/log-${build_id}.txt."]
    pub fn logs_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logs_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `queue_ttl` after provisioning.\nTTL in queue for this build. If provided and the build is enqueued longer than this value,\nthe build will expire and the build status will be EXPIRED.\nThe TTL starts ticking from createTime.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn queue_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `substitutions` after provisioning.\nSubstitutions data for Build resource."]
    pub fn substitutions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.substitutions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags for annotation of a Build. These are not docker tags."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nAmount of time that this build should be allowed to run, to second granularity.\nIf this amount of time elapses, work on the build will cease and the build status will be TIMEOUT.\nThis timeout must be equal to or greater than the sum of the timeouts for build steps within the build.\nThe expected format is the number of seconds followed by s.\nDefault time is ten minutes (600s)."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `artifacts` after provisioning.\n"]
    pub fn artifacts(&self) -> ListRef<CloudbuildTriggerBuildElArtifactsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.artifacts", self.base))
    }
    #[doc = "Get a reference to the value of field `available_secrets` after provisioning.\n"]
    pub fn available_secrets(&self) -> ListRef<CloudbuildTriggerBuildElAvailableSecretsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_secrets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> ListRef<CloudbuildTriggerBuildElOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> ListRef<CloudbuildTriggerBuildElSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<CloudbuildTriggerBuildElSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> ListRef<CloudbuildTriggerBuildElStepElRef> {
        ListRef::new(self.shared().clone(), format!("{}.step", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
}
impl CloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
    #[doc = "Set the field `branch`.\nRegex of branches to match."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `comment_control`.\nConfigure builds to run whether a repository owner or collaborator need to comment '/gcbrun'. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn set_comment_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment_control = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
    type O = BlockAssignable<CloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {}
impl BuildCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
    pub fn build(self) -> CloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
        CloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef {
        CloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `comment_control` after provisioning.\nConfigure builds to run whether a repository owner or collaborator need to comment '/gcbrun'. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn comment_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerDeveloperConnectEventConfigElPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl CloudbuildTriggerDeveloperConnectEventConfigElPushEl {
    #[doc = "Set the field `branch`.\nRegex of branches to match."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\nIf true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nRegex of tags to match."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerDeveloperConnectEventConfigElPushEl {
    type O = BlockAssignable<CloudbuildTriggerDeveloperConnectEventConfigElPushEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerDeveloperConnectEventConfigElPushEl {}
impl BuildCloudbuildTriggerDeveloperConnectEventConfigElPushEl {
    pub fn build(self) -> CloudbuildTriggerDeveloperConnectEventConfigElPushEl {
        CloudbuildTriggerDeveloperConnectEventConfigElPushEl {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerDeveloperConnectEventConfigElPushElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerDeveloperConnectEventConfigElPushElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerDeveloperConnectEventConfigElPushElRef {
        CloudbuildTriggerDeveloperConnectEventConfigElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerDeveloperConnectEventConfigElPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nRegex of tags to match."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudbuildTriggerDeveloperConnectEventConfigElDynamic {
    pull_request: Option<DynamicBlock<CloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl>>,
    push: Option<DynamicBlock<CloudbuildTriggerDeveloperConnectEventConfigElPushEl>>,
}
#[derive(Serialize)]
pub struct CloudbuildTriggerDeveloperConnectEventConfigEl {
    git_repository_link: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<Vec<CloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<Vec<CloudbuildTriggerDeveloperConnectEventConfigElPushEl>>,
    dynamic: CloudbuildTriggerDeveloperConnectEventConfigElDynamic,
}
impl CloudbuildTriggerDeveloperConnectEventConfigEl {
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pull_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pull_request = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerDeveloperConnectEventConfigElPushEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.push = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.push = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudbuildTriggerDeveloperConnectEventConfigEl {
    type O = BlockAssignable<CloudbuildTriggerDeveloperConnectEventConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerDeveloperConnectEventConfigEl {
    #[doc = "The Developer Connect Git repository link, formatted as 'projects/*/locations/*/connections/*/gitRepositoryLink/*'."]
    pub git_repository_link: PrimField<String>,
}
impl BuildCloudbuildTriggerDeveloperConnectEventConfigEl {
    pub fn build(self) -> CloudbuildTriggerDeveloperConnectEventConfigEl {
        CloudbuildTriggerDeveloperConnectEventConfigEl {
            git_repository_link: self.git_repository_link,
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudbuildTriggerDeveloperConnectEventConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerDeveloperConnectEventConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerDeveloperConnectEventConfigElRef {
        CloudbuildTriggerDeveloperConnectEventConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerDeveloperConnectEventConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `git_repository_link` after provisioning.\nThe Developer Connect Git repository link, formatted as 'projects/*/locations/*/connections/*/gitRepositoryLink/*'."]
    pub fn git_repository_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.git_repository_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `git_repository_link_type` after provisioning.\nThe type of DeveloperConnect GitRepositoryLink."]
    pub fn git_repository_link_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.git_repository_link_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(
        &self,
    ) -> ListRef<CloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }
    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> ListRef<CloudbuildTriggerDeveloperConnectEventConfigElPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerGitFileSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket_server_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_enterprise_config: Option<PrimField<String>>,
    path: PrimField<String>,
    repo_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl CloudbuildTriggerGitFileSourceEl {
    #[doc = "Set the field `bitbucket_server_config`.\nThe full resource name of the bitbucket server config.\nFormat: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}."]
    pub fn set_bitbucket_server_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bitbucket_server_config = Some(v.into());
        self
    }
    #[doc = "Set the field `github_enterprise_config`.\nThe full resource name of the github enterprise config.\nFormat: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}."]
    pub fn set_github_enterprise_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.github_enterprise_config = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\nThe fully qualified resource name of the Repo API repository. The fully qualified resource name of the Repo API repository.\nIf unspecified, the repo from which the trigger invocation originated is assumed to be the repo from which to read the specified path."]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\nThe branch, tag, arbitrary ref, or SHA version of the repo to use when resolving the\nfilename (optional). This field respects the same syntax/resolution as described here: https://git-scm.com/docs/gitrevisions\nIf unspecified, the revision from which the trigger invocation originated is assumed to be the revision from which to read the specified path."]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\nThe URI of the repo (optional). If unspecified, the repo from which the trigger\ninvocation originated is assumed to be the repo from which to read the specified path."]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerGitFileSourceEl {
    type O = BlockAssignable<CloudbuildTriggerGitFileSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerGitFileSourceEl {
    #[doc = "The path of the file, with the repo root as the root of the path."]
    pub path: PrimField<String>,
    #[doc = "The type of the repo, since it may not be explicit from the repo field (e.g from a URL).\nValues can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER Possible values: [\"UNKNOWN\", \"CLOUD_SOURCE_REPOSITORIES\", \"GITHUB\", \"BITBUCKET_SERVER\"]"]
    pub repo_type: PrimField<String>,
}
impl BuildCloudbuildTriggerGitFileSourceEl {
    pub fn build(self) -> CloudbuildTriggerGitFileSourceEl {
        CloudbuildTriggerGitFileSourceEl {
            bitbucket_server_config: core::default::Default::default(),
            github_enterprise_config: core::default::Default::default(),
            path: self.path,
            repo_type: self.repo_type,
            repository: core::default::Default::default(),
            revision: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerGitFileSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerGitFileSourceElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerGitFileSourceElRef {
        CloudbuildTriggerGitFileSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerGitFileSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitbucket_server_config` after provisioning.\nThe full resource name of the bitbucket server config.\nFormat: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}."]
    pub fn bitbucket_server_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\nThe full resource name of the github enterprise config.\nFormat: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}."]
    pub fn github_enterprise_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.github_enterprise_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe path of the file, with the repo root as the root of the path."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_type` after provisioning.\nThe type of the repo, since it may not be explicit from the repo field (e.g from a URL).\nValues can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER Possible values: [\"UNKNOWN\", \"CLOUD_SOURCE_REPOSITORIES\", \"GITHUB\", \"BITBUCKET_SERVER\"]"]
    pub fn repo_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_type", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nThe fully qualified resource name of the Repo API repository. The fully qualified resource name of the Repo API repository.\nIf unspecified, the repo from which the trigger invocation originated is assumed to be the repo from which to read the specified path."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nThe branch, tag, arbitrary ref, or SHA version of the repo to use when resolving the\nfilename (optional). This field respects the same syntax/resolution as described here: https://git-scm.com/docs/gitrevisions\nIf unspecified, the revision from which the trigger invocation originated is assumed to be the revision from which to read the specified path."]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe URI of the repo (optional). If unspecified, the repo from which the trigger\ninvocation originated is assumed to be the repo from which to read the specified path."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerGithubElPullRequestEl {
    branch: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
}
impl CloudbuildTriggerGithubElPullRequestEl {
    #[doc = "Set the field `comment_control`.\nWhether to block builds on a \"/gcbrun\" comment from a repository owner or collaborator. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn set_comment_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment_control = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerGithubElPullRequestEl {
    type O = BlockAssignable<CloudbuildTriggerGithubElPullRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerGithubElPullRequestEl {
    #[doc = "Regex of branches to match."]
    pub branch: PrimField<String>,
}
impl BuildCloudbuildTriggerGithubElPullRequestEl {
    pub fn build(self) -> CloudbuildTriggerGithubElPullRequestEl {
        CloudbuildTriggerGithubElPullRequestEl {
            branch: self.branch,
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerGithubElPullRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerGithubElPullRequestElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerGithubElPullRequestElRef {
        CloudbuildTriggerGithubElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerGithubElPullRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `comment_control` after provisioning.\nWhether to block builds on a \"/gcbrun\" comment from a repository owner or collaborator. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn comment_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerGithubElPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl CloudbuildTriggerGithubElPushEl {
    #[doc = "Set the field `branch`.\nRegex of branches to match.  Specify only one of branch or tag."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\nWhen true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nRegex of tags to match.  Specify only one of branch or tag."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerGithubElPushEl {
    type O = BlockAssignable<CloudbuildTriggerGithubElPushEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerGithubElPushEl {}
impl BuildCloudbuildTriggerGithubElPushEl {
    pub fn build(self) -> CloudbuildTriggerGithubElPushEl {
        CloudbuildTriggerGithubElPushEl {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerGithubElPushElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerGithubElPushElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerGithubElPushElRef {
        CloudbuildTriggerGithubElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerGithubElPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match.  Specify only one of branch or tag."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\nWhen true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nRegex of tags to match.  Specify only one of branch or tag."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudbuildTriggerGithubElDynamic {
    pull_request: Option<DynamicBlock<CloudbuildTriggerGithubElPullRequestEl>>,
    push: Option<DynamicBlock<CloudbuildTriggerGithubElPushEl>>,
}
#[derive(Serialize)]
pub struct CloudbuildTriggerGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enterprise_config_resource_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<Vec<CloudbuildTriggerGithubElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<Vec<CloudbuildTriggerGithubElPushEl>>,
    dynamic: CloudbuildTriggerGithubElDynamic,
}
impl CloudbuildTriggerGithubEl {
    #[doc = "Set the field `enterprise_config_resource_name`.\nThe resource name of the github enterprise config that should be applied to this installation.\nFor example: \"projects/{$projectId}/locations/{$locationId}/githubEnterpriseConfigs/{$configId}\""]
    pub fn set_enterprise_config_resource_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enterprise_config_resource_name = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nName of the repository. For example: The name for\nhttps://github.com/googlecloudplatform/cloud-builders is \"cloud-builders\"."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `owner`.\nOwner of the repository. For example: The owner for\nhttps://github.com/googlecloudplatform/cloud-builders is \"googlecloudplatform\"."]
    pub fn set_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner = Some(v.into());
        self
    }
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerGithubElPullRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pull_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pull_request = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerGithubElPushEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.push = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.push = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudbuildTriggerGithubEl {
    type O = BlockAssignable<CloudbuildTriggerGithubEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerGithubEl {}
impl BuildCloudbuildTriggerGithubEl {
    pub fn build(self) -> CloudbuildTriggerGithubEl {
        CloudbuildTriggerGithubEl {
            enterprise_config_resource_name: core::default::Default::default(),
            name: core::default::Default::default(),
            owner: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudbuildTriggerGithubElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerGithubElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerGithubElRef {
        CloudbuildTriggerGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enterprise_config_resource_name` after provisioning.\nThe resource name of the github enterprise config that should be applied to this installation.\nFor example: \"projects/{$projectId}/locations/{$locationId}/githubEnterpriseConfigs/{$configId}\""]
    pub fn enterprise_config_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enterprise_config_resource_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the repository. For example: The name for\nhttps://github.com/googlecloudplatform/cloud-builders is \"cloud-builders\"."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `owner` after provisioning.\nOwner of the repository. For example: The owner for\nhttps://github.com/googlecloudplatform/cloud-builders is \"googlecloudplatform\"."]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> ListRef<CloudbuildTriggerGithubElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }
    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> ListRef<CloudbuildTriggerGithubElPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerPubsubConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    topic: PrimField<String>,
}
impl CloudbuildTriggerPubsubConfigEl {
    #[doc = "Set the field `service_account_email`.\nService account that will make the push request."]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerPubsubConfigEl {
    type O = BlockAssignable<CloudbuildTriggerPubsubConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerPubsubConfigEl {
    #[doc = "The name of the topic from which this subscription is receiving messages."]
    pub topic: PrimField<String>,
}
impl BuildCloudbuildTriggerPubsubConfigEl {
    pub fn build(self) -> CloudbuildTriggerPubsubConfigEl {
        CloudbuildTriggerPubsubConfigEl {
            service_account_email: core::default::Default::default(),
            topic: self.topic,
        }
    }
}
pub struct CloudbuildTriggerPubsubConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerPubsubConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerPubsubConfigElRef {
        CloudbuildTriggerPubsubConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerPubsubConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nService account that will make the push request."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nPotential issues with the underlying Pub/Sub subscription configuration.\nOnly populated on get requests."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `subscription` after provisioning.\nOutput only. Name of the subscription."]
    pub fn subscription(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nThe name of the topic from which this subscription is receiving messages."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerRepositoryEventConfigElPullRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
}
impl CloudbuildTriggerRepositoryEventConfigElPullRequestEl {
    #[doc = "Set the field `branch`.\nRegex of branches to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `comment_control`.\nConfigure builds to run whether a repository owner or collaborator need to comment '/gcbrun'. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn set_comment_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment_control = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerRepositoryEventConfigElPullRequestEl {
    type O = BlockAssignable<CloudbuildTriggerRepositoryEventConfigElPullRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerRepositoryEventConfigElPullRequestEl {}
impl BuildCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
    pub fn build(self) -> CloudbuildTriggerRepositoryEventConfigElPullRequestEl {
        CloudbuildTriggerRepositoryEventConfigElPullRequestEl {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
        CloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `comment_control` after provisioning.\nConfigure builds to run whether a repository owner or collaborator need to comment '/gcbrun'. Possible values: [\"COMMENTS_DISABLED\", \"COMMENTS_ENABLED\", \"COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY\"]"]
    pub fn comment_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, branches that do NOT match the git_ref will trigger a build."]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerRepositoryEventConfigElPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl CloudbuildTriggerRepositoryEventConfigElPushEl {
    #[doc = "Set the field `branch`.\nRegex of branches to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\nIf true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nRegex of tags to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerRepositoryEventConfigElPushEl {
    type O = BlockAssignable<CloudbuildTriggerRepositoryEventConfigElPushEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerRepositoryEventConfigElPushEl {}
impl BuildCloudbuildTriggerRepositoryEventConfigElPushEl {
    pub fn build(self) -> CloudbuildTriggerRepositoryEventConfigElPushEl {
        CloudbuildTriggerRepositoryEventConfigElPushEl {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerRepositoryEventConfigElPushElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerRepositoryEventConfigElPushElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerRepositoryEventConfigElPushElRef {
        CloudbuildTriggerRepositoryEventConfigElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerRepositoryEventConfigElPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\nRegex of branches to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\nIf true, only trigger a build if the revision regex does NOT match the git_ref regex."]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nRegex of tags to match.\n\nThe syntax of the regular expressions accepted is the syntax accepted by\nRE2 and described at https://github.com/google/re2/wiki/Syntax"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudbuildTriggerRepositoryEventConfigElDynamic {
    pull_request: Option<DynamicBlock<CloudbuildTriggerRepositoryEventConfigElPullRequestEl>>,
    push: Option<DynamicBlock<CloudbuildTriggerRepositoryEventConfigElPushEl>>,
}
#[derive(Serialize)]
pub struct CloudbuildTriggerRepositoryEventConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<Vec<CloudbuildTriggerRepositoryEventConfigElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<Vec<CloudbuildTriggerRepositoryEventConfigElPushEl>>,
    dynamic: CloudbuildTriggerRepositoryEventConfigElDynamic,
}
impl CloudbuildTriggerRepositoryEventConfigEl {
    #[doc = "Set the field `repository`.\nThe resource name of the Repo API resource."]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerRepositoryEventConfigElPullRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pull_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pull_request = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl Into<BlockAssignable<CloudbuildTriggerRepositoryEventConfigElPushEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.push = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.push = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudbuildTriggerRepositoryEventConfigEl {
    type O = BlockAssignable<CloudbuildTriggerRepositoryEventConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerRepositoryEventConfigEl {}
impl BuildCloudbuildTriggerRepositoryEventConfigEl {
    pub fn build(self) -> CloudbuildTriggerRepositoryEventConfigEl {
        CloudbuildTriggerRepositoryEventConfigEl {
            repository: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudbuildTriggerRepositoryEventConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerRepositoryEventConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerRepositoryEventConfigElRef {
        CloudbuildTriggerRepositoryEventConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerRepositoryEventConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nThe resource name of the Repo API resource."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(
        &self,
    ) -> ListRef<CloudbuildTriggerRepositoryEventConfigElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }
    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> ListRef<CloudbuildTriggerRepositoryEventConfigElPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerSourceToBuildEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket_server_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_enterprise_config: Option<PrimField<String>>,
    #[serde(rename = "ref")]
    ref_: PrimField<String>,
    repo_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl CloudbuildTriggerSourceToBuildEl {
    #[doc = "Set the field `bitbucket_server_config`.\nThe full resource name of the bitbucket server config.\nFormat: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}."]
    pub fn set_bitbucket_server_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bitbucket_server_config = Some(v.into());
        self
    }
    #[doc = "Set the field `github_enterprise_config`.\nThe full resource name of the github enterprise config.\nFormat: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}."]
    pub fn set_github_enterprise_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.github_enterprise_config = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\nThe qualified resource name of the Repo API repository.\nEither uri or repository can be specified and is required."]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\nThe URI of the repo."]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerSourceToBuildEl {
    type O = BlockAssignable<CloudbuildTriggerSourceToBuildEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerSourceToBuildEl {
    #[doc = "The branch or tag to use. Must start with \"refs/\" (required)."]
    pub ref_: PrimField<String>,
    #[doc = "The type of the repo, since it may not be explicit from the repo field (e.g from a URL).\nValues can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER Possible values: [\"UNKNOWN\", \"CLOUD_SOURCE_REPOSITORIES\", \"GITHUB\", \"BITBUCKET_SERVER\"]"]
    pub repo_type: PrimField<String>,
}
impl BuildCloudbuildTriggerSourceToBuildEl {
    pub fn build(self) -> CloudbuildTriggerSourceToBuildEl {
        CloudbuildTriggerSourceToBuildEl {
            bitbucket_server_config: core::default::Default::default(),
            github_enterprise_config: core::default::Default::default(),
            ref_: self.ref_,
            repo_type: self.repo_type,
            repository: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerSourceToBuildElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerSourceToBuildElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerSourceToBuildElRef {
        CloudbuildTriggerSourceToBuildElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerSourceToBuildElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitbucket_server_config` after provisioning.\nThe full resource name of the bitbucket server config.\nFormat: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}."]
    pub fn bitbucket_server_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\nThe full resource name of the github enterprise config.\nFormat: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}."]
    pub fn github_enterprise_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.github_enterprise_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ref_` after provisioning.\nThe branch or tag to use. Must start with \"refs/\" (required)."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_type` after provisioning.\nThe type of the repo, since it may not be explicit from the repo field (e.g from a URL).\nValues can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER Possible values: [\"UNKNOWN\", \"CLOUD_SOURCE_REPOSITORIES\", \"GITHUB\", \"BITBUCKET_SERVER\"]"]
    pub fn repo_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_type", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nThe qualified resource name of the Repo API repository.\nEither uri or repository can be specified and is required."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe URI of the repo."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudbuildTriggerTimeoutsEl {
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
impl ToListMappable for CloudbuildTriggerTimeoutsEl {
    type O = BlockAssignable<CloudbuildTriggerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerTimeoutsEl {}
impl BuildCloudbuildTriggerTimeoutsEl {
    pub fn build(self) -> CloudbuildTriggerTimeoutsEl {
        CloudbuildTriggerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerTimeoutsElRef {
        CloudbuildTriggerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerTimeoutsElRef {
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
pub struct CloudbuildTriggerTriggerTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_sha: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name: Option<PrimField<String>>,
}
impl CloudbuildTriggerTriggerTemplateEl {
    #[doc = "Set the field `branch_name`.\nName of the branch to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThis field is a regular expression."]
    pub fn set_branch_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch_name = Some(v.into());
        self
    }
    #[doc = "Set the field `commit_sha`.\nExplicit commit SHA to build. Exactly one of a branch name, tag, or commit SHA must be provided."]
    pub fn set_commit_sha(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_sha = Some(v.into());
        self
    }
    #[doc = "Set the field `dir`.\nDirectory, relative to the source root, in which to run the build.\n\nThis must be a relative path. If a step's dir is specified and\nis an absolute path, this value is ignored for that step's\nexecution."]
    pub fn set_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dir = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\nOnly trigger a build if the revision regex does NOT match the revision regex."]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nID of the project that owns the Cloud Source Repository. If\nomitted, the project ID requesting the build is assumed."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `repo_name`.\nName of the Cloud Source Repository. If omitted, the name \"default\" is assumed."]
    pub fn set_repo_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_name`.\nName of the tag to build. Exactly one of a branch name, tag, or commit SHA must be provided.\nThis field is a regular expression."]
    pub fn set_tag_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_name = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildTriggerTriggerTemplateEl {
    type O = BlockAssignable<CloudbuildTriggerTriggerTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerTriggerTemplateEl {}
impl BuildCloudbuildTriggerTriggerTemplateEl {
    pub fn build(self) -> CloudbuildTriggerTriggerTemplateEl {
        CloudbuildTriggerTriggerTemplateEl {
            branch_name: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            dir: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            project_id: core::default::Default::default(),
            repo_name: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildTriggerTriggerTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerTriggerTemplateElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerTriggerTemplateElRef {
        CloudbuildTriggerTriggerTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerTriggerTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\nName of the branch to build. Exactly one a of branch name, tag, or commit SHA must be provided.\nThis field is a regular expression."]
    pub fn branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }
    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\nExplicit commit SHA to build. Exactly one of a branch name, tag, or commit SHA must be provided."]
    pub fn commit_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }
    #[doc = "Get a reference to the value of field `dir` after provisioning.\nDirectory, relative to the source root, in which to run the build.\n\nThis must be a relative path. If a step's dir is specified and\nis an absolute path, this value is ignored for that step's\nexecution."]
    pub fn dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\nOnly trigger a build if the revision regex does NOT match the revision regex."]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nID of the project that owns the Cloud Source Repository. If\nomitted, the project ID requesting the build is assumed."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_name` after provisioning.\nName of the Cloud Source Repository. If omitted, the name \"default\" is assumed."]
    pub fn repo_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_name` after provisioning.\nName of the tag to build. Exactly one of a branch name, tag, or commit SHA must be provided.\nThis field is a regular expression."]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildTriggerWebhookConfigEl {
    secret: PrimField<String>,
}
impl CloudbuildTriggerWebhookConfigEl {}
impl ToListMappable for CloudbuildTriggerWebhookConfigEl {
    type O = BlockAssignable<CloudbuildTriggerWebhookConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildTriggerWebhookConfigEl {
    #[doc = "Resource name for the secret required as a URL parameter."]
    pub secret: PrimField<String>,
}
impl BuildCloudbuildTriggerWebhookConfigEl {
    pub fn build(self) -> CloudbuildTriggerWebhookConfigEl {
        CloudbuildTriggerWebhookConfigEl {
            secret: self.secret,
        }
    }
}
pub struct CloudbuildTriggerWebhookConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildTriggerWebhookConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildTriggerWebhookConfigElRef {
        CloudbuildTriggerWebhookConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildTriggerWebhookConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nResource name for the secret required as a URL parameter."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nPotential issues with the underlying Pub/Sub subscription configuration.\nOnly populated on get requests."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudbuildTriggerDynamic {
    approval_config: Option<DynamicBlock<CloudbuildTriggerApprovalConfigEl>>,
    bitbucket_server_trigger_config:
        Option<DynamicBlock<CloudbuildTriggerBitbucketServerTriggerConfigEl>>,
    build: Option<DynamicBlock<CloudbuildTriggerBuildEl>>,
    developer_connect_event_config:
        Option<DynamicBlock<CloudbuildTriggerDeveloperConnectEventConfigEl>>,
    git_file_source: Option<DynamicBlock<CloudbuildTriggerGitFileSourceEl>>,
    github: Option<DynamicBlock<CloudbuildTriggerGithubEl>>,
    pubsub_config: Option<DynamicBlock<CloudbuildTriggerPubsubConfigEl>>,
    repository_event_config: Option<DynamicBlock<CloudbuildTriggerRepositoryEventConfigEl>>,
    source_to_build: Option<DynamicBlock<CloudbuildTriggerSourceToBuildEl>>,
    trigger_template: Option<DynamicBlock<CloudbuildTriggerTriggerTemplateEl>>,
    webhook_config: Option<DynamicBlock<CloudbuildTriggerWebhookConfigEl>>,
}
