use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodepipelineCustomActionTypeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    category: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    provider_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_property: Option<Vec<CodepipelineCustomActionTypeConfigurationPropertyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_artifact_details: Option<Vec<CodepipelineCustomActionTypeInputArtifactDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_artifact_details: Option<Vec<CodepipelineCustomActionTypeOutputArtifactDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<Vec<CodepipelineCustomActionTypeSettingsEl>>,
    dynamic: CodepipelineCustomActionTypeDynamic,
}

struct CodepipelineCustomActionType_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodepipelineCustomActionTypeData>,
}

#[derive(Clone)]
pub struct CodepipelineCustomActionType(Rc<CodepipelineCustomActionType_>);

impl CodepipelineCustomActionType {
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

    #[doc = "Set the field `configuration_property`.\n"]
    pub fn set_configuration_property(
        self,
        v: impl Into<BlockAssignable<CodepipelineCustomActionTypeConfigurationPropertyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration_property = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration_property = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_artifact_details`.\n"]
    pub fn set_input_artifact_details(
        self,
        v: impl Into<BlockAssignable<CodepipelineCustomActionTypeInputArtifactDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_artifact_details = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_artifact_details = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_artifact_details`.\n"]
    pub fn set_output_artifact_details(
        self,
        v: impl Into<BlockAssignable<CodepipelineCustomActionTypeOutputArtifactDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_artifact_details = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_artifact_details = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(self, v: impl Into<BlockAssignable<CodepipelineCustomActionTypeSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_property` after provisioning.\n"]
    pub fn configuration_property(&self) -> ListRef<CodepipelineCustomActionTypeConfigurationPropertyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_artifact_details` after provisioning.\n"]
    pub fn input_artifact_details(&self) -> ListRef<CodepipelineCustomActionTypeInputArtifactDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input_artifact_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_artifact_details` after provisioning.\n"]
    pub fn output_artifact_details(&self) -> ListRef<CodepipelineCustomActionTypeOutputArtifactDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output_artifact_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<CodepipelineCustomActionTypeSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl Referable for CodepipelineCustomActionType {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CodepipelineCustomActionType { }

impl ToListMappable for CodepipelineCustomActionType {
    type O = ListRef<CodepipelineCustomActionTypeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CodepipelineCustomActionType_ {
    fn extract_resource_type(&self) -> String {
        "aws_codepipeline_custom_action_type".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodepipelineCustomActionType {
    pub tf_id: String,
    #[doc = ""]
    pub category: PrimField<String>,
    #[doc = ""]
    pub provider_name: PrimField<String>,
    #[doc = ""]
    pub version: PrimField<String>,
}

impl BuildCodepipelineCustomActionType {
    pub fn build(self, stack: &mut Stack) -> CodepipelineCustomActionType {
        let out = CodepipelineCustomActionType(Rc::new(CodepipelineCustomActionType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodepipelineCustomActionTypeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                category: self.category,
                id: core::default::Default::default(),
                provider_name: self.provider_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version: self.version,
                configuration_property: core::default::Default::default(),
                input_artifact_details: core::default::Default::default(),
                output_artifact_details: core::default::Default::default(),
                settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodepipelineCustomActionTypeRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineCustomActionTypeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CodepipelineCustomActionTypeRef {
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

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_property` after provisioning.\n"]
    pub fn configuration_property(&self) -> ListRef<CodepipelineCustomActionTypeConfigurationPropertyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_artifact_details` after provisioning.\n"]
    pub fn input_artifact_details(&self) -> ListRef<CodepipelineCustomActionTypeInputArtifactDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.input_artifact_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_artifact_details` after provisioning.\n"]
    pub fn output_artifact_details(&self) -> ListRef<CodepipelineCustomActionTypeOutputArtifactDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output_artifact_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<CodepipelineCustomActionTypeSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodepipelineCustomActionTypeConfigurationPropertyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    key: PrimField<bool>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queryable: Option<PrimField<bool>>,
    required: PrimField<bool>,
    secret: PrimField<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl CodepipelineCustomActionTypeConfigurationPropertyEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `queryable`.\n"]
    pub fn set_queryable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.queryable = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineCustomActionTypeConfigurationPropertyEl {
    type O = BlockAssignable<CodepipelineCustomActionTypeConfigurationPropertyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineCustomActionTypeConfigurationPropertyEl {
    #[doc = ""]
    pub key: PrimField<bool>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub required: PrimField<bool>,
    #[doc = ""]
    pub secret: PrimField<bool>,
}

impl BuildCodepipelineCustomActionTypeConfigurationPropertyEl {
    pub fn build(self) -> CodepipelineCustomActionTypeConfigurationPropertyEl {
        CodepipelineCustomActionTypeConfigurationPropertyEl {
            description: core::default::Default::default(),
            key: self.key,
            name: self.name,
            queryable: core::default::Default::default(),
            required: self.required,
            secret: self.secret,
            type_: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineCustomActionTypeConfigurationPropertyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineCustomActionTypeConfigurationPropertyElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineCustomActionTypeConfigurationPropertyElRef {
        CodepipelineCustomActionTypeConfigurationPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineCustomActionTypeConfigurationPropertyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `queryable` after provisioning.\n"]
    pub fn queryable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.queryable", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineCustomActionTypeInputArtifactDetailsEl {
    maximum_count: PrimField<f64>,
    minimum_count: PrimField<f64>,
}

impl CodepipelineCustomActionTypeInputArtifactDetailsEl { }

impl ToListMappable for CodepipelineCustomActionTypeInputArtifactDetailsEl {
    type O = BlockAssignable<CodepipelineCustomActionTypeInputArtifactDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineCustomActionTypeInputArtifactDetailsEl {
    #[doc = ""]
    pub maximum_count: PrimField<f64>,
    #[doc = ""]
    pub minimum_count: PrimField<f64>,
}

impl BuildCodepipelineCustomActionTypeInputArtifactDetailsEl {
    pub fn build(self) -> CodepipelineCustomActionTypeInputArtifactDetailsEl {
        CodepipelineCustomActionTypeInputArtifactDetailsEl {
            maximum_count: self.maximum_count,
            minimum_count: self.minimum_count,
        }
    }
}

pub struct CodepipelineCustomActionTypeInputArtifactDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineCustomActionTypeInputArtifactDetailsElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineCustomActionTypeInputArtifactDetailsElRef {
        CodepipelineCustomActionTypeInputArtifactDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineCustomActionTypeInputArtifactDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_count` after provisioning.\n"]
    pub fn maximum_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_count", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_count` after provisioning.\n"]
    pub fn minimum_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_count", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineCustomActionTypeOutputArtifactDetailsEl {
    maximum_count: PrimField<f64>,
    minimum_count: PrimField<f64>,
}

impl CodepipelineCustomActionTypeOutputArtifactDetailsEl { }

impl ToListMappable for CodepipelineCustomActionTypeOutputArtifactDetailsEl {
    type O = BlockAssignable<CodepipelineCustomActionTypeOutputArtifactDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineCustomActionTypeOutputArtifactDetailsEl {
    #[doc = ""]
    pub maximum_count: PrimField<f64>,
    #[doc = ""]
    pub minimum_count: PrimField<f64>,
}

impl BuildCodepipelineCustomActionTypeOutputArtifactDetailsEl {
    pub fn build(self) -> CodepipelineCustomActionTypeOutputArtifactDetailsEl {
        CodepipelineCustomActionTypeOutputArtifactDetailsEl {
            maximum_count: self.maximum_count,
            minimum_count: self.minimum_count,
        }
    }
}

pub struct CodepipelineCustomActionTypeOutputArtifactDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineCustomActionTypeOutputArtifactDetailsElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineCustomActionTypeOutputArtifactDetailsElRef {
        CodepipelineCustomActionTypeOutputArtifactDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineCustomActionTypeOutputArtifactDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_count` after provisioning.\n"]
    pub fn maximum_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_count", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_count` after provisioning.\n"]
    pub fn minimum_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_count", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineCustomActionTypeSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_url_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_url_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision_url_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    third_party_configuration_url: Option<PrimField<String>>,
}

impl CodepipelineCustomActionTypeSettingsEl {
    #[doc = "Set the field `entity_url_template`.\n"]
    pub fn set_entity_url_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entity_url_template = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_url_template`.\n"]
    pub fn set_execution_url_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_url_template = Some(v.into());
        self
    }

    #[doc = "Set the field `revision_url_template`.\n"]
    pub fn set_revision_url_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision_url_template = Some(v.into());
        self
    }

    #[doc = "Set the field `third_party_configuration_url`.\n"]
    pub fn set_third_party_configuration_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.third_party_configuration_url = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineCustomActionTypeSettingsEl {
    type O = BlockAssignable<CodepipelineCustomActionTypeSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineCustomActionTypeSettingsEl {}

impl BuildCodepipelineCustomActionTypeSettingsEl {
    pub fn build(self) -> CodepipelineCustomActionTypeSettingsEl {
        CodepipelineCustomActionTypeSettingsEl {
            entity_url_template: core::default::Default::default(),
            execution_url_template: core::default::Default::default(),
            revision_url_template: core::default::Default::default(),
            third_party_configuration_url: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineCustomActionTypeSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineCustomActionTypeSettingsElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineCustomActionTypeSettingsElRef {
        CodepipelineCustomActionTypeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineCustomActionTypeSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_url_template` after provisioning.\n"]
    pub fn entity_url_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_url_template", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_url_template` after provisioning.\n"]
    pub fn execution_url_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_url_template", self.base))
    }

    #[doc = "Get a reference to the value of field `revision_url_template` after provisioning.\n"]
    pub fn revision_url_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision_url_template", self.base))
    }

    #[doc = "Get a reference to the value of field `third_party_configuration_url` after provisioning.\n"]
    pub fn third_party_configuration_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.third_party_configuration_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineCustomActionTypeDynamic {
    configuration_property: Option<DynamicBlock<CodepipelineCustomActionTypeConfigurationPropertyEl>>,
    input_artifact_details: Option<DynamicBlock<CodepipelineCustomActionTypeInputArtifactDetailsEl>>,
    output_artifact_details: Option<DynamicBlock<CodepipelineCustomActionTypeOutputArtifactDetailsEl>>,
    settings: Option<DynamicBlock<CodepipelineCustomActionTypeSettingsEl>>,
}
