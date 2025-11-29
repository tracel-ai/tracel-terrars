use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AmplifyAppData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_branch_creation_patterns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_credentials: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_spec: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_headers: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_auto_branch_creation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_basic_auth: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_branch_auto_build: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_branch_auto_deletion: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_service_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_branch_creation_config: Option<Vec<AmplifyAppAutoBranchCreationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_config: Option<Vec<AmplifyAppCacheConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_rule: Option<Vec<AmplifyAppCustomRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_config: Option<Vec<AmplifyAppJobConfigEl>>,
    dynamic: AmplifyAppDynamic,
}

struct AmplifyApp_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AmplifyAppData>,
}

#[derive(Clone)]
pub struct AmplifyApp(Rc<AmplifyApp_>);

impl AmplifyApp {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_token = Some(v.into());
        self
    }

    #[doc = "Set the field `auto_branch_creation_patterns`.\n"]
    pub fn set_auto_branch_creation_patterns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().auto_branch_creation_patterns = Some(v.into());
        self
    }

    #[doc = "Set the field `basic_auth_credentials`.\n"]
    pub fn set_basic_auth_credentials(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().basic_auth_credentials = Some(v.into());
        self
    }

    #[doc = "Set the field `build_spec`.\n"]
    pub fn set_build_spec(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().build_spec = Some(v.into());
        self
    }

    #[doc = "Set the field `compute_role_arn`.\n"]
    pub fn set_compute_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().compute_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_headers`.\n"]
    pub fn set_custom_headers(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_headers = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_auto_branch_creation`.\n"]
    pub fn set_enable_auto_branch_creation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_auto_branch_creation = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_basic_auth`.\n"]
    pub fn set_enable_basic_auth(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_basic_auth = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_branch_auto_build`.\n"]
    pub fn set_enable_branch_auto_build(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_branch_auto_build = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_branch_auto_deletion`.\n"]
    pub fn set_enable_branch_auto_deletion(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_branch_auto_deletion = Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().environment_variables = Some(v.into());
        self
    }

    #[doc = "Set the field `iam_service_role_arn`.\n"]
    pub fn set_iam_service_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_service_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_token`.\n"]
    pub fn set_oauth_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().oauth_token = Some(v.into());
        self
    }

    #[doc = "Set the field `platform`.\n"]
    pub fn set_platform(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().platform = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().repository = Some(v.into());
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

    #[doc = "Set the field `auto_branch_creation_config`.\n"]
    pub fn set_auto_branch_creation_config(
        self,
        v: impl Into<BlockAssignable<AmplifyAppAutoBranchCreationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_branch_creation_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_branch_creation_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cache_config`.\n"]
    pub fn set_cache_config(self, v: impl Into<BlockAssignable<AmplifyAppCacheConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_rule`.\n"]
    pub fn set_custom_rule(self, v: impl Into<BlockAssignable<AmplifyAppCustomRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_rule = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job_config`.\n"]
    pub fn set_job_config(self, v: impl Into<BlockAssignable<AmplifyAppJobConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().job_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.job_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_branch_creation_patterns` after provisioning.\n"]
    pub fn auto_branch_creation_patterns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.auto_branch_creation_patterns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_auth_credentials` after provisioning.\n"]
    pub fn basic_auth_credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.basic_auth_credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_spec` after provisioning.\n"]
    pub fn build_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_role_arn` after provisioning.\n"]
    pub fn compute_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers` after provisioning.\n"]
    pub fn custom_headers(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_domain` after provisioning.\n"]
    pub fn default_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_auto_branch_creation` after provisioning.\n"]
    pub fn enable_auto_branch_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_auto_branch_creation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_basic_auth` after provisioning.\n"]
    pub fn enable_basic_auth(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_basic_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_branch_auto_build` after provisioning.\n"]
    pub fn enable_branch_auto_build(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_branch_auto_build", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_branch_auto_deletion` after provisioning.\n"]
    pub fn enable_branch_auto_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_branch_auto_deletion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_service_role_arn` after provisioning.\n"]
    pub fn iam_service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oauth_token` after provisioning.\n"]
    pub fn oauth_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oauth_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `production_branch` after provisioning.\n"]
    pub fn production_branch(&self) -> ListRef<AmplifyAppProductionBranchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.production_branch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_branch_creation_config` after provisioning.\n"]
    pub fn auto_branch_creation_config(&self) -> ListRef<AmplifyAppAutoBranchCreationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_branch_creation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_config` after provisioning.\n"]
    pub fn cache_config(&self) -> ListRef<AmplifyAppCacheConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_rule` after provisioning.\n"]
    pub fn custom_rule(&self) -> ListRef<AmplifyAppCustomRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_config` after provisioning.\n"]
    pub fn job_config(&self) -> ListRef<AmplifyAppJobConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.job_config", self.extract_ref()))
    }
}

impl Referable for AmplifyApp {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AmplifyApp { }

impl ToListMappable for AmplifyApp {
    type O = ListRef<AmplifyAppRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AmplifyApp_ {
    fn extract_resource_type(&self) -> String {
        "aws_amplify_app".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAmplifyApp {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAmplifyApp {
    pub fn build(self, stack: &mut Stack) -> AmplifyApp {
        let out = AmplifyApp(Rc::new(AmplifyApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AmplifyAppData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_token: core::default::Default::default(),
                auto_branch_creation_patterns: core::default::Default::default(),
                basic_auth_credentials: core::default::Default::default(),
                build_spec: core::default::Default::default(),
                compute_role_arn: core::default::Default::default(),
                custom_headers: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_auto_branch_creation: core::default::Default::default(),
                enable_basic_auth: core::default::Default::default(),
                enable_branch_auto_build: core::default::Default::default(),
                enable_branch_auto_deletion: core::default::Default::default(),
                environment_variables: core::default::Default::default(),
                iam_service_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                oauth_token: core::default::Default::default(),
                platform: core::default::Default::default(),
                region: core::default::Default::default(),
                repository: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                auto_branch_creation_config: core::default::Default::default(),
                cache_config: core::default::Default::default(),
                custom_rule: core::default::Default::default(),
                job_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AmplifyAppRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmplifyAppRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AmplifyAppRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_branch_creation_patterns` after provisioning.\n"]
    pub fn auto_branch_creation_patterns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.auto_branch_creation_patterns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_auth_credentials` after provisioning.\n"]
    pub fn basic_auth_credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.basic_auth_credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_spec` after provisioning.\n"]
    pub fn build_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_role_arn` after provisioning.\n"]
    pub fn compute_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers` after provisioning.\n"]
    pub fn custom_headers(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_domain` after provisioning.\n"]
    pub fn default_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_auto_branch_creation` after provisioning.\n"]
    pub fn enable_auto_branch_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_auto_branch_creation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_basic_auth` after provisioning.\n"]
    pub fn enable_basic_auth(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_basic_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_branch_auto_build` after provisioning.\n"]
    pub fn enable_branch_auto_build(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_branch_auto_build", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_branch_auto_deletion` after provisioning.\n"]
    pub fn enable_branch_auto_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_branch_auto_deletion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_service_role_arn` after provisioning.\n"]
    pub fn iam_service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oauth_token` after provisioning.\n"]
    pub fn oauth_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oauth_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `production_branch` after provisioning.\n"]
    pub fn production_branch(&self) -> ListRef<AmplifyAppProductionBranchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.production_branch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_branch_creation_config` after provisioning.\n"]
    pub fn auto_branch_creation_config(&self) -> ListRef<AmplifyAppAutoBranchCreationConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_branch_creation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_config` after provisioning.\n"]
    pub fn cache_config(&self) -> ListRef<AmplifyAppCacheConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_rule` after provisioning.\n"]
    pub fn custom_rule(&self) -> ListRef<AmplifyAppCustomRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_config` after provisioning.\n"]
    pub fn job_config(&self) -> ListRef<AmplifyAppJobConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.job_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AmplifyAppProductionBranchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_deploy_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_url: Option<PrimField<String>>,
}

impl AmplifyAppProductionBranchEl {
    #[doc = "Set the field `branch_name`.\n"]
    pub fn set_branch_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch_name = Some(v.into());
        self
    }

    #[doc = "Set the field `last_deploy_time`.\n"]
    pub fn set_last_deploy_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_deploy_time = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `thumbnail_url`.\n"]
    pub fn set_thumbnail_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.thumbnail_url = Some(v.into());
        self
    }
}

impl ToListMappable for AmplifyAppProductionBranchEl {
    type O = BlockAssignable<AmplifyAppProductionBranchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyAppProductionBranchEl {}

impl BuildAmplifyAppProductionBranchEl {
    pub fn build(self) -> AmplifyAppProductionBranchEl {
        AmplifyAppProductionBranchEl {
            branch_name: core::default::Default::default(),
            last_deploy_time: core::default::Default::default(),
            status: core::default::Default::default(),
            thumbnail_url: core::default::Default::default(),
        }
    }
}

pub struct AmplifyAppProductionBranchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmplifyAppProductionBranchElRef {
    fn new(shared: StackShared, base: String) -> AmplifyAppProductionBranchElRef {
        AmplifyAppProductionBranchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyAppProductionBranchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc = "Get a reference to the value of field `last_deploy_time` after provisioning.\n"]
    pub fn last_deploy_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_deploy_time", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `thumbnail_url` after provisioning.\n"]
    pub fn thumbnail_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.thumbnail_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AmplifyAppAutoBranchCreationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_credentials: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_spec: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_auto_build: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_basic_auth: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_performance_mode: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_pull_request_preview: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    framework: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request_environment_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<PrimField<String>>,
}

impl AmplifyAppAutoBranchCreationConfigEl {
    #[doc = "Set the field `basic_auth_credentials`.\n"]
    pub fn set_basic_auth_credentials(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.basic_auth_credentials = Some(v.into());
        self
    }

    #[doc = "Set the field `build_spec`.\n"]
    pub fn set_build_spec(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_spec = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_auto_build`.\n"]
    pub fn set_enable_auto_build(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_auto_build = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_basic_auth`.\n"]
    pub fn set_enable_basic_auth(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_basic_auth = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_performance_mode`.\n"]
    pub fn set_enable_performance_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_performance_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_pull_request_preview`.\n"]
    pub fn set_enable_pull_request_preview(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_pull_request_preview = Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment_variables = Some(v.into());
        self
    }

    #[doc = "Set the field `framework`.\n"]
    pub fn set_framework(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.framework = Some(v.into());
        self
    }

    #[doc = "Set the field `pull_request_environment_name`.\n"]
    pub fn set_pull_request_environment_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pull_request_environment_name = Some(v.into());
        self
    }

    #[doc = "Set the field `stage`.\n"]
    pub fn set_stage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stage = Some(v.into());
        self
    }
}

impl ToListMappable for AmplifyAppAutoBranchCreationConfigEl {
    type O = BlockAssignable<AmplifyAppAutoBranchCreationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyAppAutoBranchCreationConfigEl {}

impl BuildAmplifyAppAutoBranchCreationConfigEl {
    pub fn build(self) -> AmplifyAppAutoBranchCreationConfigEl {
        AmplifyAppAutoBranchCreationConfigEl {
            basic_auth_credentials: core::default::Default::default(),
            build_spec: core::default::Default::default(),
            enable_auto_build: core::default::Default::default(),
            enable_basic_auth: core::default::Default::default(),
            enable_performance_mode: core::default::Default::default(),
            enable_pull_request_preview: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            framework: core::default::Default::default(),
            pull_request_environment_name: core::default::Default::default(),
            stage: core::default::Default::default(),
        }
    }
}

pub struct AmplifyAppAutoBranchCreationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmplifyAppAutoBranchCreationConfigElRef {
    fn new(shared: StackShared, base: String) -> AmplifyAppAutoBranchCreationConfigElRef {
        AmplifyAppAutoBranchCreationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyAppAutoBranchCreationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `basic_auth_credentials` after provisioning.\n"]
    pub fn basic_auth_credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.basic_auth_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `build_spec` after provisioning.\n"]
    pub fn build_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_auto_build` after provisioning.\n"]
    pub fn enable_auto_build(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_auto_build", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_basic_auth` after provisioning.\n"]
    pub fn enable_basic_auth(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_basic_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_performance_mode` after provisioning.\n"]
    pub fn enable_performance_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_performance_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_pull_request_preview` after provisioning.\n"]
    pub fn enable_pull_request_preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_pull_request_preview", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `framework` after provisioning.\n"]
    pub fn framework(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.framework", self.base))
    }

    #[doc = "Get a reference to the value of field `pull_request_environment_name` after provisioning.\n"]
    pub fn pull_request_environment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pull_request_environment_name", self.base))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }
}

#[derive(Serialize)]
pub struct AmplifyAppCacheConfigEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl AmplifyAppCacheConfigEl { }

impl ToListMappable for AmplifyAppCacheConfigEl {
    type O = BlockAssignable<AmplifyAppCacheConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyAppCacheConfigEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildAmplifyAppCacheConfigEl {
    pub fn build(self) -> AmplifyAppCacheConfigEl {
        AmplifyAppCacheConfigEl { type_: self.type_ }
    }
}

pub struct AmplifyAppCacheConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmplifyAppCacheConfigElRef {
    fn new(shared: StackShared, base: String) -> AmplifyAppCacheConfigElRef {
        AmplifyAppCacheConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyAppCacheConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct AmplifyAppCustomRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<PrimField<String>>,
    source: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    target: PrimField<String>,
}

impl AmplifyAppCustomRuleEl {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.condition = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for AmplifyAppCustomRuleEl {
    type O = BlockAssignable<AmplifyAppCustomRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyAppCustomRuleEl {
    #[doc = ""]
    pub source: PrimField<String>,
    #[doc = ""]
    pub target: PrimField<String>,
}

impl BuildAmplifyAppCustomRuleEl {
    pub fn build(self) -> AmplifyAppCustomRuleEl {
        AmplifyAppCustomRuleEl {
            condition: core::default::Default::default(),
            source: self.source,
            status: core::default::Default::default(),
            target: self.target,
        }
    }
}

pub struct AmplifyAppCustomRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmplifyAppCustomRuleElRef {
    fn new(shared: StackShared, base: String) -> AmplifyAppCustomRuleElRef {
        AmplifyAppCustomRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyAppCustomRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct AmplifyAppJobConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_compute_type: Option<PrimField<String>>,
}

impl AmplifyAppJobConfigEl {
    #[doc = "Set the field `build_compute_type`.\n"]
    pub fn set_build_compute_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_compute_type = Some(v.into());
        self
    }
}

impl ToListMappable for AmplifyAppJobConfigEl {
    type O = BlockAssignable<AmplifyAppJobConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyAppJobConfigEl {}

impl BuildAmplifyAppJobConfigEl {
    pub fn build(self) -> AmplifyAppJobConfigEl {
        AmplifyAppJobConfigEl { build_compute_type: core::default::Default::default() }
    }
}

pub struct AmplifyAppJobConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmplifyAppJobConfigElRef {
    fn new(shared: StackShared, base: String) -> AmplifyAppJobConfigElRef {
        AmplifyAppJobConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyAppJobConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `build_compute_type` after provisioning.\n"]
    pub fn build_compute_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_compute_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AmplifyAppDynamic {
    auto_branch_creation_config: Option<DynamicBlock<AmplifyAppAutoBranchCreationConfigEl>>,
    cache_config: Option<DynamicBlock<AmplifyAppCacheConfigEl>>,
    custom_rule: Option<DynamicBlock<AmplifyAppCustomRuleEl>>,
    job_config: Option<DynamicBlock<AmplifyAppJobConfigEl>>,
}
