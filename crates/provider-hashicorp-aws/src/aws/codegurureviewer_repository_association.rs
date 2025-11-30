use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CodegurureviewerRepositoryAssociationData {
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
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_details: Option<Vec<CodegurureviewerRepositoryAssociationKmsKeyDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<Vec<CodegurureviewerRepositoryAssociationRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CodegurureviewerRepositoryAssociationTimeoutsEl>,
    dynamic: CodegurureviewerRepositoryAssociationDynamic,
}

struct CodegurureviewerRepositoryAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodegurureviewerRepositoryAssociationData>,
}

#[derive(Clone)]
pub struct CodegurureviewerRepositoryAssociation(Rc<CodegurureviewerRepositoryAssociation_>);

impl CodegurureviewerRepositoryAssociation {
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

    #[doc = "Set the field `kms_key_details`.\n"]
    pub fn set_kms_key_details(
        self,
        v: impl Into<BlockAssignable<CodegurureviewerRepositoryAssociationKmsKeyDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kms_key_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kms_key_details = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(
        self,
        v: impl Into<BlockAssignable<CodegurureviewerRepositoryAssociationRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.repository = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<CodegurureviewerRepositoryAssociationTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `connection_arn` after provisioning.\n"]
    pub fn connection_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `provider_type` after provisioning.\n"]
    pub fn provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_repository_details` after provisioning.\n"]
    pub fn s3_repository_details(
        &self,
    ) -> ListRef<CodegurureviewerRepositoryAssociationS3RepositoryDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_repository_details", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\n"]
    pub fn state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_reason", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `kms_key_details` after provisioning.\n"]
    pub fn kms_key_details(
        &self,
    ) -> ListRef<CodegurureviewerRepositoryAssociationKmsKeyDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kms_key_details", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> ListRef<CodegurureviewerRepositoryAssociationRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repository", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CodegurureviewerRepositoryAssociationTimeoutsElRef {
        CodegurureviewerRepositoryAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for CodegurureviewerRepositoryAssociation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CodegurureviewerRepositoryAssociation {}

impl ToListMappable for CodegurureviewerRepositoryAssociation {
    type O = ListRef<CodegurureviewerRepositoryAssociationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CodegurureviewerRepositoryAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_codegurureviewer_repository_association".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodegurureviewerRepositoryAssociation {
    pub tf_id: String,
}

impl BuildCodegurureviewerRepositoryAssociation {
    pub fn build(self, stack: &mut Stack) -> CodegurureviewerRepositoryAssociation {
        let out = CodegurureviewerRepositoryAssociation(Rc::new(
            CodegurureviewerRepositoryAssociation_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(CodegurureviewerRepositoryAssociationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    tags: core::default::Default::default(),
                    tags_all: core::default::Default::default(),
                    kms_key_details: core::default::Default::default(),
                    repository: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodegurureviewerRepositoryAssociationRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodegurureviewerRepositoryAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CodegurureviewerRepositoryAssociationRef {
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

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `connection_arn` after provisioning.\n"]
    pub fn connection_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `provider_type` after provisioning.\n"]
    pub fn provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_repository_details` after provisioning.\n"]
    pub fn s3_repository_details(
        &self,
    ) -> ListRef<CodegurureviewerRepositoryAssociationS3RepositoryDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_repository_details", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\n"]
    pub fn state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_reason", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `kms_key_details` after provisioning.\n"]
    pub fn kms_key_details(
        &self,
    ) -> ListRef<CodegurureviewerRepositoryAssociationKmsKeyDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kms_key_details", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> ListRef<CodegurureviewerRepositoryAssociationRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repository", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CodegurureviewerRepositoryAssociationTimeoutsElRef {
        CodegurureviewerRepositoryAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_artifacts_object_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_code_artifacts_object_key: Option<PrimField<String>>,
}

impl CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsEl {
    #[doc = "Set the field `build_artifacts_object_key`.\n"]
    pub fn set_build_artifacts_object_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_artifacts_object_key = Some(v.into());
        self
    }

