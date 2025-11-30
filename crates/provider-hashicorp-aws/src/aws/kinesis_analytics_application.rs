use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct KinesisAnalyticsApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_application: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logging_options: Option<Vec<KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inputs: Option<Vec<KinesisAnalyticsApplicationInputsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outputs: Option<Vec<KinesisAnalyticsApplicationOutputsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_data_sources: Option<Vec<KinesisAnalyticsApplicationReferenceDataSourcesEl>>,
    dynamic: KinesisAnalyticsApplicationDynamic,
}

struct KinesisAnalyticsApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KinesisAnalyticsApplicationData>,
}

#[derive(Clone)]
pub struct KinesisAnalyticsApplication(Rc<KinesisAnalyticsApplication_>);

impl KinesisAnalyticsApplication {
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

    #[doc = "Set the field `code`.\n"]
    pub fn set_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().code = Some(v.into());
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_logging_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_logging_options = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `inputs`.\n"]
    pub fn set_inputs(
        self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationInputsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inputs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inputs = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `outputs`.\n"]
    pub fn set_outputs(
        self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationOutputsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().outputs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.outputs = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `reference_data_sources`.\n"]
    pub fn set_reference_data_sources(
        self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reference_data_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reference_data_sources = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_timestamp` after provisioning.\n"]
    pub fn create_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_update_timestamp` after provisioning.\n"]
    pub fn last_update_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_update_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `start_application` after provisioning.\n"]
    pub fn start_application(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_application", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logging_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inputs` after provisioning.\n"]
    pub fn inputs(&self) -> ListRef<KinesisAnalyticsApplicationInputsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inputs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `reference_data_sources` after provisioning.\n"]
    pub fn reference_data_sources(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reference_data_sources", self.extract_ref()),
        )
    }
}

impl Referable for KinesisAnalyticsApplication {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for KinesisAnalyticsApplication {}

impl ToListMappable for KinesisAnalyticsApplication {
    type O = ListRef<KinesisAnalyticsApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for KinesisAnalyticsApplication_ {
    fn extract_resource_type(&self) -> String {
        "aws_kinesis_analytics_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKinesisAnalyticsApplication {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildKinesisAnalyticsApplication {
    pub fn build(self, stack: &mut Stack) -> KinesisAnalyticsApplication {
        let out = KinesisAnalyticsApplication(Rc::new(KinesisAnalyticsApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KinesisAnalyticsApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                code: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                start_application: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                cloudwatch_logging_options: core::default::Default::default(),
                inputs: core::default::Default::default(),
                outputs: core::default::Default::default(),
                reference_data_sources: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KinesisAnalyticsApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl KinesisAnalyticsApplicationRef {
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

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_timestamp` after provisioning.\n"]
    pub fn create_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_update_timestamp` after provisioning.\n"]
    pub fn last_update_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_update_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `start_application` after provisioning.\n"]
    pub fn start_application(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_application", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logging_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `inputs` after provisioning.\n"]
    pub fn inputs(&self) -> ListRef<KinesisAnalyticsApplicationInputsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inputs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `reference_data_sources` after provisioning.\n"]
    pub fn reference_data_sources(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reference_data_sources", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
    log_stream_arn: PrimField<String>,
    role_arn: PrimField<String>,
}

impl KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {}

impl ToListMappable for KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
    #[doc = ""]
    pub log_stream_arn: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
    pub fn build(self) -> KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
        KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl {
            log_stream_arn: self.log_stream_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef {
        KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationCloudwatchLoggingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream_arn` after provisioning.\n"]
    pub fn log_stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_stream_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
    resource_arn: PrimField<String>,
    role_arn: PrimField<String>,
}

impl KinesisAnalyticsApplicationInputsElKinesisFirehoseEl {}

impl ToListMappable for KinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationInputsElKinesisFirehoseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
        KinesisAnalyticsApplicationInputsElKinesisFirehoseEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef {
        KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElKinesisStreamEl {
    resource_arn: PrimField<String>,
    role_arn: PrimField<String>,
}

impl KinesisAnalyticsApplicationInputsElKinesisStreamEl {}

impl ToListMappable for KinesisAnalyticsApplicationInputsElKinesisStreamEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationInputsElKinesisStreamEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElKinesisStreamEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationInputsElKinesisStreamEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElKinesisStreamEl {
        KinesisAnalyticsApplicationInputsElKinesisStreamEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElKinesisStreamElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElKinesisStreamElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationInputsElKinesisStreamElRef {
        KinesisAnalyticsApplicationInputsElKinesisStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElKinesisStreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElParallelismEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
}

impl KinesisAnalyticsApplicationInputsElParallelismEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationInputsElParallelismEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationInputsElParallelismEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElParallelismEl {}

impl BuildKinesisAnalyticsApplicationInputsElParallelismEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElParallelismEl {
        KinesisAnalyticsApplicationInputsElParallelismEl {
            count: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElParallelismElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElParallelismElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationInputsElParallelismElRef {
        KinesisAnalyticsApplicationInputsElParallelismElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElParallelismElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
    resource_arn: PrimField<String>,
    role_arn: PrimField<String>,
}

impl KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {}

impl ToListMappable for KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
        KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef {
        KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationInputsElProcessingConfigurationElDynamic {
    lambda:
        Option<DynamicBlock<KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda: Option<Vec<KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl>>,
    dynamic: KinesisAnalyticsApplicationInputsElProcessingConfigurationElDynamic,
}

impl KinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(
        mut self,
        v: impl Into<
            BlockAssignable<KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationInputsElProcessingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElProcessingConfigurationEl {}

impl BuildKinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
        KinesisAnalyticsApplicationInputsElProcessingConfigurationEl {
            lambda: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef {
        KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationInputsElProcessingConfigurationElLambdaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping: Option<PrimField<String>>,
    name: PrimField<String>,
    sql_type: PrimField<String>,
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
    #[doc = "Set the field `mapping`.\n"]
    pub fn set_mapping(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mapping = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub sql_type: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
        KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl {
            mapping: core::default::Default::default(),
            name: self.name,
            sql_type: self.sql_type,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef {
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
pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {
    record_column_delimiter: PrimField<String>,
    record_row_delimiter: PrimField<String>,
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {}

impl ToListMappable
    for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl
{
    type O = BlockAssignable<
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {
    #[doc = ""]
    pub record_column_delimiter: PrimField<String>,
    #[doc = ""]
    pub record_row_delimiter: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {
    pub fn build(
        self,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl {
            record_column_delimiter: self.record_column_delimiter,
            record_row_delimiter: self.record_row_delimiter,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_column_delimiter` after provisioning.\n"]
    pub fn record_column_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_column_delimiter", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `record_row_delimiter` after provisioning.\n"]
    pub fn record_row_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_row_delimiter", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {
    record_row_path: PrimField<String>,
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {}

impl ToListMappable
    for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl
{
    type O = BlockAssignable<
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {
    #[doc = ""]
    pub record_row_path: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {
    pub fn build(
        self,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl {
            record_row_path: self.record_row_path,
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_row_path` after provisioning.\n"]
    pub fn record_row_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_row_path", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElDynamic {
    csv: Option<
        DynamicBlock<
            KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl,
        >,
    >,
    json: Option<
        DynamicBlock<
            KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    csv: Option<
        Vec<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<
        Vec<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl>,
    >,
    dynamic: KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElDynamic,
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {
    #[doc = "Set the field `csv`.\n"]
    pub fn set_csv(
        mut self,
        v: impl Into<
            BlockAssignable<
                KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.csv = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.csv = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v: impl Into<
            BlockAssignable<
                KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl
{
    type O = BlockAssignable<
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {}

impl BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {
    pub fn build(
        self,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl {
            csv: core::default::Default::default(),
            json: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv` after provisioning.\n"]
    pub fn csv(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElCsvElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.csv", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(
        &self,
    ) -> ListRef<
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElJsonElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElDynamic {
    mapping_parameters: Option<
        DynamicBlock<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping_parameters:
        Option<Vec<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl>>,
    dynamic: KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElDynamic,
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
    #[doc = "Set the field `mapping_parameters`.\n"]
    pub fn set_mapping_parameters(
        mut self,
        v: impl Into<
            BlockAssignable<
                KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mapping_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mapping_parameters = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {}

impl BuildKinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl {
            mapping_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_format_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `mapping_parameters` after provisioning.\n"]
    pub fn mapping_parameters(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElMappingParametersElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mapping_parameters", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationInputsElSchemaElDynamic {
    record_columns:
        Option<DynamicBlock<KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl>>,
    record_format: Option<DynamicBlock<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    record_encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_columns: Option<Vec<KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_format: Option<Vec<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl>>,
    dynamic: KinesisAnalyticsApplicationInputsElSchemaElDynamic,
}

impl KinesisAnalyticsApplicationInputsElSchemaEl {
    #[doc = "Set the field `record_encoding`.\n"]
    pub fn set_record_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.record_encoding = Some(v.into());
        self
    }

    #[doc = "Set the field `record_columns`.\n"]
    pub fn set_record_columns(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record_columns = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `record_format`.\n"]
    pub fn set_record_format(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record_format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record_format = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationInputsElSchemaEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElSchemaEl {}

impl BuildKinesisAnalyticsApplicationInputsElSchemaEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElSchemaEl {
        KinesisAnalyticsApplicationInputsElSchemaEl {
            record_encoding: core::default::Default::default(),
            record_columns: core::default::Default::default(),
            record_format: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElSchemaElRef {
    fn new(shared: StackShared, base: String) -> KinesisAnalyticsApplicationInputsElSchemaElRef {
        KinesisAnalyticsApplicationInputsElSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_encoding` after provisioning.\n"]
    pub fn record_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_encoding", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `record_columns` after provisioning.\n"]
    pub fn record_columns(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationInputsElSchemaElRecordColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.record_columns", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `record_format` after provisioning.\n"]
    pub fn record_format(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationInputsElSchemaElRecordFormatElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.record_format", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_position: Option<PrimField<String>>,
}

impl KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
    #[doc = "Set the field `starting_position`.\n"]
    pub fn set_starting_position(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.starting_position = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {}

impl BuildKinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
        KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl {
            starting_position: core::default::Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef {
        KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.starting_position", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationInputsElDynamic {
    kinesis_firehose: Option<DynamicBlock<KinesisAnalyticsApplicationInputsElKinesisFirehoseEl>>,
    kinesis_stream: Option<DynamicBlock<KinesisAnalyticsApplicationInputsElKinesisStreamEl>>,
    parallelism: Option<DynamicBlock<KinesisAnalyticsApplicationInputsElParallelismEl>>,
    processing_configuration:
        Option<DynamicBlock<KinesisAnalyticsApplicationInputsElProcessingConfigurationEl>>,
    schema: Option<DynamicBlock<KinesisAnalyticsApplicationInputsElSchemaEl>>,
    starting_position_configuration:
        Option<DynamicBlock<KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationInputsEl {
    name_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_firehose: Option<Vec<KinesisAnalyticsApplicationInputsElKinesisFirehoseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_stream: Option<Vec<KinesisAnalyticsApplicationInputsElKinesisStreamEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelism: Option<Vec<KinesisAnalyticsApplicationInputsElParallelismEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processing_configuration:
        Option<Vec<KinesisAnalyticsApplicationInputsElProcessingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Vec<KinesisAnalyticsApplicationInputsElSchemaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_position_configuration:
        Option<Vec<KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl>>,
    dynamic: KinesisAnalyticsApplicationInputsElDynamic,
}

impl KinesisAnalyticsApplicationInputsEl {
    #[doc = "Set the field `kinesis_firehose`.\n"]
    pub fn set_kinesis_firehose(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationInputsElKinesisFirehoseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_firehose = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `kinesis_stream`.\n"]
    pub fn set_kinesis_stream(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationInputsElKinesisStreamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_stream = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `parallelism`.\n"]
    pub fn set_parallelism(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationInputsElParallelismEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parallelism = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parallelism = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `processing_configuration`.\n"]
    pub fn set_processing_configuration(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationInputsElProcessingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.processing_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.processing_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationInputsElSchemaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `starting_position_configuration`.\n"]
    pub fn set_starting_position_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<KinesisAnalyticsApplicationInputsElStartingPositionConfigurationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.starting_position_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.starting_position_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationInputsEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationInputsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationInputsEl {
    #[doc = ""]
    pub name_prefix: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationInputsEl {
    pub fn build(self) -> KinesisAnalyticsApplicationInputsEl {
        KinesisAnalyticsApplicationInputsEl {
            name_prefix: self.name_prefix,
            kinesis_firehose: core::default::Default::default(),
            kinesis_stream: core::default::Default::default(),
            parallelism: core::default::Default::default(),
            processing_configuration: core::default::Default::default(),
            schema: core::default::Default::default(),
            starting_position_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationInputsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationInputsElRef {
    fn new(shared: StackShared, base: String) -> KinesisAnalyticsApplicationInputsElRef {
        KinesisAnalyticsApplicationInputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationInputsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_names` after provisioning.\n"]
    pub fn stream_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.stream_names", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose` after provisioning.\n"]
    pub fn kinesis_firehose(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationInputsElKinesisFirehoseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_firehose", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kinesis_stream` after provisioning.\n"]
    pub fn kinesis_stream(&self) -> ListRef<KinesisAnalyticsApplicationInputsElKinesisStreamElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_stream", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `parallelism` after provisioning.\n"]
    pub fn parallelism(&self) -> ListRef<KinesisAnalyticsApplicationInputsElParallelismElRef> {
        ListRef::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }

    #[doc = "Get a reference to the value of field `processing_configuration` after provisioning.\n"]
    pub fn processing_configuration(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationInputsElProcessingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.processing_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<KinesisAnalyticsApplicationInputsElSchemaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_position_configuration` after provisioning.\n"]
    pub fn starting_position_configuration(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationInputsElStartingPositionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.starting_position_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
    resource_arn: PrimField<String>,
    role_arn: PrimField<String>,
}

impl KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {}

impl ToListMappable for KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
    pub fn build(self) -> KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
        KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef {
        KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationOutputsElKinesisStreamEl {
    resource_arn: PrimField<String>,
    role_arn: PrimField<String>,
}

impl KinesisAnalyticsApplicationOutputsElKinesisStreamEl {}

impl ToListMappable for KinesisAnalyticsApplicationOutputsElKinesisStreamEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationOutputsElKinesisStreamEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationOutputsElKinesisStreamEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationOutputsElKinesisStreamEl {
    pub fn build(self) -> KinesisAnalyticsApplicationOutputsElKinesisStreamEl {
        KinesisAnalyticsApplicationOutputsElKinesisStreamEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationOutputsElKinesisStreamElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationOutputsElKinesisStreamElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationOutputsElKinesisStreamElRef {
        KinesisAnalyticsApplicationOutputsElKinesisStreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationOutputsElKinesisStreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationOutputsElLambdaEl {
    resource_arn: PrimField<String>,
    role_arn: PrimField<String>,
}

impl KinesisAnalyticsApplicationOutputsElLambdaEl {}

impl ToListMappable for KinesisAnalyticsApplicationOutputsElLambdaEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationOutputsElLambdaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationOutputsElLambdaEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationOutputsElLambdaEl {
    pub fn build(self) -> KinesisAnalyticsApplicationOutputsElLambdaEl {
        KinesisAnalyticsApplicationOutputsElLambdaEl {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationOutputsElLambdaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationOutputsElLambdaElRef {
    fn new(shared: StackShared, base: String) -> KinesisAnalyticsApplicationOutputsElLambdaElRef {
        KinesisAnalyticsApplicationOutputsElLambdaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationOutputsElLambdaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationOutputsElSchemaEl {
    record_format_type: PrimField<String>,
}

impl KinesisAnalyticsApplicationOutputsElSchemaEl {}

impl ToListMappable for KinesisAnalyticsApplicationOutputsElSchemaEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationOutputsElSchemaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationOutputsElSchemaEl {
    #[doc = ""]
    pub record_format_type: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationOutputsElSchemaEl {
    pub fn build(self) -> KinesisAnalyticsApplicationOutputsElSchemaEl {
        KinesisAnalyticsApplicationOutputsElSchemaEl {
            record_format_type: self.record_format_type,
        }
    }
}

pub struct KinesisAnalyticsApplicationOutputsElSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationOutputsElSchemaElRef {
    fn new(shared: StackShared, base: String) -> KinesisAnalyticsApplicationOutputsElSchemaElRef {
        KinesisAnalyticsApplicationOutputsElSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationOutputsElSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_format_type", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationOutputsElDynamic {
    kinesis_firehose: Option<DynamicBlock<KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl>>,
    kinesis_stream: Option<DynamicBlock<KinesisAnalyticsApplicationOutputsElKinesisStreamEl>>,
    lambda: Option<DynamicBlock<KinesisAnalyticsApplicationOutputsElLambdaEl>>,
    schema: Option<DynamicBlock<KinesisAnalyticsApplicationOutputsElSchemaEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationOutputsEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_firehose: Option<Vec<KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_stream: Option<Vec<KinesisAnalyticsApplicationOutputsElKinesisStreamEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda: Option<Vec<KinesisAnalyticsApplicationOutputsElLambdaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Vec<KinesisAnalyticsApplicationOutputsElSchemaEl>>,
    dynamic: KinesisAnalyticsApplicationOutputsElDynamic,
}

impl KinesisAnalyticsApplicationOutputsEl {
    #[doc = "Set the field `kinesis_firehose`.\n"]
    pub fn set_kinesis_firehose(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationOutputsElKinesisFirehoseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_firehose = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `kinesis_stream`.\n"]
    pub fn set_kinesis_stream(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationOutputsElKinesisStreamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_stream = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationOutputsElLambdaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationOutputsElSchemaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationOutputsEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationOutputsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationOutputsEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationOutputsEl {
    pub fn build(self) -> KinesisAnalyticsApplicationOutputsEl {
        KinesisAnalyticsApplicationOutputsEl {
            name: self.name,
            kinesis_firehose: core::default::Default::default(),
            kinesis_stream: core::default::Default::default(),
            lambda: core::default::Default::default(),
            schema: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationOutputsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationOutputsElRef {
    fn new(shared: StackShared, base: String) -> KinesisAnalyticsApplicationOutputsElRef {
        KinesisAnalyticsApplicationOutputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationOutputsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose` after provisioning.\n"]
    pub fn kinesis_firehose(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationOutputsElKinesisFirehoseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_firehose", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kinesis_stream` after provisioning.\n"]
    pub fn kinesis_stream(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationOutputsElKinesisStreamElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_stream", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> ListRef<KinesisAnalyticsApplicationOutputsElLambdaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<KinesisAnalyticsApplicationOutputsElSchemaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElS3El {
    bucket_arn: PrimField<String>,
    file_key: PrimField<String>,
    role_arn: PrimField<String>,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElS3El {}

impl ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElS3El {
    type O = BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElS3El {
    #[doc = ""]
    pub bucket_arn: PrimField<String>,
    #[doc = ""]
    pub file_key: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElS3El {
    pub fn build(self) -> KinesisAnalyticsApplicationReferenceDataSourcesElS3El {
        KinesisAnalyticsApplicationReferenceDataSourcesElS3El {
            bucket_arn: self.bucket_arn,
            file_key: self.file_key,
            role_arn: self.role_arn,
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef {
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping: Option<PrimField<String>>,
    name: PrimField<String>,
    sql_type: PrimField<String>,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
    #[doc = "Set the field `mapping`.\n"]
    pub fn set_mapping(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mapping = Some(v.into());
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
    type O =
        BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub sql_type: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
    pub fn build(self) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl {
            mapping: core::default::Default::default(),
            name: self.name,
            sql_type: self.sql_type,
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef {
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
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl
{
    record_column_delimiter: PrimField<String>,
    record_row_delimiter: PrimField<String>,
}

impl
    KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl
{
}

impl ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl {
    type O =
        BlockAssignable<
            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl
{
    #[doc = ""]
    pub record_column_delimiter: PrimField<String>,
    #[doc = ""]
    pub record_row_delimiter: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl {
    pub fn build(
        self,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl {
            record_column_delimiter: self.record_column_delimiter,
            record_row_delimiter: self.record_row_delimiter,
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef {
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
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl
{
    record_row_path: PrimField<String>,
}

impl
    KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl
{
}

impl ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl {
    type O =
        BlockAssignable<
            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl
{
    #[doc = ""]
    pub record_row_path: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl {
    pub fn build(
        self,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl {
            record_row_path: self.record_row_path,
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_row_path` after provisioning.\n"]
    pub fn record_row_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_row_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElDynamic {
    csv: Option<
        DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl>,
    >,
    json: Option<
        DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    csv: Option<Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl>>,
    dynamic: KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElDynamic,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl {
    #[doc = "Set the field `csv`.\n"]
    pub fn set_csv(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.csv = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.csv = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl
{
    type O = BlockAssignable<
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl
{}

impl
    BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl
{
    pub fn build(
        self,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl
    {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl {
            csv: core::default::Default::default(),
            json: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv` after provisioning.\n"]
    pub fn csv(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElCsvElRef>{
        ListRef::new(self.shared().clone(), format!("{}.csv", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElJsonElRef>{
        ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElDynamic {
    mapping_parameters: Option<
        DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping_parameters: Option<
        Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl>,
    >,
    dynamic: KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElDynamic,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
    #[doc = "Set the field `mapping_parameters`.\n"]
    pub fn set_mapping_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mapping_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mapping_parameters = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
    type O =
        BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
    pub fn build(self) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl {
            mapping_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_format_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `mapping_parameters` after provisioning.\n"]
    pub fn mapping_parameters(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElMappingParametersElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.mapping_parameters", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElDynamic {
    record_columns: Option<
        DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl>,
    >,
    record_format: Option<
        DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl>,
    >,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    record_encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_columns:
        Option<Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_format:
        Option<Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl>>,
    dynamic: KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElDynamic,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
    #[doc = "Set the field `record_encoding`.\n"]
    pub fn set_record_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.record_encoding = Some(v.into());
        self
    }

    #[doc = "Set the field `record_columns`.\n"]
    pub fn set_record_columns(
        mut self,
        v: impl Into<
            BlockAssignable<
                KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record_columns = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `record_format`.\n"]
    pub fn set_record_format(
        mut self,
        v: impl Into<
            BlockAssignable<
                KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.record_format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.record_format = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
    pub fn build(self) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl {
            record_encoding: core::default::Default::default(),
            record_columns: core::default::Default::default(),
            record_format: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_encoding` after provisioning.\n"]
    pub fn record_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_encoding", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `record_columns` after provisioning.\n"]
    pub fn record_columns(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.record_columns", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `record_format` after provisioning.\n"]
    pub fn record_format(
        &self,
    ) -> ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRecordFormatElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.record_format", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationReferenceDataSourcesElDynamic {
    s3: Option<DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElS3El>>,
    schema: Option<DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl>>,
}

#[derive(Serialize)]
pub struct KinesisAnalyticsApplicationReferenceDataSourcesEl {
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<KinesisAnalyticsApplicationReferenceDataSourcesElS3El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Vec<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl>>,
    dynamic: KinesisAnalyticsApplicationReferenceDataSourcesElDynamic,
}

impl KinesisAnalyticsApplicationReferenceDataSourcesEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl Into<BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for KinesisAnalyticsApplicationReferenceDataSourcesEl {
    type O = BlockAssignable<KinesisAnalyticsApplicationReferenceDataSourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisAnalyticsApplicationReferenceDataSourcesEl {
    #[doc = ""]
    pub table_name: PrimField<String>,
}

impl BuildKinesisAnalyticsApplicationReferenceDataSourcesEl {
    pub fn build(self) -> KinesisAnalyticsApplicationReferenceDataSourcesEl {
        KinesisAnalyticsApplicationReferenceDataSourcesEl {
            table_name: self.table_name,
            s3: core::default::Default::default(),
            schema: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KinesisAnalyticsApplicationReferenceDataSourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KinesisAnalyticsApplicationReferenceDataSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KinesisAnalyticsApplicationReferenceDataSourcesElRef {
        KinesisAnalyticsApplicationReferenceDataSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KinesisAnalyticsApplicationReferenceDataSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<KinesisAnalyticsApplicationReferenceDataSourcesElSchemaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct KinesisAnalyticsApplicationDynamic {
    cloudwatch_logging_options:
        Option<DynamicBlock<KinesisAnalyticsApplicationCloudwatchLoggingOptionsEl>>,
    inputs: Option<DynamicBlock<KinesisAnalyticsApplicationInputsEl>>,
    outputs: Option<DynamicBlock<KinesisAnalyticsApplicationOutputsEl>>,
    reference_data_sources: Option<DynamicBlock<KinesisAnalyticsApplicationReferenceDataSourcesEl>>,
}
