use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComprehendEntityRecognizerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    data_access_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    language_code: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_kms_key_id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_data_config: Option<Vec<ComprehendEntityRecognizerInputDataConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComprehendEntityRecognizerTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<ComprehendEntityRecognizerVpcConfigEl>>,
    dynamic: ComprehendEntityRecognizerDynamic,
}
struct ComprehendEntityRecognizer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComprehendEntityRecognizerData>,
}
#[derive(Clone)]
pub struct ComprehendEntityRecognizer(Rc<ComprehendEntityRecognizer_>);
impl ComprehendEntityRecognizer {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `model_kms_key_id`.\n"]
    pub fn set_model_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().model_kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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
    #[doc = "Set the field `version_name`.\n"]
    pub fn set_version_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_name = Some(v.into());
        self
    }
    #[doc = "Set the field `version_name_prefix`.\n"]
    pub fn set_version_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_name_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_kms_key_id`.\n"]
    pub fn set_volume_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().volume_kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `input_data_config`.\n"]
    pub fn set_input_data_config(
        self,
        v: impl Into<BlockAssignable<ComprehendEntityRecognizerInputDataConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_data_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_data_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComprehendEntityRecognizerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl Into<BlockAssignable<ComprehendEntityRecognizerVpcConfigEl>>,
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
    #[doc = "Get a reference to the value of field `data_access_role_arn` after provisioning.\n"]
    pub fn data_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_access_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_kms_key_id` after provisioning.\n"]
    pub fn model_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_kms_key_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_name_prefix` after provisioning.\n"]
    pub fn version_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_kms_key_id` after provisioning.\n"]
    pub fn volume_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(&self) -> ListRef<ComprehendEntityRecognizerInputDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComprehendEntityRecognizerTimeoutsElRef {
        ComprehendEntityRecognizerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<ComprehendEntityRecognizerVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
impl Referable for ComprehendEntityRecognizer {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComprehendEntityRecognizer {}
impl ToListMappable for ComprehendEntityRecognizer {
    type O = ListRef<ComprehendEntityRecognizerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComprehendEntityRecognizer_ {
    fn extract_resource_type(&self) -> String {
        "aws_comprehend_entity_recognizer".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComprehendEntityRecognizer {
    pub tf_id: String,
    #[doc = ""]
    pub data_access_role_arn: PrimField<String>,
    #[doc = ""]
    pub language_code: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildComprehendEntityRecognizer {
    pub fn build(self, stack: &mut Stack) -> ComprehendEntityRecognizer {
        let out = ComprehendEntityRecognizer(Rc::new(ComprehendEntityRecognizer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComprehendEntityRecognizerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_access_role_arn: self.data_access_role_arn,
                id: core::default::Default::default(),
                language_code: self.language_code,
                model_kms_key_id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version_name: core::default::Default::default(),
                version_name_prefix: core::default::Default::default(),
                volume_kms_key_id: core::default::Default::default(),
                input_data_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComprehendEntityRecognizerRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendEntityRecognizerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComprehendEntityRecognizerRef {
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
    #[doc = "Get a reference to the value of field `data_access_role_arn` after provisioning.\n"]
    pub fn data_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_access_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_kms_key_id` after provisioning.\n"]
    pub fn model_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_kms_key_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_name_prefix` after provisioning.\n"]
    pub fn version_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_kms_key_id` after provisioning.\n"]
    pub fn volume_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(&self) -> ListRef<ComprehendEntityRecognizerInputDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComprehendEntityRecognizerTimeoutsElRef {
        ComprehendEntityRecognizerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<ComprehendEntityRecognizerVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComprehendEntityRecognizerInputDataConfigElAnnotationsEl {
    s3_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_s3_uri: Option<PrimField<String>>,
}
impl ComprehendEntityRecognizerInputDataConfigElAnnotationsEl {
    #[doc = "Set the field `test_s3_uri`.\n"]
    pub fn set_test_s3_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.test_s3_uri = Some(v.into());
        self
    }
}
impl ToListMappable for ComprehendEntityRecognizerInputDataConfigElAnnotationsEl {
    type O = BlockAssignable<ComprehendEntityRecognizerInputDataConfigElAnnotationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendEntityRecognizerInputDataConfigElAnnotationsEl {
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildComprehendEntityRecognizerInputDataConfigElAnnotationsEl {
    pub fn build(self) -> ComprehendEntityRecognizerInputDataConfigElAnnotationsEl {
        ComprehendEntityRecognizerInputDataConfigElAnnotationsEl {
            s3_uri: self.s3_uri,
            test_s3_uri: core::default::Default::default(),
        }
    }
}
pub struct ComprehendEntityRecognizerInputDataConfigElAnnotationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendEntityRecognizerInputDataConfigElAnnotationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComprehendEntityRecognizerInputDataConfigElAnnotationsElRef {
        ComprehendEntityRecognizerInputDataConfigElAnnotationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendEntityRecognizerInputDataConfigElAnnotationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `test_s3_uri` after provisioning.\n"]
    pub fn test_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.test_s3_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotation_data_s3_uri: Option<PrimField<String>>,
    attribute_names: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_type: Option<PrimField<String>>,
    s3_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_documents_s3_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split: Option<PrimField<String>>,
}
impl ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsEl {
    #[doc = "Set the field `annotation_data_s3_uri`.\n"]
    pub fn set_annotation_data_s3_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.annotation_data_s3_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `document_type`.\n"]
    pub fn set_document_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document_type = Some(v.into());
        self
    }
    #[doc = "Set the field `source_documents_s3_uri`.\n"]
    pub fn set_source_documents_s3_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_documents_s3_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `split`.\n"]
    pub fn set_split(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.split = Some(v.into());
        self
    }
}
impl ToListMappable for ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsEl {
    type O = BlockAssignable<ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendEntityRecognizerInputDataConfigElAugmentedManifestsEl {
    #[doc = ""]
    pub attribute_names: ListField<PrimField<String>>,
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildComprehendEntityRecognizerInputDataConfigElAugmentedManifestsEl {
    pub fn build(self) -> ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsEl {
        ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsEl {
            annotation_data_s3_uri: core::default::Default::default(),
            attribute_names: self.attribute_names,
            document_type: core::default::Default::default(),
            s3_uri: self.s3_uri,
            source_documents_s3_uri: core::default::Default::default(),
            split: core::default::Default::default(),
        }
    }
}
pub struct ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsElRef {
        ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotation_data_s3_uri` after provisioning.\n"]
    pub fn annotation_data_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.annotation_data_s3_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `attribute_names` after provisioning.\n"]
    pub fn attribute_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attribute_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `document_type` after provisioning.\n"]
    pub fn document_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `source_documents_s3_uri` after provisioning.\n"]
    pub fn source_documents_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_documents_s3_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `split` after provisioning.\n"]
    pub fn split(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.split", self.base))
    }
}
#[derive(Serialize)]
pub struct ComprehendEntityRecognizerInputDataConfigElDocumentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_format: Option<PrimField<String>>,
    s3_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_s3_uri: Option<PrimField<String>>,
}
impl ComprehendEntityRecognizerInputDataConfigElDocumentsEl {
    #[doc = "Set the field `input_format`.\n"]
    pub fn set_input_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_format = Some(v.into());
        self
    }
    #[doc = "Set the field `test_s3_uri`.\n"]
    pub fn set_test_s3_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.test_s3_uri = Some(v.into());
        self
    }
}
impl ToListMappable for ComprehendEntityRecognizerInputDataConfigElDocumentsEl {
    type O = BlockAssignable<ComprehendEntityRecognizerInputDataConfigElDocumentsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendEntityRecognizerInputDataConfigElDocumentsEl {
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildComprehendEntityRecognizerInputDataConfigElDocumentsEl {
    pub fn build(self) -> ComprehendEntityRecognizerInputDataConfigElDocumentsEl {
        ComprehendEntityRecognizerInputDataConfigElDocumentsEl {
            input_format: core::default::Default::default(),
            s3_uri: self.s3_uri,
            test_s3_uri: core::default::Default::default(),
        }
    }
}
pub struct ComprehendEntityRecognizerInputDataConfigElDocumentsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendEntityRecognizerInputDataConfigElDocumentsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComprehendEntityRecognizerInputDataConfigElDocumentsElRef {
        ComprehendEntityRecognizerInputDataConfigElDocumentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendEntityRecognizerInputDataConfigElDocumentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `input_format` after provisioning.\n"]
    pub fn input_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_format", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `test_s3_uri` after provisioning.\n"]
    pub fn test_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.test_s3_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct ComprehendEntityRecognizerInputDataConfigElEntityListEl {
    s3_uri: PrimField<String>,
}
impl ComprehendEntityRecognizerInputDataConfigElEntityListEl {}
impl ToListMappable for ComprehendEntityRecognizerInputDataConfigElEntityListEl {
    type O = BlockAssignable<ComprehendEntityRecognizerInputDataConfigElEntityListEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendEntityRecognizerInputDataConfigElEntityListEl {
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildComprehendEntityRecognizerInputDataConfigElEntityListEl {
    pub fn build(self) -> ComprehendEntityRecognizerInputDataConfigElEntityListEl {
        ComprehendEntityRecognizerInputDataConfigElEntityListEl {
            s3_uri: self.s3_uri,
        }
    }
}
pub struct ComprehendEntityRecognizerInputDataConfigElEntityListElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendEntityRecognizerInputDataConfigElEntityListElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComprehendEntityRecognizerInputDataConfigElEntityListElRef {
        ComprehendEntityRecognizerInputDataConfigElEntityListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendEntityRecognizerInputDataConfigElEntityListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct ComprehendEntityRecognizerInputDataConfigElEntityTypesEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ComprehendEntityRecognizerInputDataConfigElEntityTypesEl {}
impl ToListMappable for ComprehendEntityRecognizerInputDataConfigElEntityTypesEl {
    type O = BlockAssignable<ComprehendEntityRecognizerInputDataConfigElEntityTypesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendEntityRecognizerInputDataConfigElEntityTypesEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildComprehendEntityRecognizerInputDataConfigElEntityTypesEl {
    pub fn build(self) -> ComprehendEntityRecognizerInputDataConfigElEntityTypesEl {
        ComprehendEntityRecognizerInputDataConfigElEntityTypesEl { type_: self.type_ }
    }
}
pub struct ComprehendEntityRecognizerInputDataConfigElEntityTypesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendEntityRecognizerInputDataConfigElEntityTypesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComprehendEntityRecognizerInputDataConfigElEntityTypesElRef {
        ComprehendEntityRecognizerInputDataConfigElEntityTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendEntityRecognizerInputDataConfigElEntityTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComprehendEntityRecognizerInputDataConfigElDynamic {
    annotations: Option<DynamicBlock<ComprehendEntityRecognizerInputDataConfigElAnnotationsEl>>,
    augmented_manifests:
        Option<DynamicBlock<ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsEl>>,
    documents: Option<DynamicBlock<ComprehendEntityRecognizerInputDataConfigElDocumentsEl>>,
    entity_list: Option<DynamicBlock<ComprehendEntityRecognizerInputDataConfigElEntityListEl>>,
    entity_types: Option<DynamicBlock<ComprehendEntityRecognizerInputDataConfigElEntityTypesEl>>,
}
#[derive(Serialize)]
pub struct ComprehendEntityRecognizerInputDataConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<Vec<ComprehendEntityRecognizerInputDataConfigElAnnotationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    augmented_manifests:
        Option<Vec<ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documents: Option<Vec<ComprehendEntityRecognizerInputDataConfigElDocumentsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_list: Option<Vec<ComprehendEntityRecognizerInputDataConfigElEntityListEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_types: Option<Vec<ComprehendEntityRecognizerInputDataConfigElEntityTypesEl>>,
    dynamic: ComprehendEntityRecognizerInputDataConfigElDynamic,
}
impl ComprehendEntityRecognizerInputDataConfigEl {
    #[doc = "Set the field `data_format`.\n"]
    pub fn set_data_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_format = Some(v.into());
        self
    }
    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(
        mut self,
        v: impl Into<BlockAssignable<ComprehendEntityRecognizerInputDataConfigElAnnotationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.annotations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.annotations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `augmented_manifests`.\n"]
    pub fn set_augmented_manifests(
        mut self,
        v: impl Into<BlockAssignable<ComprehendEntityRecognizerInputDataConfigElAugmentedManifestsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.augmented_manifests = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.augmented_manifests = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `documents`.\n"]
    pub fn set_documents(
        mut self,
        v: impl Into<BlockAssignable<ComprehendEntityRecognizerInputDataConfigElDocumentsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.documents = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.documents = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `entity_list`.\n"]
    pub fn set_entity_list(
        mut self,
        v: impl Into<BlockAssignable<ComprehendEntityRecognizerInputDataConfigElEntityListEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.entity_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.entity_list = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `entity_types`.\n"]
    pub fn set_entity_types(
        mut self,
        v: impl Into<BlockAssignable<ComprehendEntityRecognizerInputDataConfigElEntityTypesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.entity_types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.entity_types = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComprehendEntityRecognizerInputDataConfigEl {
    type O = BlockAssignable<ComprehendEntityRecognizerInputDataConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendEntityRecognizerInputDataConfigEl {}
impl BuildComprehendEntityRecognizerInputDataConfigEl {
    pub fn build(self) -> ComprehendEntityRecognizerInputDataConfigEl {
        ComprehendEntityRecognizerInputDataConfigEl {
            data_format: core::default::Default::default(),
            annotations: core::default::Default::default(),
            augmented_manifests: core::default::Default::default(),
            documents: core::default::Default::default(),
            entity_list: core::default::Default::default(),
            entity_types: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComprehendEntityRecognizerInputDataConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendEntityRecognizerInputDataConfigElRef {
    fn new(shared: StackShared, base: String) -> ComprehendEntityRecognizerInputDataConfigElRef {
        ComprehendEntityRecognizerInputDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendEntityRecognizerInputDataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_format` after provisioning.\n"]
    pub fn data_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_format", self.base))
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(
        &self,
    ) -> ListRef<ComprehendEntityRecognizerInputDataConfigElAnnotationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `documents` after provisioning.\n"]
    pub fn documents(&self) -> ListRef<ComprehendEntityRecognizerInputDataConfigElDocumentsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.documents", self.base))
    }
    #[doc = "Get a reference to the value of field `entity_list` after provisioning.\n"]
    pub fn entity_list(
        &self,
    ) -> ListRef<ComprehendEntityRecognizerInputDataConfigElEntityListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.entity_list", self.base))
    }
}
#[derive(Serialize)]
pub struct ComprehendEntityRecognizerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComprehendEntityRecognizerTimeoutsEl {
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
impl ToListMappable for ComprehendEntityRecognizerTimeoutsEl {
    type O = BlockAssignable<ComprehendEntityRecognizerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendEntityRecognizerTimeoutsEl {}
impl BuildComprehendEntityRecognizerTimeoutsEl {
    pub fn build(self) -> ComprehendEntityRecognizerTimeoutsEl {
        ComprehendEntityRecognizerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComprehendEntityRecognizerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendEntityRecognizerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComprehendEntityRecognizerTimeoutsElRef {
        ComprehendEntityRecognizerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendEntityRecognizerTimeoutsElRef {
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
pub struct ComprehendEntityRecognizerVpcConfigEl {
    security_group_ids: SetField<PrimField<String>>,
    subnets: SetField<PrimField<String>>,
}
impl ComprehendEntityRecognizerVpcConfigEl {}
impl ToListMappable for ComprehendEntityRecognizerVpcConfigEl {
    type O = BlockAssignable<ComprehendEntityRecognizerVpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendEntityRecognizerVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}
impl BuildComprehendEntityRecognizerVpcConfigEl {
    pub fn build(self) -> ComprehendEntityRecognizerVpcConfigEl {
        ComprehendEntityRecognizerVpcConfigEl {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}
pub struct ComprehendEntityRecognizerVpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendEntityRecognizerVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> ComprehendEntityRecognizerVpcConfigElRef {
        ComprehendEntityRecognizerVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendEntityRecognizerVpcConfigElRef {
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
}
#[derive(Serialize, Default)]
struct ComprehendEntityRecognizerDynamic {
    input_data_config: Option<DynamicBlock<ComprehendEntityRecognizerInputDataConfigEl>>,
    vpc_config: Option<DynamicBlock<ComprehendEntityRecognizerVpcConfigEl>>,
}
