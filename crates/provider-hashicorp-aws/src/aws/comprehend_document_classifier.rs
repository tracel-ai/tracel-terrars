use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComprehendDocumentClassifierData {
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
    mode: Option<PrimField<String>>,
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
    input_data_config: Option<Vec<ComprehendDocumentClassifierInputDataConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_data_config: Option<Vec<ComprehendDocumentClassifierOutputDataConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComprehendDocumentClassifierTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<ComprehendDocumentClassifierVpcConfigEl>>,
    dynamic: ComprehendDocumentClassifierDynamic,
}
struct ComprehendDocumentClassifier_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComprehendDocumentClassifierData>,
}
#[derive(Clone)]
pub struct ComprehendDocumentClassifier(Rc<ComprehendDocumentClassifier_>);
impl ComprehendDocumentClassifier {
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
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mode = Some(v.into());
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
        v: impl Into<BlockAssignable<ComprehendDocumentClassifierInputDataConfigEl>>,
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
    #[doc = "Set the field `output_data_config`.\n"]
    pub fn set_output_data_config(
        self,
        v: impl Into<BlockAssignable<ComprehendDocumentClassifierOutputDataConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_data_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_data_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComprehendDocumentClassifierTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl Into<BlockAssignable<ComprehendDocumentClassifierVpcConfigEl>>,
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
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
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
    pub fn input_data_config(&self) -> ListRef<ComprehendDocumentClassifierInputDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> ListRef<ComprehendDocumentClassifierOutputDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComprehendDocumentClassifierTimeoutsElRef {
        ComprehendDocumentClassifierTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<ComprehendDocumentClassifierVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
impl Referable for ComprehendDocumentClassifier {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComprehendDocumentClassifier {}
impl ToListMappable for ComprehendDocumentClassifier {
    type O = ListRef<ComprehendDocumentClassifierRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComprehendDocumentClassifier_ {
    fn extract_resource_type(&self) -> String {
        "aws_comprehend_document_classifier".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComprehendDocumentClassifier {
    pub tf_id: String,
    #[doc = ""]
    pub data_access_role_arn: PrimField<String>,
    #[doc = ""]
    pub language_code: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildComprehendDocumentClassifier {
    pub fn build(self, stack: &mut Stack) -> ComprehendDocumentClassifier {
        let out = ComprehendDocumentClassifier(Rc::new(ComprehendDocumentClassifier_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComprehendDocumentClassifierData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_access_role_arn: self.data_access_role_arn,
                id: core::default::Default::default(),
                language_code: self.language_code,
                mode: core::default::Default::default(),
                model_kms_key_id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version_name: core::default::Default::default(),
                version_name_prefix: core::default::Default::default(),
                volume_kms_key_id: core::default::Default::default(),
                input_data_config: core::default::Default::default(),
                output_data_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComprehendDocumentClassifierRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendDocumentClassifierRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComprehendDocumentClassifierRef {
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
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
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
    pub fn input_data_config(&self) -> ListRef<ComprehendDocumentClassifierInputDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> ListRef<ComprehendDocumentClassifierOutputDataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_data_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComprehendDocumentClassifierTimeoutsElRef {
        ComprehendDocumentClassifierTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<ComprehendDocumentClassifierVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
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
impl ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
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
impl ToListMappable for ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
    type O = BlockAssignable<ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
    #[doc = ""]
    pub attribute_names: ListField<PrimField<String>>,
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
    pub fn build(self) -> ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
        ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
            annotation_data_s3_uri: core::default::Default::default(),
            attribute_names: self.attribute_names,
            document_type: core::default::Default::default(),
            s3_uri: self.s3_uri,
            source_documents_s3_uri: core::default::Default::default(),
            split: core::default::Default::default(),
        }
    }
}
pub struct ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsElRef {
        ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsElRef {
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
#[derive(Serialize, Default)]
struct ComprehendDocumentClassifierInputDataConfigElDynamic {
    augmented_manifests:
        Option<DynamicBlock<ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl>>,
}
#[derive(Serialize)]
pub struct ComprehendDocumentClassifierInputDataConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_s3_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    augmented_manifests:
        Option<Vec<ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl>>,
    dynamic: ComprehendDocumentClassifierInputDataConfigElDynamic,
}
impl ComprehendDocumentClassifierInputDataConfigEl {
    #[doc = "Set the field `data_format`.\n"]
    pub fn set_data_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_format = Some(v.into());
        self
    }
    #[doc = "Set the field `label_delimiter`.\n"]
    pub fn set_label_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label_delimiter = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `test_s3_uri`.\n"]
    pub fn set_test_s3_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.test_s3_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `augmented_manifests`.\n"]
    pub fn set_augmented_manifests(
        mut self,
        v: impl Into<BlockAssignable<ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl>>,
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
}
impl ToListMappable for ComprehendDocumentClassifierInputDataConfigEl {
    type O = BlockAssignable<ComprehendDocumentClassifierInputDataConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendDocumentClassifierInputDataConfigEl {}
impl BuildComprehendDocumentClassifierInputDataConfigEl {
    pub fn build(self) -> ComprehendDocumentClassifierInputDataConfigEl {
        ComprehendDocumentClassifierInputDataConfigEl {
            data_format: core::default::Default::default(),
            label_delimiter: core::default::Default::default(),
            s3_uri: core::default::Default::default(),
            test_s3_uri: core::default::Default::default(),
            augmented_manifests: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComprehendDocumentClassifierInputDataConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendDocumentClassifierInputDataConfigElRef {
    fn new(shared: StackShared, base: String) -> ComprehendDocumentClassifierInputDataConfigElRef {
        ComprehendDocumentClassifierInputDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendDocumentClassifierInputDataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_format` after provisioning.\n"]
    pub fn data_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_format", self.base))
    }
    #[doc = "Get a reference to the value of field `label_delimiter` after provisioning.\n"]
    pub fn label_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_delimiter", self.base),
        )
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
pub struct ComprehendDocumentClassifierOutputDataConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    s3_uri: PrimField<String>,
}
impl ComprehendDocumentClassifierOutputDataConfigEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}
impl ToListMappable for ComprehendDocumentClassifierOutputDataConfigEl {
    type O = BlockAssignable<ComprehendDocumentClassifierOutputDataConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendDocumentClassifierOutputDataConfigEl {
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildComprehendDocumentClassifierOutputDataConfigEl {
    pub fn build(self) -> ComprehendDocumentClassifierOutputDataConfigEl {
        ComprehendDocumentClassifierOutputDataConfigEl {
            kms_key_id: core::default::Default::default(),
            s3_uri: self.s3_uri,
        }
    }
}
pub struct ComprehendDocumentClassifierOutputDataConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendDocumentClassifierOutputDataConfigElRef {
    fn new(shared: StackShared, base: String) -> ComprehendDocumentClassifierOutputDataConfigElRef {
        ComprehendDocumentClassifierOutputDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendDocumentClassifierOutputDataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
    #[doc = "Get a reference to the value of field `output_s3_uri` after provisioning.\n"]
    pub fn output_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_s3_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct ComprehendDocumentClassifierTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComprehendDocumentClassifierTimeoutsEl {
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
impl ToListMappable for ComprehendDocumentClassifierTimeoutsEl {
    type O = BlockAssignable<ComprehendDocumentClassifierTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendDocumentClassifierTimeoutsEl {}
impl BuildComprehendDocumentClassifierTimeoutsEl {
    pub fn build(self) -> ComprehendDocumentClassifierTimeoutsEl {
        ComprehendDocumentClassifierTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComprehendDocumentClassifierTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendDocumentClassifierTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComprehendDocumentClassifierTimeoutsElRef {
        ComprehendDocumentClassifierTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendDocumentClassifierTimeoutsElRef {
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
pub struct ComprehendDocumentClassifierVpcConfigEl {
    security_group_ids: SetField<PrimField<String>>,
    subnets: SetField<PrimField<String>>,
}
impl ComprehendDocumentClassifierVpcConfigEl {}
impl ToListMappable for ComprehendDocumentClassifierVpcConfigEl {
    type O = BlockAssignable<ComprehendDocumentClassifierVpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComprehendDocumentClassifierVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}
impl BuildComprehendDocumentClassifierVpcConfigEl {
    pub fn build(self) -> ComprehendDocumentClassifierVpcConfigEl {
        ComprehendDocumentClassifierVpcConfigEl {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}
pub struct ComprehendDocumentClassifierVpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComprehendDocumentClassifierVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> ComprehendDocumentClassifierVpcConfigElRef {
        ComprehendDocumentClassifierVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComprehendDocumentClassifierVpcConfigElRef {
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
struct ComprehendDocumentClassifierDynamic {
    input_data_config: Option<DynamicBlock<ComprehendDocumentClassifierInputDataConfigEl>>,
    output_data_config: Option<DynamicBlock<ComprehendDocumentClassifierOutputDataConfigEl>>,
    vpc_config: Option<DynamicBlock<ComprehendDocumentClassifierVpcConfigEl>>,
}
