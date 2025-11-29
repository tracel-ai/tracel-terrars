use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrcontainersJobTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_template_data: Option<Vec<EmrcontainersJobTemplateJobTemplateDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EmrcontainersJobTemplateTimeoutsEl>,
    dynamic: EmrcontainersJobTemplateDynamic,
}

struct EmrcontainersJobTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmrcontainersJobTemplateData>,
}

#[derive(Clone)]
pub struct EmrcontainersJobTemplate(Rc<EmrcontainersJobTemplate_>);

impl EmrcontainersJobTemplate {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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

    #[doc = "Set the field `job_template_data`.\n"]
    pub fn set_job_template_data(
        self,
        v: impl Into<BlockAssignable<EmrcontainersJobTemplateJobTemplateDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().job_template_data = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.job_template_data = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EmrcontainersJobTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_template_data` after provisioning.\n"]
    pub fn job_template_data(&self) -> ListRef<EmrcontainersJobTemplateJobTemplateDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.job_template_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EmrcontainersJobTemplateTimeoutsElRef {
        EmrcontainersJobTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for EmrcontainersJobTemplate {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EmrcontainersJobTemplate { }

impl ToListMappable for EmrcontainersJobTemplate {
    type O = ListRef<EmrcontainersJobTemplateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmrcontainersJobTemplate_ {
    fn extract_resource_type(&self) -> String {
        "aws_emrcontainers_job_template".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrcontainersJobTemplate {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildEmrcontainersJobTemplate {
    pub fn build(self, stack: &mut Stack) -> EmrcontainersJobTemplate {
        let out = EmrcontainersJobTemplate(Rc::new(EmrcontainersJobTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmrcontainersJobTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                job_template_data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrcontainersJobTemplateRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EmrcontainersJobTemplateRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_template_data` after provisioning.\n"]
    pub fn job_template_data(&self) -> ListRef<EmrcontainersJobTemplateJobTemplateDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.job_template_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EmrcontainersJobTemplateTimeoutsElRef {
        EmrcontainersJobTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    classification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsEl {
    #[doc = "Set the field `classification`.\n"]
    pub fn set_classification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.classification = Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
}

impl ToListMappable for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsEl {
    type O =
        BlockAssignable<
            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsEl {}

impl BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsEl {
    pub fn build(
        self,
    ) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsEl {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsEl {
            classification: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsElRef {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.classification", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElDynamic {
    configurations: Option<
        DynamicBlock<
            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationEl {
    classification: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configurations: Option<
        Vec<
            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsEl,
        >,
    >,
    dynamic: EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElDynamic,
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationEl {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }

    #[doc = "Set the field `configurations`.\n"]
    pub fn set_configurations(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.configurations = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.configurations = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationEl {
    type O =
        BlockAssignable<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationEl {
    #[doc = ""]
    pub classification: PrimField<String>,
}

impl BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationEl {
    pub fn build(
        self,
    ) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationEl {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationEl {
            classification: self.classification,
            properties: core::default::Default::default(),
            configurations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElRef {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.classification", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `configurations` after provisioning.\n"]
    pub fn configurations(
        &self,
    ) -> ListRef<
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElConfigurationsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.configurations", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationEl {
    log_group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_stream_name_prefix: Option<PrimField<String>>,
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationEl {
    #[doc = "Set the field `log_stream_name_prefix`.\n"]
    pub fn set_log_stream_name_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_stream_name_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationEl {
    type O =
        BlockAssignable<
            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationEl {
    #[doc = ""]
    pub log_group_name: PrimField<String>,
}

impl BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationEl {
    pub fn build(
        self,
    ) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationEl {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationEl {
            log_group_name: self.log_group_name,
            log_stream_name_prefix: core::default::Default::default(),
        }
    }
}

pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationElRef {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream_name_prefix` after provisioning.\n"]
    pub fn log_stream_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationEl {
    log_uri: PrimField<String>,
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationEl {

}

impl ToListMappable for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationEl {
    type O =
        BlockAssignable<
            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationEl {
    #[doc = ""]
    pub log_uri: PrimField<String>,
}

impl BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationEl {
    pub fn build(
        self,
    ) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationEl {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationEl {
            log_uri: self.log_uri,
        }
    }
}

pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationElRef {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_uri` after provisioning.\n"]
    pub fn log_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElDynamic {
    cloud_watch_monitoring_configuration: Option<
        DynamicBlock<
            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationEl,
        >,
    >,
    s3_monitoring_configuration: Option<
        DynamicBlock<
            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    persistent_app_ui: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_watch_monitoring_configuration: Option<
        Vec<
            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_monitoring_configuration: Option<
        Vec<
            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationEl,
        >,
    >,
    dynamic: EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElDynamic,
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationEl {
    #[doc = "Set the field `persistent_app_ui`.\n"]
    pub fn set_persistent_app_ui(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.persistent_app_ui = Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_watch_monitoring_configuration`.\n"]
    pub fn set_cloud_watch_monitoring_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_watch_monitoring_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_watch_monitoring_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_monitoring_configuration`.\n"]
    pub fn set_s3_monitoring_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_monitoring_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_monitoring_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationEl {
    type O =
        BlockAssignable<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationEl {}

impl BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationEl {
    pub fn build(self) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationEl {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationEl {
            persistent_app_ui: core::default::Default::default(),
            cloud_watch_monitoring_configuration: core::default::Default::default(),
            s3_monitoring_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElRef {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `persistent_app_ui` after provisioning.\n"]
    pub fn persistent_app_ui(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.persistent_app_ui", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_monitoring_configuration` after provisioning.\n"]
    pub fn cloud_watch_monitoring_configuration(
        &self,
    ) -> ListRef<
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElCloudWatchMonitoringConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.cloud_watch_monitoring_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_monitoring_configuration` after provisioning.\n"]
    pub fn s3_monitoring_configuration(
        &self,
    ) -> ListRef<
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElS3MonitoringConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.s3_monitoring_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElDynamic {
    application_configuration: Option<
        DynamicBlock<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationEl>,
    >,
    monitoring_configuration: Option<
        DynamicBlock<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_configuration: Option<
        Vec<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_configuration: Option<
        Vec<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationEl>,
    >,
    dynamic: EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElDynamic,
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesEl {
    #[doc = "Set the field `application_configuration`.\n"]
    pub fn set_application_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.application_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.application_configuration = Some(d);
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
                            EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationEl,
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
}

impl ToListMappable for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesEl {
    type O = BlockAssignable<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesEl {}

impl BuildEmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesEl {
    pub fn build(self) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesEl {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesEl {
            application_configuration: core::default::Default::default(),
            monitoring_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElRef {
        EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_configuration` after provisioning.\n"]
    pub fn application_configuration(
        &self,
    ) -> ListRef<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElApplicationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.application_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_configuration` after provisioning.\n"]
    pub fn monitoring_configuration(
        &self,
    ) -> ListRef<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElMonitoringConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.monitoring_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_point: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_sql_parameters: Option<PrimField<String>>,
}

impl EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverEl {
    #[doc = "Set the field `entry_point`.\n"]
    pub fn set_entry_point(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entry_point = Some(v.into());
        self
    }

    #[doc = "Set the field `spark_sql_parameters`.\n"]
    pub fn set_spark_sql_parameters(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spark_sql_parameters = Some(v.into());
        self
    }
}

impl ToListMappable for EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverEl {
    type O = BlockAssignable<EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverEl {}

impl BuildEmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverEl {
    pub fn build(self) -> EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverEl {
        EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverEl {
            entry_point: core::default::Default::default(),
            spark_sql_parameters: core::default::Default::default(),
        }
    }
}

pub struct EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverElRef {
        EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\n"]
    pub fn entry_point(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_sql_parameters` after provisioning.\n"]
    pub fn spark_sql_parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spark_sql_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverEl {
    entry_point: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_point_arguments: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_submit_parameters: Option<PrimField<String>>,
}

impl EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverEl {
    #[doc = "Set the field `entry_point_arguments`.\n"]
    pub fn set_entry_point_arguments(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.entry_point_arguments = Some(v.into());
        self
    }

    #[doc = "Set the field `spark_submit_parameters`.\n"]
    pub fn set_spark_submit_parameters(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spark_submit_parameters = Some(v.into());
        self
    }
}

impl ToListMappable for EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverEl {
    type O = BlockAssignable<EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverEl {
    #[doc = ""]
    pub entry_point: PrimField<String>,
}

impl BuildEmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverEl {
    pub fn build(self) -> EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverEl {
        EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverEl {
            entry_point: self.entry_point,
            entry_point_arguments: core::default::Default::default(),
            spark_submit_parameters: core::default::Default::default(),
        }
    }
}

pub struct EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverElRef {
        EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\n"]
    pub fn entry_point(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }

    #[doc = "Get a reference to the value of field `entry_point_arguments` after provisioning.\n"]
    pub fn entry_point_arguments(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.entry_point_arguments", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_submit_parameters` after provisioning.\n"]
    pub fn spark_submit_parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spark_submit_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplateJobTemplateDataElJobDriverElDynamic {
    spark_sql_job_driver: Option<
        DynamicBlock<EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverEl>,
    >,
    spark_submit_job_driver: Option<
        DynamicBlock<EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverEl>,
    >,
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplateJobTemplateDataElJobDriverEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_sql_job_driver: Option<Vec<EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_submit_job_driver: Option<Vec<EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverEl>>,
    dynamic: EmrcontainersJobTemplateJobTemplateDataElJobDriverElDynamic,
}

impl EmrcontainersJobTemplateJobTemplateDataElJobDriverEl {
    #[doc = "Set the field `spark_sql_job_driver`.\n"]
    pub fn set_spark_sql_job_driver(
        mut self,
        v: impl Into<BlockAssignable<EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spark_sql_job_driver = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_sql_job_driver = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_submit_job_driver`.\n"]
    pub fn set_spark_submit_job_driver(
        mut self,
        v: impl Into<BlockAssignable<EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spark_submit_job_driver = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_submit_job_driver = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrcontainersJobTemplateJobTemplateDataElJobDriverEl {
    type O = BlockAssignable<EmrcontainersJobTemplateJobTemplateDataElJobDriverEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplateJobTemplateDataElJobDriverEl {}

impl BuildEmrcontainersJobTemplateJobTemplateDataElJobDriverEl {
    pub fn build(self) -> EmrcontainersJobTemplateJobTemplateDataElJobDriverEl {
        EmrcontainersJobTemplateJobTemplateDataElJobDriverEl {
            spark_sql_job_driver: core::default::Default::default(),
            spark_submit_job_driver: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrcontainersJobTemplateJobTemplateDataElJobDriverElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateJobTemplateDataElJobDriverElRef {
    fn new(shared: StackShared, base: String) -> EmrcontainersJobTemplateJobTemplateDataElJobDriverElRef {
        EmrcontainersJobTemplateJobTemplateDataElJobDriverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplateJobTemplateDataElJobDriverElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `spark_sql_job_driver` after provisioning.\n"]
    pub fn spark_sql_job_driver(
        &self,
    ) -> ListRef<EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSqlJobDriverElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spark_sql_job_driver", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_submit_job_driver` after provisioning.\n"]
    pub fn spark_submit_job_driver(
        &self,
    ) -> ListRef<EmrcontainersJobTemplateJobTemplateDataElJobDriverElSparkSubmitJobDriverElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spark_submit_job_driver", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplateJobTemplateDataElDynamic {
    configuration_overrides: Option<DynamicBlock<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesEl>>,
    job_driver: Option<DynamicBlock<EmrcontainersJobTemplateJobTemplateDataElJobDriverEl>>,
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplateJobTemplateDataEl {
    execution_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_tags: Option<RecField<PrimField<String>>>,
    release_label: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_overrides: Option<Vec<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_driver: Option<Vec<EmrcontainersJobTemplateJobTemplateDataElJobDriverEl>>,
    dynamic: EmrcontainersJobTemplateJobTemplateDataElDynamic,
}

impl EmrcontainersJobTemplateJobTemplateDataEl {
    #[doc = "Set the field `job_tags`.\n"]
    pub fn set_job_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.job_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `configuration_overrides`.\n"]
    pub fn set_configuration_overrides(
        mut self,
        v: impl Into<BlockAssignable<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.configuration_overrides = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.configuration_overrides = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job_driver`.\n"]
    pub fn set_job_driver(
        mut self,
        v: impl Into<BlockAssignable<EmrcontainersJobTemplateJobTemplateDataElJobDriverEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.job_driver = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.job_driver = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrcontainersJobTemplateJobTemplateDataEl {
    type O = BlockAssignable<EmrcontainersJobTemplateJobTemplateDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplateJobTemplateDataEl {
    #[doc = ""]
    pub execution_role_arn: PrimField<String>,
    #[doc = ""]
    pub release_label: PrimField<String>,
}

impl BuildEmrcontainersJobTemplateJobTemplateDataEl {
    pub fn build(self) -> EmrcontainersJobTemplateJobTemplateDataEl {
        EmrcontainersJobTemplateJobTemplateDataEl {
            execution_role_arn: self.execution_role_arn,
            job_tags: core::default::Default::default(),
            release_label: self.release_label,
            configuration_overrides: core::default::Default::default(),
            job_driver: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrcontainersJobTemplateJobTemplateDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateJobTemplateDataElRef {
    fn new(shared: StackShared, base: String) -> EmrcontainersJobTemplateJobTemplateDataElRef {
        EmrcontainersJobTemplateJobTemplateDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplateJobTemplateDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `job_tags` after provisioning.\n"]
    pub fn job_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.job_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration_overrides` after provisioning.\n"]
    pub fn configuration_overrides(
        &self,
    ) -> ListRef<EmrcontainersJobTemplateJobTemplateDataElConfigurationOverridesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `job_driver` after provisioning.\n"]
    pub fn job_driver(&self) -> ListRef<EmrcontainersJobTemplateJobTemplateDataElJobDriverElRef> {
        ListRef::new(self.shared().clone(), format!("{}.job_driver", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl EmrcontainersJobTemplateTimeoutsEl {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for EmrcontainersJobTemplateTimeoutsEl {
    type O = BlockAssignable<EmrcontainersJobTemplateTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplateTimeoutsEl {}

impl BuildEmrcontainersJobTemplateTimeoutsEl {
    pub fn build(self) -> EmrcontainersJobTemplateTimeoutsEl {
        EmrcontainersJobTemplateTimeoutsEl { delete: core::default::Default::default() }
    }
}

pub struct EmrcontainersJobTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrcontainersJobTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EmrcontainersJobTemplateTimeoutsElRef {
        EmrcontainersJobTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplateTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplateDynamic {
    job_template_data: Option<DynamicBlock<EmrcontainersJobTemplateJobTemplateDataEl>>,
}
