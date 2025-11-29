use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Kinesisanalyticsv2ApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_stop: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    runtime_environment: PrimField<String>,
    service_execution_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_application: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_configuration: Option<Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logging_options: Option<Vec<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Kinesisanalyticsv2ApplicationTimeoutsEl>,
    dynamic: Kinesisanalyticsv2ApplicationDynamic,
}

struct Kinesisanalyticsv2Application_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Kinesisanalyticsv2ApplicationData>,
}

#[derive(Clone)]
pub struct Kinesisanalyticsv2Application(Rc<Kinesisanalyticsv2Application_>);

impl Kinesisanalyticsv2Application {
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

    #[doc = "Set the field `application_mode`.\n"]
    pub fn set_application_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().application_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `force_stop`.\n"]
    pub fn set_force_stop(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_stop = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `start_application`.\n"]
    pub fn set_start_application(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_application = Some(v.into());
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

    #[doc = "Set the field `application_configuration`.\n"]
    pub fn set_application_configuration(
        self,
        v: impl Into<BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().application_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.application_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        self,
        v: impl Into<BlockAssignable<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_logging_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_logging_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Kinesisanalyticsv2ApplicationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `application_mode` after provisioning.\n"]
    pub fn application_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_timestamp` after provisioning.\n"]
    pub fn create_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_stop` after provisioning.\n"]
    pub fn force_stop(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_stop", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_update_timestamp` after provisioning.\n"]
    pub fn last_update_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_update_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_environment` after provisioning.\n"]
    pub fn runtime_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runtime_environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_execution_role` after provisioning.\n"]
    pub fn service_execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_application` after provisioning.\n"]
    pub fn start_application(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_application", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_configuration` after provisioning.\n"]
    pub fn application_configuration(&self) -> ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.application_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(&self) -> ListRef<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Kinesisanalyticsv2ApplicationTimeoutsElRef {
        Kinesisanalyticsv2ApplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for Kinesisanalyticsv2Application {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Kinesisanalyticsv2Application { }

impl ToListMappable for Kinesisanalyticsv2Application {
    type O = ListRef<Kinesisanalyticsv2ApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Kinesisanalyticsv2Application_ {
    fn extract_resource_type(&self) -> String {
        "aws_kinesisanalyticsv2_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKinesisanalyticsv2Application {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub runtime_environment: PrimField<String>,
    #[doc = ""]
    pub service_execution_role: PrimField<String>,
}

impl BuildKinesisanalyticsv2Application {
    pub fn build(self, stack: &mut Stack) -> Kinesisanalyticsv2Application {
        let out = Kinesisanalyticsv2Application(Rc::new(Kinesisanalyticsv2Application_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Kinesisanalyticsv2ApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application_mode: core::default::Default::default(),
                description: core::default::Default::default(),
                force_stop: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                runtime_environment: self.runtime_environment,
                service_execution_role: self.service_execution_role,
                start_application: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                application_configuration: core::default::Default::default(),
                cloudwatch_logging_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Kinesisanalyticsv2ApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Kinesisanalyticsv2ApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_mode` after provisioning.\n"]
    pub fn application_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_timestamp` after provisioning.\n"]
    pub fn create_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_stop` after provisioning.\n"]
    pub fn force_stop(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_stop", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_update_timestamp` after provisioning.\n"]
    pub fn last_update_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_update_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_environment` after provisioning.\n"]
    pub fn runtime_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runtime_environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_execution_role` after provisioning.\n"]
    pub fn service_execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_application` after provisioning.\n"]
    pub fn start_application(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_application", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_configuration` after provisioning.\n"]
    pub fn application_configuration(&self) -> ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.application_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(&self) -> ListRef<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Kinesisanalyticsv2ApplicationTimeoutsElRef {
        Kinesisanalyticsv2ApplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
    bucket_arn: PrimField<String>,
    file_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_version: Option<PrimField<String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
    #[doc = "Set the field `object_version`.\n"]
    pub fn set_object_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object_version = Some(v.into());
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
    #[doc = ""]
    pub bucket_arn: PrimField<String>,
    #[doc = ""]
    pub file_key: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
            bucket_arn: self.bucket_arn,
            file_key: self.file_key,
            object_version: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `file_key` after provisioning.\n"]
    pub fn file_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_key", self.base))
    }

    #[doc = "Get a reference to the value of field `object_version` after provisioning.\n"]
    pub fn object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElDynamic {
    s3_content_location: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text_content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_content_location: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
    #[doc = "Set the field `text_content`.\n"]
    pub fn set_text_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text_content = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_content_location`.\n"]
    pub fn set_s3_content_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_content_location = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_content_location = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
            text_content: core::default::Default::default(),
            s3_content_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text_content` after provisioning.\n"]
    pub fn text_content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_content", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_content_location` after provisioning.\n"]
    pub fn s3_content_location(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.s3_content_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElDynamic {
    code_content: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
    code_content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_content: Option<
        Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl>,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
    #[doc = "Set the field `code_content`.\n"]
    pub fn set_code_content(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_content = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_content = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
    type O = BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
    #[doc = ""]
    pub code_content_type: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
            code_content_type: self.code_content_type,
            code_content: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code_content_type` after provisioning.\n"]
    pub fn code_content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code_content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `code_content` after provisioning.\n"]
    pub fn code_content(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.code_content", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
    snapshots_enabled: PrimField<bool>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl { }

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
    type O =
        BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
    #[doc = ""]
    pub snapshots_enabled: PrimField<bool>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
            snapshots_enabled: self.snapshots_enabled,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `snapshots_enabled` after provisioning.\n"]
    pub fn snapshots_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshots_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
    property_group_id: PrimField<String>,
    property_map: RecField<PrimField<String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl { }

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
    #[doc = ""]
    pub property_group_id: PrimField<String>,
    #[doc = ""]
    pub property_map: RecField<PrimField<String>>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
            property_group_id: self.property_group_id,
            property_map: self.property_map,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `property_group_id` after provisioning.\n"]
    pub fn property_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.property_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `property_map` after provisioning.\n"]
    pub fn property_map(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.property_map", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElDynamic {
    property_group: Option<
        DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl>,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    property_group: Option<
        Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl>,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
    #[doc = "Set the field `property_group`.\n"]
    pub fn set_property_group(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.property_group = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.property_group = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
    type O = BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
            property_group: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    checkpoint_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checkpointing_enabled: Option<PrimField<bool>>,
    configuration_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_pause_between_checkpoints: Option<PrimField<f64>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
    #[doc = "Set the field `checkpoint_interval`.\n"]
    pub fn set_checkpoint_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.checkpoint_interval = Some(v.into());
        self
    }

    #[doc = "Set the field `checkpointing_enabled`.\n"]
    pub fn set_checkpointing_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.checkpointing_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `min_pause_between_checkpoints`.\n"]
    pub fn set_min_pause_between_checkpoints(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_pause_between_checkpoints = Some(v.into());
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
    #[doc = ""]
    pub configuration_type: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
            checkpoint_interval: core::default::Default::default(),
            checkpointing_enabled: core::default::Default::default(),
            configuration_type: self.configuration_type,
            min_pause_between_checkpoints: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `checkpoint_interval` after provisioning.\n"]
    pub fn checkpoint_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.checkpoint_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `checkpointing_enabled` after provisioning.\n"]
    pub fn checkpointing_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.checkpointing_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration_type` after provisioning.\n"]
    pub fn configuration_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_type", self.base))
    }

    #[doc = "Get a reference to the value of field `min_pause_between_checkpoints` after provisioning.\n"]
    pub fn min_pause_between_checkpoints(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_pause_between_checkpoints", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
    configuration_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics_level: Option<PrimField<String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_level = Some(v.into());
        self
    }

    #[doc = "Set the field `metrics_level`.\n"]
    pub fn set_metrics_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metrics_level = Some(v.into());
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
    #[doc = ""]
    pub configuration_type: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
            configuration_type: self.configuration_type,
            log_level: core::default::Default::default(),
            metrics_level: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_type` after provisioning.\n"]
    pub fn configuration_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_type", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics_level` after provisioning.\n"]
    pub fn metrics_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metrics_level", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_scaling_enabled: Option<PrimField<bool>>,
    configuration_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelism: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelism_per_kpu: Option<PrimField<f64>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
    #[doc = "Set the field `auto_scaling_enabled`.\n"]
    pub fn set_auto_scaling_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_scaling_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `parallelism`.\n"]
    pub fn set_parallelism(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.parallelism = Some(v.into());
        self
    }

    #[doc = "Set the field `parallelism_per_kpu`.\n"]
    pub fn set_parallelism_per_kpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.parallelism_per_kpu = Some(v.into());
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
    #[doc = ""]
    pub configuration_type: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
            auto_scaling_enabled: core::default::Default::default(),
            configuration_type: self.configuration_type,
            parallelism: core::default::Default::default(),
            parallelism_per_kpu: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_scaling_enabled` after provisioning.\n"]
    pub fn auto_scaling_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration_type` after provisioning.\n"]
    pub fn configuration_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_type", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism` after provisioning.\n"]
    pub fn parallelism(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism_per_kpu` after provisioning.\n"]
    pub fn parallelism_per_kpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parallelism_per_kpu", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElDynamic {
    checkpoint_configuration: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl,
        >,
    >,
    monitoring_configuration: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl,
        >,
    >,
    parallelism_configuration: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    checkpoint_configuration: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_configuration: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelism_configuration: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
    #[doc = "Set the field `checkpoint_configuration`.\n"]
    pub fn set_checkpoint_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.checkpoint_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.checkpoint_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_configuration`.\n"]
    pub fn set_monitoring_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.monitoring_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parallelism_configuration`.\n"]
    pub fn set_parallelism_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parallelism_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parallelism_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
    type O = BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
            checkpoint_configuration: core::default::Default::default(),
            monitoring_configuration: core::default::Default::default(),
            parallelism_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `checkpoint_configuration` after provisioning.\n"]
    pub fn checkpoint_configuration(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.checkpoint_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_configuration` after provisioning.\n"]
    pub fn monitoring_configuration(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.monitoring_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism_configuration` after provisioning.\n"]
    pub fn parallelism_configuration(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.parallelism_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_restore_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_name: Option<PrimField<String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
    #[doc = "Set the field `application_restore_type`.\n"]
    pub fn set_application_restore_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_restore_type = Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_name`.\n"]
    pub fn set_snapshot_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_name = Some(v.into());
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
            application_restore_type: core::default::Default::default(),
            snapshot_name: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_restore_type` after provisioning.\n"]
    pub fn application_restore_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_restore_type", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_name` after provisioning.\n"]
    pub fn snapshot_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot_name", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_non_restored_state: Option<PrimField<bool>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
    #[doc = "Set the field `allow_non_restored_state`.\n"]
    pub fn set_allow_non_restored_state(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_non_restored_state = Some(v.into());
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
            allow_non_restored_state: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_non_restored_state` after provisioning.\n"]
    pub fn allow_non_restored_state(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_non_restored_state", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElDynamic {
    application_restore_configuration: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl,
        >,
    >,
    flink_run_configuration: Option<
        DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_restore_configuration: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    flink_run_configuration: Option<
        Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl>,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
    #[doc = "Set the field `application_restore_configuration`.\n"]
    pub fn set_application_restore_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.application_restore_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.application_restore_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `flink_run_configuration`.\n"]
    pub fn set_flink_run_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.flink_run_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.flink_run_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
    type O = BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
            application_restore_configuration: core::default::Default::default(),
            flink_run_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_restore_configuration` after provisioning.\n"]
    pub fn application_restore_configuration(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.application_restore_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `flink_run_configuration` after provisioning.\n"]
    pub fn flink_run_configuration(
        &self,
    ) -> ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.flink_run_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
            count: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
    resource_arn: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {

}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElDynamic {
    input_lambda_processor: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_lambda_processor: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
    #[doc = "Set the field `input_lambda_processor`.\n"]
    pub fn set_input_lambda_processor(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_lambda_processor = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_lambda_processor = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
            input_lambda_processor: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_lambda_processor` after provisioning.\n"]
    pub fn input_lambda_processor(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.input_lambda_processor", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping: Option<PrimField<String>>,
    name: PrimField<String>,
    sql_type: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
    #[doc = "Set the field `mapping`.\n"]
    pub fn set_mapping(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mapping = Some(v.into());
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub sql_type: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
            mapping: core::default::Default::default(),
            name: self.name,
            sql_type: self.sql_type,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mapping` after provisioning.\n"]
    pub fn mapping(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_type` after provisioning.\n"]
    pub fn sql_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql_type", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    record_column_delimiter: PrimField<String>,
    record_row_delimiter: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {

}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    #[doc = ""]
    pub record_column_delimiter: PrimField<String>,
    #[doc = ""]
    pub record_row_delimiter: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
            record_column_delimiter: self.record_column_delimiter,
            record_row_delimiter: self.record_row_delimiter,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_column_delimiter` after provisioning.\n"]
    pub fn record_column_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_column_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `record_row_delimiter` after provisioning.\n"]
    pub fn record_row_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_row_delimiter", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    record_row_path: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {

}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    #[doc = ""]
    pub record_row_path: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
            record_row_path: self.record_row_path,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_row_path` after provisioning.\n"]
    pub fn record_row_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_row_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElDynamic {
    csv_mapping_parameters: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >,
    >,
    json_mapping_parameters: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_mapping_parameters: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_mapping_parameters: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
    #[doc = "Set the field `csv_mapping_parameters`.\n"]
    pub fn set_csv_mapping_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.csv_mapping_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.csv_mapping_parameters = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json_mapping_parameters`.\n"]
    pub fn set_json_mapping_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json_mapping_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json_mapping_parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
            csv_mapping_parameters: core::default::Default::default(),
            json_mapping_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv_mapping_parameters` after provisioning.\n"]
    pub fn csv_mapping_parameters(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.csv_mapping_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `json_mapping_parameters` after provisioning.\n"]
    pub fn json_mapping_parameters(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.json_mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElDynamic {
    mapping_parameters: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
    record_format_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping_parameters: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
    #[doc = "Set the field `mapping_parameters`.\n"]
    pub fn set_mapping_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mapping_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mapping_parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
    #[doc = ""]
    pub record_format_type: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
            record_format_type: self.record_format_type,
            mapping_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_format_type", self.base))
    }

    #[doc = "Get a reference to the value of field `mapping_parameters` after provisioning.\n"]
    pub fn mapping_parameters(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElDynamic {
    record_column: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl,
        >,
    >,
    record_format: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    record_encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_column: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_format: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
    #[doc = "Set the field `record_encoding`.\n"]
    pub fn set_record_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.record_encoding = Some(v.into());
        self
    }

    #[doc = "Set the field `record_column`.\n"]
    pub fn set_record_column(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record_column = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record_column = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `record_format`.\n"]
    pub fn set_record_format(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record_format = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record_format = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
            record_encoding: core::default::Default::default(),
            record_column: core::default::Default::default(),
            record_format: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_encoding` after provisioning.\n"]
    pub fn record_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `record_column` after provisioning.\n"]
    pub fn record_column(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.record_column", self.base))
    }

    #[doc = "Get a reference to the value of field `record_format` after provisioning.\n"]
    pub fn record_format(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.record_format", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_starting_position: Option<PrimField<String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
    #[doc = "Set the field `input_starting_position`.\n"]
    pub fn set_input_starting_position(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_starting_position = Some(v.into());
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
            input_starting_position: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_starting_position` after provisioning.\n"]
    pub fn input_starting_position(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_starting_position", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
    resource_arn: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {

}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
    resource_arn: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {

}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElDynamic {
    input_parallelism: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl,
        >,
    >,
    input_processing_configuration: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl,
        >,
    >,
    input_schema: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl,
        >,
    >,
    input_starting_position_configuration: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl,
        >,
    >,
    kinesis_firehose_input: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl,
        >,
    >,
    kinesis_streams_input: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
    name_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_parallelism: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_processing_configuration: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_schema: Option<
        Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_starting_position_configuration: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_firehose_input: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_streams_input: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
    #[doc = "Set the field `input_parallelism`.\n"]
    pub fn set_input_parallelism(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_parallelism = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_parallelism = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_processing_configuration`.\n"]
    pub fn set_input_processing_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_processing_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_processing_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_schema`.\n"]
    pub fn set_input_schema(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_schema = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_schema = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_starting_position_configuration`.\n"]
    pub fn set_input_starting_position_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_starting_position_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_starting_position_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_firehose_input`.\n"]
    pub fn set_kinesis_firehose_input(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_firehose_input = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose_input = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_streams_input`.\n"]
    pub fn set_kinesis_streams_input(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_streams_input = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_streams_input = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
    type O =
        BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
    #[doc = ""]
    pub name_prefix: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
            name_prefix: self.name_prefix,
            input_parallelism: core::default::Default::default(),
            input_processing_configuration: core::default::Default::default(),
            input_schema: core::default::Default::default(),
            input_starting_position_configuration: core::default::Default::default(),
            kinesis_firehose_input: core::default::Default::default(),
            kinesis_streams_input: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `in_app_stream_names` after provisioning.\n"]
    pub fn in_app_stream_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.in_app_stream_names", self.base))
    }

    #[doc = "Get a reference to the value of field `input_id` after provisioning.\n"]
    pub fn input_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `input_parallelism` after provisioning.\n"]
    pub fn input_parallelism(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.input_parallelism", self.base))
    }

    #[doc = "Get a reference to the value of field `input_processing_configuration` after provisioning.\n"]
    pub fn input_processing_configuration(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.input_processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `input_schema` after provisioning.\n"]
    pub fn input_schema(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `input_starting_position_configuration` after provisioning.\n"]
    pub fn input_starting_position_configuration(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.input_starting_position_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_input` after provisioning.\n"]
    pub fn kinesis_firehose_input(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_input", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_streams_input` after provisioning.\n"]
    pub fn kinesis_streams_input(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_streams_input", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
    record_format_type: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {

}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
    #[doc = ""]
    pub record_format_type: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
            record_format_type: self.record_format_type,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_format_type", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
    resource_arn: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {

}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
    resource_arn: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {

}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
    resource_arn: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl { }

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDynamic {
    destination_schema: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl,
        >,
    >,
    kinesis_firehose_output: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl,
        >,
    >,
    kinesis_streams_output: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl,
        >,
    >,
    lambda_output: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_schema: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_firehose_output: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_streams_output: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_output: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
    #[doc = "Set the field `destination_schema`.\n"]
    pub fn set_destination_schema(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_schema = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_schema = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_firehose_output`.\n"]
    pub fn set_kinesis_firehose_output(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_firehose_output = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose_output = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_streams_output`.\n"]
    pub fn set_kinesis_streams_output(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_streams_output = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_streams_output = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_output`.\n"]
    pub fn set_lambda_output(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_output = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_output = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
            name: self.name,
            destination_schema: core::default::Default::default(),
            kinesis_firehose_output: core::default::Default::default(),
            kinesis_streams_output: core::default::Default::default(),
            lambda_output: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `output_id` after provisioning.\n"]
    pub fn output_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_id", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_schema` after provisioning.\n"]
    pub fn destination_schema(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.destination_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_output` after provisioning.\n"]
    pub fn kinesis_firehose_output(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_output", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_streams_output` after provisioning.\n"]
    pub fn kinesis_streams_output(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_streams_output", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_output` after provisioning.\n"]
    pub fn lambda_output(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.lambda_output", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping: Option<PrimField<String>>,
    name: PrimField<String>,
    sql_type: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
    #[doc = "Set the field `mapping`.\n"]
    pub fn set_mapping(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mapping = Some(v.into());
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub sql_type: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
            mapping: core::default::Default::default(),
            name: self.name,
            sql_type: self.sql_type,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mapping` after provisioning.\n"]
    pub fn mapping(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_type` after provisioning.\n"]
    pub fn sql_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql_type", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    record_column_delimiter: PrimField<String>,
    record_row_delimiter: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {

}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    #[doc = ""]
    pub record_column_delimiter: PrimField<String>,
    #[doc = ""]
    pub record_row_delimiter: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
            record_column_delimiter: self.record_column_delimiter,
            record_row_delimiter: self.record_row_delimiter,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_column_delimiter` after provisioning.\n"]
    pub fn record_column_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_column_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `record_row_delimiter` after provisioning.\n"]
    pub fn record_row_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_row_delimiter", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    record_row_path: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {

}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    #[doc = ""]
    pub record_row_path: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
            record_row_path: self.record_row_path,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_row_path` after provisioning.\n"]
    pub fn record_row_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_row_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElDynamic {
    csv_mapping_parameters: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >,
    >,
    json_mapping_parameters: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_mapping_parameters: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_mapping_parameters: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
    #[doc = "Set the field `csv_mapping_parameters`.\n"]
    pub fn set_csv_mapping_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.csv_mapping_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.csv_mapping_parameters = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json_mapping_parameters`.\n"]
    pub fn set_json_mapping_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json_mapping_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json_mapping_parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
            csv_mapping_parameters: core::default::Default::default(),
            json_mapping_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv_mapping_parameters` after provisioning.\n"]
    pub fn csv_mapping_parameters(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.csv_mapping_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `json_mapping_parameters` after provisioning.\n"]
    pub fn json_mapping_parameters(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.json_mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElDynamic {
    mapping_parameters: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
    record_format_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping_parameters: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
    #[doc = "Set the field `mapping_parameters`.\n"]
    pub fn set_mapping_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mapping_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mapping_parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
    #[doc = ""]
    pub record_format_type: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
            record_format_type: self.record_format_type,
            mapping_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_format_type", self.base))
    }

    #[doc = "Get a reference to the value of field `mapping_parameters` after provisioning.\n"]
    pub fn mapping_parameters(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElDynamic {
    record_column: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl,
        >,
    >,
    record_format: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    record_encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_column: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_format: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
    #[doc = "Set the field `record_encoding`.\n"]
    pub fn set_record_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.record_encoding = Some(v.into());
        self
    }

    #[doc = "Set the field `record_column`.\n"]
    pub fn set_record_column(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record_column = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record_column = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `record_format`.\n"]
    pub fn set_record_format(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record_format = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record_format = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
            record_encoding: core::default::Default::default(),
            record_column: core::default::Default::default(),
            record_format: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_encoding` after provisioning.\n"]
    pub fn record_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `record_column` after provisioning.\n"]
    pub fn record_column(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.record_column", self.base))
    }

    #[doc = "Get a reference to the value of field `record_format` after provisioning.\n"]
    pub fn record_format(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.record_format", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
    bucket_arn: PrimField<String>,
    file_key: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {

}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
    #[doc = ""]
    pub bucket_arn: PrimField<String>,
    #[doc = ""]
    pub file_key: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
            bucket_arn: self.bucket_arn,
            file_key: self.file_key,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `file_key` after provisioning.\n"]
    pub fn file_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElDynamic {
    reference_schema: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl,
        >,
    >,
    s3_reference_data_source: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_schema: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_reference_data_source: Option<
        Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
    #[doc = "Set the field `reference_schema`.\n"]
    pub fn set_reference_schema(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reference_schema = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reference_schema = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_reference_data_source`.\n"]
    pub fn set_s3_reference_data_source(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_reference_data_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_reference_data_source = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
    type O =
        BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
    #[doc = ""]
    pub table_name: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
            table_name: self.table_name,
            reference_schema: core::default::Default::default(),
            s3_reference_data_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `reference_schema` after provisioning.\n"]
    pub fn reference_schema(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.reference_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_reference_data_source` after provisioning.\n"]
    pub fn s3_reference_data_source(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.s3_reference_data_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElDynamic {
    input: Option<
        DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl>,
    >,
    output: Option<
        DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl>,
    >,
    reference_data_source: Option<
        DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_data_source: Option<
        Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl>,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output`.\n"]
    pub fn set_output(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reference_data_source`.\n"]
    pub fn set_reference_data_source(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reference_data_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reference_data_source = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
    type O = BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
            input: core::default::Default::default(),
            output: core::default::Default::default(),
            reference_data_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(
        &self,
    ) -> ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `reference_data_source` after provisioning.\n"]
    pub fn reference_data_source(
        &self,
    ) -> ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.reference_data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
    security_group_ids: SetField<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl { }

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
    type O = BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration_id` after provisioning.\n"]
    pub fn vpc_configuration_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_configuration_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElDynamic {
    application_code_configuration: Option<
        DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl>,
    >,
    application_snapshot_configuration: Option<
        DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl>,
    >,
    environment_properties: Option<
        DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl>,
    >,
    flink_application_configuration: Option<
        DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl>,
    >,
    run_configuration: Option<
        DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl>,
    >,
    sql_application_configuration: Option<
        DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl>,
    >,
    vpc_configuration: Option<
        DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_code_configuration: Option<
        Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_snapshot_configuration: Option<
        Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_properties: Option<Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flink_application_configuration: Option<
        Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_configuration: Option<Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_application_configuration: Option<
        Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_configuration: Option<Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl>>,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationEl {
    #[doc = "Set the field `application_code_configuration`.\n"]
    pub fn set_application_code_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.application_code_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.application_code_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `application_snapshot_configuration`.\n"]
    pub fn set_application_snapshot_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.application_snapshot_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.application_snapshot_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment_properties`.\n"]
    pub fn set_environment_properties(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.environment_properties = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.environment_properties = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `flink_application_configuration`.\n"]
    pub fn set_flink_application_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.flink_application_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.flink_application_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `run_configuration`.\n"]
    pub fn set_run_configuration(
        mut self,
        v: impl Into<BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.run_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.run_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sql_application_configuration`.\n"]
    pub fn set_sql_application_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sql_application_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_application_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_configuration`.\n"]
    pub fn set_vpc_configuration(
        mut self,
        v: impl Into<BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationEl {
    type O = BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationEl {
            application_code_configuration: core::default::Default::default(),
            application_snapshot_configuration: core::default::Default::default(),
            environment_properties: core::default::Default::default(),
            flink_application_configuration: core::default::Default::default(),
            run_configuration: core::default::Default::default(),
            sql_application_configuration: core::default::Default::default(),
            vpc_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElRef {
    fn new(shared: StackShared, base: String) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_code_configuration` after provisioning.\n"]
    pub fn application_code_configuration(
        &self,
    ) -> ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.application_code_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `application_snapshot_configuration` after provisioning.\n"]
    pub fn application_snapshot_configuration(
        &self,
    ) -> ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.application_snapshot_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_properties` after provisioning.\n"]
    pub fn environment_properties(
        &self,
    ) -> ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.environment_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `flink_application_configuration` after provisioning.\n"]
    pub fn flink_application_configuration(
        &self,
    ) -> ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.flink_application_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `run_configuration` after provisioning.\n"]
    pub fn run_configuration(
        &self,
    ) -> ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.run_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_application_configuration` after provisioning.\n"]
    pub fn sql_application_configuration(
        &self,
    ) -> ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sql_application_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(
        &self,
    ) -> ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl {
    log_stream_arn: PrimField<String>,
}

impl Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl { }

impl ToListMappable for Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl {
    type O = BlockAssignable<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl {
    #[doc = ""]
    pub log_stream_arn: PrimField<String>,
}

impl BuildKinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl {
        Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl { log_stream_arn: self.log_stream_arn }
    }
}

pub struct Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef {
    fn new(shared: StackShared, base: String) -> Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef {
        Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_option_id` after provisioning.\n"]
    pub fn cloudwatch_logging_option_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_logging_option_id", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream_arn` after provisioning.\n"]
    pub fn log_stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl Kinesisanalyticsv2ApplicationTimeoutsEl {
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

impl ToListMappable for Kinesisanalyticsv2ApplicationTimeoutsEl {
    type O = BlockAssignable<Kinesisanalyticsv2ApplicationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationTimeoutsEl {}

impl BuildKinesisanalyticsv2ApplicationTimeoutsEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationTimeoutsEl {
        Kinesisanalyticsv2ApplicationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Kinesisanalyticsv2ApplicationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Kinesisanalyticsv2ApplicationTimeoutsElRef {
        Kinesisanalyticsv2ApplicationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationTimeoutsElRef {
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
struct Kinesisanalyticsv2ApplicationDynamic {
    application_configuration: Option<DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationEl>>,
    cloudwatch_logging_options: Option<DynamicBlock<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl>>,
}
