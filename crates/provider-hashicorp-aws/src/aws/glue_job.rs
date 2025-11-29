use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connections: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_arguments: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    glue_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_run_queuing_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retries: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_overridable_arguments: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_workers: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_configuration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<Vec<GlueJobCommandEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_property: Option<Vec<GlueJobExecutionPropertyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_property: Option<Vec<GlueJobNotificationPropertyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_control_details: Option<Vec<GlueJobSourceControlDetailsEl>>,
    dynamic: GlueJobDynamic,
}

struct GlueJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlueJobData>,
}

#[derive(Clone)]
pub struct GlueJob(Rc<GlueJob_>);

impl GlueJob {
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

    #[doc = "Set the field `connections`.\n"]
    pub fn set_connections(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().connections = Some(v.into());
        self
    }

    #[doc = "Set the field `default_arguments`.\n"]
    pub fn set_default_arguments(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().default_arguments = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_class`.\n"]
    pub fn set_execution_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().execution_class = Some(v.into());
        self
    }

    #[doc = "Set the field `glue_version`.\n"]
    pub fn set_glue_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().glue_version = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `job_mode`.\n"]
    pub fn set_job_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().job_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `job_run_queuing_enabled`.\n"]
    pub fn set_job_run_queuing_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().job_run_queuing_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maintenance_window = Some(v.into());
        self
    }

    #[doc = "Set the field `max_capacity`.\n"]
    pub fn set_max_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_retries = Some(v.into());
        self
    }

    #[doc = "Set the field `non_overridable_arguments`.\n"]
    pub fn set_non_overridable_arguments(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().non_overridable_arguments = Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_workers`.\n"]
    pub fn set_number_of_workers(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().number_of_workers = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `security_configuration`.\n"]
    pub fn set_security_configuration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_configuration = Some(v.into());
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

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `worker_type`.\n"]
    pub fn set_worker_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().worker_type = Some(v.into());
        self
    }

    #[doc = "Set the field `command`.\n"]
    pub fn set_command(self, v: impl Into<BlockAssignable<GlueJobCommandEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().command = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.command = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `execution_property`.\n"]
    pub fn set_execution_property(self, v: impl Into<BlockAssignable<GlueJobExecutionPropertyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_property = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_property = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_property`.\n"]
    pub fn set_notification_property(self, v: impl Into<BlockAssignable<GlueJobNotificationPropertyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_property = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_property = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_control_details`.\n"]
    pub fn set_source_control_details(self, v: impl Into<BlockAssignable<GlueJobSourceControlDetailsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_control_details = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_control_details = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connections` after provisioning.\n"]
    pub fn connections(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.connections", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_arguments` after provisioning.\n"]
    pub fn default_arguments(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.default_arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_class` after provisioning.\n"]
    pub fn execution_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glue_version` after provisioning.\n"]
    pub fn glue_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.glue_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_mode` after provisioning.\n"]
    pub fn job_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_run_queuing_enabled` after provisioning.\n"]
    pub fn job_run_queuing_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_run_queuing_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `non_overridable_arguments` after provisioning.\n"]
    pub fn non_overridable_arguments(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.non_overridable_arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_workers` after provisioning.\n"]
    pub fn number_of_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_workers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_type` after provisioning.\n"]
    pub fn worker_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<GlueJobCommandElRef> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_property` after provisioning.\n"]
    pub fn execution_property(&self) -> ListRef<GlueJobExecutionPropertyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.execution_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_property` after provisioning.\n"]
    pub fn notification_property(&self) -> ListRef<GlueJobNotificationPropertyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_control_details` after provisioning.\n"]
    pub fn source_control_details(&self) -> ListRef<GlueJobSourceControlDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_control_details", self.extract_ref()))
    }
}

impl Referable for GlueJob {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GlueJob { }

impl ToListMappable for GlueJob {
    type O = ListRef<GlueJobRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GlueJob_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_job".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueJob {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildGlueJob {
    pub fn build(self, stack: &mut Stack) -> GlueJob {
        let out = GlueJob(Rc::new(GlueJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlueJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                connections: core::default::Default::default(),
                default_arguments: core::default::Default::default(),
                description: core::default::Default::default(),
                execution_class: core::default::Default::default(),
                glue_version: core::default::Default::default(),
                id: core::default::Default::default(),
                job_mode: core::default::Default::default(),
                job_run_queuing_enabled: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                max_capacity: core::default::Default::default(),
                max_retries: core::default::Default::default(),
                name: self.name,
                non_overridable_arguments: core::default::Default::default(),
                number_of_workers: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                security_configuration: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeout: core::default::Default::default(),
                worker_type: core::default::Default::default(),
                command: core::default::Default::default(),
                execution_property: core::default::Default::default(),
                notification_property: core::default::Default::default(),
                source_control_details: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueJobRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl GlueJobRef {
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

    #[doc = "Get a reference to the value of field `connections` after provisioning.\n"]
    pub fn connections(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.connections", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_arguments` after provisioning.\n"]
    pub fn default_arguments(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.default_arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_class` after provisioning.\n"]
    pub fn execution_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glue_version` after provisioning.\n"]
    pub fn glue_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.glue_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_mode` after provisioning.\n"]
    pub fn job_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_run_queuing_enabled` after provisioning.\n"]
    pub fn job_run_queuing_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_run_queuing_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `non_overridable_arguments` after provisioning.\n"]
    pub fn non_overridable_arguments(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.non_overridable_arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_workers` after provisioning.\n"]
    pub fn number_of_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_workers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_type` after provisioning.\n"]
    pub fn worker_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<GlueJobCommandElRef> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_property` after provisioning.\n"]
    pub fn execution_property(&self) -> ListRef<GlueJobExecutionPropertyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.execution_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_property` after provisioning.\n"]
    pub fn notification_property(&self) -> ListRef<GlueJobNotificationPropertyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_control_details` after provisioning.\n"]
    pub fn source_control_details(&self) -> ListRef<GlueJobSourceControlDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_control_details", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GlueJobCommandEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime: Option<PrimField<String>>,
    script_location: PrimField<String>,
}

impl GlueJobCommandEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `python_version`.\n"]
    pub fn set_python_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_version = Some(v.into());
        self
    }

    #[doc = "Set the field `runtime`.\n"]
    pub fn set_runtime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.runtime = Some(v.into());
        self
    }
}

impl ToListMappable for GlueJobCommandEl {
    type O = BlockAssignable<GlueJobCommandEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueJobCommandEl {
    #[doc = ""]
    pub script_location: PrimField<String>,
}

impl BuildGlueJobCommandEl {
    pub fn build(self) -> GlueJobCommandEl {
        GlueJobCommandEl {
            name: core::default::Default::default(),
            python_version: core::default::Default::default(),
            runtime: core::default::Default::default(),
            script_location: self.script_location,
        }
    }
}

pub struct GlueJobCommandElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueJobCommandElRef {
    fn new(shared: StackShared, base: String) -> GlueJobCommandElRef {
        GlueJobCommandElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueJobCommandElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `python_version` after provisioning.\n"]
    pub fn python_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_version", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }

    #[doc = "Get a reference to the value of field `script_location` after provisioning.\n"]
    pub fn script_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.script_location", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueJobExecutionPropertyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_runs: Option<PrimField<f64>>,
}

impl GlueJobExecutionPropertyEl {
    #[doc = "Set the field `max_concurrent_runs`.\n"]
    pub fn set_max_concurrent_runs(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_runs = Some(v.into());
        self
    }
}

impl ToListMappable for GlueJobExecutionPropertyEl {
    type O = BlockAssignable<GlueJobExecutionPropertyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueJobExecutionPropertyEl {}

impl BuildGlueJobExecutionPropertyEl {
    pub fn build(self) -> GlueJobExecutionPropertyEl {
        GlueJobExecutionPropertyEl { max_concurrent_runs: core::default::Default::default() }
    }
}

pub struct GlueJobExecutionPropertyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueJobExecutionPropertyElRef {
    fn new(shared: StackShared, base: String) -> GlueJobExecutionPropertyElRef {
        GlueJobExecutionPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueJobExecutionPropertyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_concurrent_runs` after provisioning.\n"]
    pub fn max_concurrent_runs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_runs", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueJobNotificationPropertyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_delay_after: Option<PrimField<f64>>,
}

impl GlueJobNotificationPropertyEl {
    #[doc = "Set the field `notify_delay_after`.\n"]
    pub fn set_notify_delay_after(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.notify_delay_after = Some(v.into());
        self
    }
}

impl ToListMappable for GlueJobNotificationPropertyEl {
    type O = BlockAssignable<GlueJobNotificationPropertyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueJobNotificationPropertyEl {}

impl BuildGlueJobNotificationPropertyEl {
    pub fn build(self) -> GlueJobNotificationPropertyEl {
        GlueJobNotificationPropertyEl { notify_delay_after: core::default::Default::default() }
    }
}

pub struct GlueJobNotificationPropertyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueJobNotificationPropertyElRef {
    fn new(shared: StackShared, base: String) -> GlueJobNotificationPropertyElRef {
        GlueJobNotificationPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueJobNotificationPropertyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `notify_delay_after` after provisioning.\n"]
    pub fn notify_delay_after(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify_delay_after", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueJobSourceControlDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_commit_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
}

impl GlueJobSourceControlDetailsEl {
    #[doc = "Set the field `auth_strategy`.\n"]
    pub fn set_auth_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_strategy = Some(v.into());
        self
    }

    #[doc = "Set the field `auth_token`.\n"]
    pub fn set_auth_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_token = Some(v.into());
        self
    }

    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc = "Set the field `folder`.\n"]
    pub fn set_folder(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.folder = Some(v.into());
        self
    }

    #[doc = "Set the field `last_commit_id`.\n"]
    pub fn set_last_commit_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_commit_id = Some(v.into());
        self
    }

    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner = Some(v.into());
        self
    }

    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provider = Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
}

impl ToListMappable for GlueJobSourceControlDetailsEl {
    type O = BlockAssignable<GlueJobSourceControlDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueJobSourceControlDetailsEl {}

impl BuildGlueJobSourceControlDetailsEl {
    pub fn build(self) -> GlueJobSourceControlDetailsEl {
        GlueJobSourceControlDetailsEl {
            auth_strategy: core::default::Default::default(),
            auth_token: core::default::Default::default(),
            branch: core::default::Default::default(),
            folder: core::default::Default::default(),
            last_commit_id: core::default::Default::default(),
            owner: core::default::Default::default(),
            provider: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

pub struct GlueJobSourceControlDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueJobSourceControlDetailsElRef {
    fn new(shared: StackShared, base: String) -> GlueJobSourceControlDetailsElRef {
        GlueJobSourceControlDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueJobSourceControlDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_strategy` after provisioning.\n"]
    pub fn auth_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_token` after provisioning.\n"]
    pub fn auth_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_token", self.base))
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `folder` after provisioning.\n"]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder", self.base))
    }

    #[doc = "Get a reference to the value of field `last_commit_id` after provisioning.\n"]
    pub fn last_commit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_commit_id", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueJobDynamic {
    command: Option<DynamicBlock<GlueJobCommandEl>>,
    execution_property: Option<DynamicBlock<GlueJobExecutionPropertyEl>>,
    notification_property: Option<DynamicBlock<GlueJobNotificationPropertyEl>>,
    source_control_details: Option<DynamicBlock<GlueJobSourceControlDetailsEl>>,
}