    #[doc = "Set the field `source_code_artifacts_object_key`.\n"]
    pub fn set_source_code_artifacts_object_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_code_artifacts_object_key = Some(v.into());
        self
    }
}

impl ToListMappable for CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsEl {
    type O =
        BlockAssignable<CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsEl {}

impl BuildCodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsEl {
    pub fn build(
        self,
    ) -> CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsEl {
        CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsEl {
            build_artifacts_object_key: core::default::Default::default(),
            source_code_artifacts_object_key: core::default::Default::default(),
        }
    }
}

pub struct CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsElRef {
        CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `build_artifacts_object_key` after provisioning.\n"]
    pub fn build_artifacts_object_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_artifacts_object_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_code_artifacts_object_key` after provisioning.\n"]
    pub fn source_code_artifacts_object_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_code_artifacts_object_key", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociationS3RepositoryDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_artifacts: Option<
        ListField<CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsEl>,
    >,
}

impl CodegurureviewerRepositoryAssociationS3RepositoryDetailsEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `code_artifacts`.\n"]
    pub fn set_code_artifacts(
        mut self,
        v: impl Into<
            ListField<CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsEl>,
        >,
    ) -> Self {
        self.code_artifacts = Some(v.into());
        self
    }
}

impl ToListMappable for CodegurureviewerRepositoryAssociationS3RepositoryDetailsEl {
    type O = BlockAssignable<CodegurureviewerRepositoryAssociationS3RepositoryDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociationS3RepositoryDetailsEl {}

impl BuildCodegurureviewerRepositoryAssociationS3RepositoryDetailsEl {
    pub fn build(self) -> CodegurureviewerRepositoryAssociationS3RepositoryDetailsEl {
        CodegurureviewerRepositoryAssociationS3RepositoryDetailsEl {
            bucket_name: core::default::Default::default(),
            code_artifacts: core::default::Default::default(),
        }
    }
}

pub struct CodegurureviewerRepositoryAssociationS3RepositoryDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodegurureviewerRepositoryAssociationS3RepositoryDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodegurureviewerRepositoryAssociationS3RepositoryDetailsElRef {
        CodegurureviewerRepositoryAssociationS3RepositoryDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociationS3RepositoryDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `code_artifacts` after provisioning.\n"]
    pub fn code_artifacts(
        &self,
    ) -> ListRef<CodegurureviewerRepositoryAssociationS3RepositoryDetailsElCodeArtifactsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code_artifacts", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociationKmsKeyDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
}

impl CodegurureviewerRepositoryAssociationKmsKeyDetailsEl {
    #[doc = "Set the field `encryption_option`.\n"]
    pub fn set_encryption_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_option = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for CodegurureviewerRepositoryAssociationKmsKeyDetailsEl {
    type O = BlockAssignable<CodegurureviewerRepositoryAssociationKmsKeyDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociationKmsKeyDetailsEl {}

impl BuildCodegurureviewerRepositoryAssociationKmsKeyDetailsEl {
    pub fn build(self) -> CodegurureviewerRepositoryAssociationKmsKeyDetailsEl {
        CodegurureviewerRepositoryAssociationKmsKeyDetailsEl {
            encryption_option: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct CodegurureviewerRepositoryAssociationKmsKeyDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodegurureviewerRepositoryAssociationKmsKeyDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodegurureviewerRepositoryAssociationKmsKeyDetailsElRef {
        CodegurureviewerRepositoryAssociationKmsKeyDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociationKmsKeyDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_option` after provisioning.\n"]
    pub fn encryption_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_option", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociationRepositoryElBitbucketEl {
    connection_arn: PrimField<String>,
    name: PrimField<String>,
    owner: PrimField<String>,
}

impl CodegurureviewerRepositoryAssociationRepositoryElBitbucketEl {}

impl ToListMappable for CodegurureviewerRepositoryAssociationRepositoryElBitbucketEl {
    type O = BlockAssignable<CodegurureviewerRepositoryAssociationRepositoryElBitbucketEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociationRepositoryElBitbucketEl {
    #[doc = ""]
    pub connection_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub owner: PrimField<String>,
}

impl BuildCodegurureviewerRepositoryAssociationRepositoryElBitbucketEl {
    pub fn build(self) -> CodegurureviewerRepositoryAssociationRepositoryElBitbucketEl {
        CodegurureviewerRepositoryAssociationRepositoryElBitbucketEl {
            connection_arn: self.connection_arn,
            name: self.name,
            owner: self.owner,
        }
    }
}

pub struct CodegurureviewerRepositoryAssociationRepositoryElBitbucketElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodegurureviewerRepositoryAssociationRepositoryElBitbucketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodegurureviewerRepositoryAssociationRepositoryElBitbucketElRef {
        CodegurureviewerRepositoryAssociationRepositoryElBitbucketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociationRepositoryElBitbucketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_arn` after provisioning.\n"]
    pub fn connection_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociationRepositoryElCodecommitEl {
    name: PrimField<String>,
}

impl CodegurureviewerRepositoryAssociationRepositoryElCodecommitEl {}

impl ToListMappable for CodegurureviewerRepositoryAssociationRepositoryElCodecommitEl {
    type O = BlockAssignable<CodegurureviewerRepositoryAssociationRepositoryElCodecommitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociationRepositoryElCodecommitEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCodegurureviewerRepositoryAssociationRepositoryElCodecommitEl {
    pub fn build(self) -> CodegurureviewerRepositoryAssociationRepositoryElCodecommitEl {
        CodegurureviewerRepositoryAssociationRepositoryElCodecommitEl { name: self.name }
    }
}

pub struct CodegurureviewerRepositoryAssociationRepositoryElCodecommitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodegurureviewerRepositoryAssociationRepositoryElCodecommitElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodegurureviewerRepositoryAssociationRepositoryElCodecommitElRef {
        CodegurureviewerRepositoryAssociationRepositoryElCodecommitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociationRepositoryElCodecommitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerEl {
    connection_arn: PrimField<String>,
    name: PrimField<String>,
    owner: PrimField<String>,
}

impl CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerEl {}

impl ToListMappable for CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerEl {
    type O =
        BlockAssignable<CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerEl {
    #[doc = ""]
    pub connection_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub owner: PrimField<String>,
}

impl BuildCodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerEl {
    pub fn build(
        self,
    ) -> CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerEl {
        CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerEl {
            connection_arn: self.connection_arn,
            name: self.name,
            owner: self.owner,
        }
    }
}

pub struct CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerElRef {
        CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_arn` after provisioning.\n"]
    pub fn connection_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociationRepositoryElS3BucketEl {
    bucket_name: PrimField<String>,
    name: PrimField<String>,
}

impl CodegurureviewerRepositoryAssociationRepositoryElS3BucketEl {}

impl ToListMappable for CodegurureviewerRepositoryAssociationRepositoryElS3BucketEl {
    type O = BlockAssignable<CodegurureviewerRepositoryAssociationRepositoryElS3BucketEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociationRepositoryElS3BucketEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCodegurureviewerRepositoryAssociationRepositoryElS3BucketEl {
    pub fn build(self) -> CodegurureviewerRepositoryAssociationRepositoryElS3BucketEl {
        CodegurureviewerRepositoryAssociationRepositoryElS3BucketEl {
            bucket_name: self.bucket_name,
            name: self.name,
        }
    }
}

pub struct CodegurureviewerRepositoryAssociationRepositoryElS3BucketElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodegurureviewerRepositoryAssociationRepositoryElS3BucketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodegurureviewerRepositoryAssociationRepositoryElS3BucketElRef {
        CodegurureviewerRepositoryAssociationRepositoryElS3BucketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociationRepositoryElS3BucketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodegurureviewerRepositoryAssociationRepositoryElDynamic {
    bitbucket: Option<DynamicBlock<CodegurureviewerRepositoryAssociationRepositoryElBitbucketEl>>,
    codecommit: Option<DynamicBlock<CodegurureviewerRepositoryAssociationRepositoryElCodecommitEl>>,
    github_enterprise_server: Option<
        DynamicBlock<CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerEl>,
    >,
    s3_bucket: Option<DynamicBlock<CodegurureviewerRepositoryAssociationRepositoryElS3BucketEl>>,
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociationRepositoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket: Option<Vec<CodegurureviewerRepositoryAssociationRepositoryElBitbucketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codecommit: Option<Vec<CodegurureviewerRepositoryAssociationRepositoryElCodecommitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_enterprise_server:
        Option<Vec<CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket: Option<Vec<CodegurureviewerRepositoryAssociationRepositoryElS3BucketEl>>,
    dynamic: CodegurureviewerRepositoryAssociationRepositoryElDynamic,
}

impl CodegurureviewerRepositoryAssociationRepositoryEl {
    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(
        mut self,
        v: impl Into<BlockAssignable<CodegurureviewerRepositoryAssociationRepositoryElBitbucketEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bitbucket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bitbucket = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `codecommit`.\n"]
    pub fn set_codecommit(
        mut self,
        v: impl Into<BlockAssignable<CodegurureviewerRepositoryAssociationRepositoryElCodecommitEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.codecommit = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.codecommit = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `github_enterprise_server`.\n"]
    pub fn set_github_enterprise_server(
        mut self,
        v: impl Into<
            BlockAssignable<
                CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github_enterprise_server = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github_enterprise_server = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(
        mut self,
        v: impl Into<BlockAssignable<CodegurureviewerRepositoryAssociationRepositoryElS3BucketEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_bucket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_bucket = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CodegurureviewerRepositoryAssociationRepositoryEl {
    type O = BlockAssignable<CodegurureviewerRepositoryAssociationRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociationRepositoryEl {}

impl BuildCodegurureviewerRepositoryAssociationRepositoryEl {
    pub fn build(self) -> CodegurureviewerRepositoryAssociationRepositoryEl {
        CodegurureviewerRepositoryAssociationRepositoryEl {
            bitbucket: core::default::Default::default(),
            codecommit: core::default::Default::default(),
            github_enterprise_server: core::default::Default::default(),
            s3_bucket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodegurureviewerRepositoryAssociationRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodegurureviewerRepositoryAssociationRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodegurureviewerRepositoryAssociationRepositoryElRef {
        CodegurureviewerRepositoryAssociationRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociationRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(
        &self,
    ) -> ListRef<CodegurureviewerRepositoryAssociationRepositoryElBitbucketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
    }

    #[doc = "Get a reference to the value of field `codecommit` after provisioning.\n"]
    pub fn codecommit(
        &self,
    ) -> ListRef<CodegurureviewerRepositoryAssociationRepositoryElCodecommitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.codecommit", self.base))
    }

    #[doc = "Get a reference to the value of field `github_enterprise_server` after provisioning.\n"]
    pub fn github_enterprise_server(
        &self,
    ) -> ListRef<CodegurureviewerRepositoryAssociationRepositoryElGithubEnterpriseServerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github_enterprise_server", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(
        &self,
    ) -> ListRef<CodegurureviewerRepositoryAssociationRepositoryElS3BucketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl CodegurureviewerRepositoryAssociationTimeoutsEl {
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

impl ToListMappable for CodegurureviewerRepositoryAssociationTimeoutsEl {
    type O = BlockAssignable<CodegurureviewerRepositoryAssociationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociationTimeoutsEl {}

impl BuildCodegurureviewerRepositoryAssociationTimeoutsEl {
    pub fn build(self) -> CodegurureviewerRepositoryAssociationTimeoutsEl {
        CodegurureviewerRepositoryAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CodegurureviewerRepositoryAssociationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodegurureviewerRepositoryAssociationTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CodegurureviewerRepositoryAssociationTimeoutsElRef {
        CodegurureviewerRepositoryAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociationTimeoutsElRef {
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
struct CodegurureviewerRepositoryAssociationDynamic {
    kms_key_details: Option<DynamicBlock<CodegurureviewerRepositoryAssociationKmsKeyDetailsEl>>,
    repository: Option<DynamicBlock<CodegurureviewerRepositoryAssociationRepositoryEl>>,
}
