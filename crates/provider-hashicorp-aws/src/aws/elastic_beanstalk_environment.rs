use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ElasticBeanstalkEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cname_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    poll_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    solution_stack_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_ready_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    setting: Option<Vec<ElasticBeanstalkEnvironmentSettingEl>>,
    dynamic: ElasticBeanstalkEnvironmentDynamic,
}

struct ElasticBeanstalkEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ElasticBeanstalkEnvironmentData>,
}

#[derive(Clone)]
pub struct ElasticBeanstalkEnvironment(Rc<ElasticBeanstalkEnvironment_>);

impl ElasticBeanstalkEnvironment {
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

    #[doc = "Set the field `cname_prefix`.\n"]
    pub fn set_cname_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cname_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `platform_arn`.\n"]
    pub fn set_platform_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().platform_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `poll_interval`.\n"]
    pub fn set_poll_interval(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().poll_interval = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `solution_stack_name`.\n"]
    pub fn set_solution_stack_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().solution_stack_name = Some(v.into());
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

    #[doc = "Set the field `template_name`.\n"]
    pub fn set_template_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_name = Some(v.into());
        self
    }

    #[doc = "Set the field `tier`.\n"]
    pub fn set_tier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tier = Some(v.into());
        self
    }

    #[doc = "Set the field `version_label`.\n"]
    pub fn set_version_label(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_label = Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_ready_timeout`.\n"]
    pub fn set_wait_for_ready_timeout(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().wait_for_ready_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `setting`.\n"]
    pub fn set_setting(
        self,
        v: impl Into<BlockAssignable<ElasticBeanstalkEnvironmentSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.setting = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `all_settings` after provisioning.\n"]
    pub fn all_settings(&self) -> SetRef<ElasticBeanstalkEnvironmentAllSettingsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.all_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `application` after provisioning.\n"]
    pub fn application(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cname` after provisioning.\n"]
    pub fn cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cname", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cname_prefix` after provisioning.\n"]
    pub fn cname_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cname_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_url` after provisioning.\n"]
    pub fn endpoint_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instances", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_configurations` after provisioning.\n"]
    pub fn launch_configurations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_configurations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_arn` after provisioning.\n"]
    pub fn platform_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `poll_interval` after provisioning.\n"]
    pub fn poll_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.poll_interval", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `queues` after provisioning.\n"]
    pub fn queues(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.queues", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `solution_stack_name` after provisioning.\n"]
    pub fn solution_stack_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.solution_stack_name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `template_name` after provisioning.\n"]
    pub fn template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tier` after provisioning.\n"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.triggers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version_label` after provisioning.\n"]
    pub fn version_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_label", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `wait_for_ready_timeout` after provisioning.\n"]
    pub fn wait_for_ready_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_ready_timeout", self.extract_ref()),
        )
    }
}

impl Referable for ElasticBeanstalkEnvironment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ElasticBeanstalkEnvironment {}

impl ToListMappable for ElasticBeanstalkEnvironment {
    type O = ListRef<ElasticBeanstalkEnvironmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ElasticBeanstalkEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "aws_elastic_beanstalk_environment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildElasticBeanstalkEnvironment {
    pub tf_id: String,
    #[doc = ""]
    pub application: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildElasticBeanstalkEnvironment {
    pub fn build(self, stack: &mut Stack) -> ElasticBeanstalkEnvironment {
        let out = ElasticBeanstalkEnvironment(Rc::new(ElasticBeanstalkEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ElasticBeanstalkEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application: self.application,
                cname_prefix: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                platform_arn: core::default::Default::default(),
                poll_interval: core::default::Default::default(),
                region: core::default::Default::default(),
                solution_stack_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                template_name: core::default::Default::default(),
                tier: core::default::Default::default(),
                version_label: core::default::Default::default(),
                wait_for_ready_timeout: core::default::Default::default(),
                setting: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ElasticBeanstalkEnvironmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticBeanstalkEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ElasticBeanstalkEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_settings` after provisioning.\n"]
    pub fn all_settings(&self) -> SetRef<ElasticBeanstalkEnvironmentAllSettingsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.all_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `application` after provisioning.\n"]
    pub fn application(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cname` after provisioning.\n"]
    pub fn cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cname", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cname_prefix` after provisioning.\n"]
    pub fn cname_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cname_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_url` after provisioning.\n"]
    pub fn endpoint_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instances", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_configurations` after provisioning.\n"]
    pub fn launch_configurations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_configurations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `load_balancers` after provisioning.\n"]
    pub fn load_balancers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `platform_arn` after provisioning.\n"]
    pub fn platform_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `poll_interval` after provisioning.\n"]
    pub fn poll_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.poll_interval", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `queues` after provisioning.\n"]
    pub fn queues(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.queues", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `solution_stack_name` after provisioning.\n"]
    pub fn solution_stack_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.solution_stack_name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `template_name` after provisioning.\n"]
    pub fn template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tier` after provisioning.\n"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `triggers` after provisioning.\n"]
    pub fn triggers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.triggers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version_label` after provisioning.\n"]
    pub fn version_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_label", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `wait_for_ready_timeout` after provisioning.\n"]
    pub fn wait_for_ready_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_ready_timeout", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ElasticBeanstalkEnvironmentAllSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl ElasticBeanstalkEnvironmentAllSettingsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }

    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for ElasticBeanstalkEnvironmentAllSettingsEl {
    type O = BlockAssignable<ElasticBeanstalkEnvironmentAllSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticBeanstalkEnvironmentAllSettingsEl {}

impl BuildElasticBeanstalkEnvironmentAllSettingsEl {
    pub fn build(self) -> ElasticBeanstalkEnvironmentAllSettingsEl {
        ElasticBeanstalkEnvironmentAllSettingsEl {
            name: core::default::Default::default(),
            namespace: core::default::Default::default(),
            resource: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct ElasticBeanstalkEnvironmentAllSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticBeanstalkEnvironmentAllSettingsElRef {
    fn new(shared: StackShared, base: String) -> ElasticBeanstalkEnvironmentAllSettingsElRef {
        ElasticBeanstalkEnvironmentAllSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticBeanstalkEnvironmentAllSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticBeanstalkEnvironmentSettingEl {
    name: PrimField<String>,
    namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
    value: PrimField<String>,
}

impl ElasticBeanstalkEnvironmentSettingEl {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }
}

impl ToListMappable for ElasticBeanstalkEnvironmentSettingEl {
    type O = BlockAssignable<ElasticBeanstalkEnvironmentSettingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticBeanstalkEnvironmentSettingEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub namespace: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildElasticBeanstalkEnvironmentSettingEl {
    pub fn build(self) -> ElasticBeanstalkEnvironmentSettingEl {
        ElasticBeanstalkEnvironmentSettingEl {
            name: self.name,
            namespace: self.namespace,
            resource: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct ElasticBeanstalkEnvironmentSettingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ElasticBeanstalkEnvironmentSettingElRef {
    fn new(shared: StackShared, base: String) -> ElasticBeanstalkEnvironmentSettingElRef {
        ElasticBeanstalkEnvironmentSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticBeanstalkEnvironmentSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ElasticBeanstalkEnvironmentDynamic {
    setting: Option<DynamicBlock<ElasticBeanstalkEnvironmentSettingEl>>,
}
