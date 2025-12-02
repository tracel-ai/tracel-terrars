use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ElastictranscoderPipelineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    input_bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_config: Option<Vec<ElastictranscoderPipelineContentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_config_permissions: Option<Vec<ElastictranscoderPipelineContentConfigPermissionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notifications: Option<Vec<ElastictranscoderPipelineNotificationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_config: Option<Vec<ElastictranscoderPipelineThumbnailConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_config_permissions:
        Option<Vec<ElastictranscoderPipelineThumbnailConfigPermissionsEl>>,
    dynamic: ElastictranscoderPipelineDynamic,
}
struct ElastictranscoderPipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ElastictranscoderPipelineData>,
}
#[derive(Clone)]
pub struct ElastictranscoderPipeline(Rc<ElastictranscoderPipeline_>);
impl ElastictranscoderPipeline {
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
    #[doc = "Set the field `aws_kms_key_arn`.\n"]
    pub fn set_aws_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_kms_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `output_bucket`.\n"]
    pub fn set_output_bucket(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().output_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `content_config`.\n"]
    pub fn set_content_config(
        self,
        v: impl Into<BlockAssignable<ElastictranscoderPipelineContentConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `content_config_permissions`.\n"]
    pub fn set_content_config_permissions(
        self,
        v: impl Into<BlockAssignable<ElastictranscoderPipelineContentConfigPermissionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_config_permissions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_config_permissions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notifications`.\n"]
    pub fn set_notifications(
        self,
        v: impl Into<BlockAssignable<ElastictranscoderPipelineNotificationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notifications = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notifications = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `thumbnail_config`.\n"]
    pub fn set_thumbnail_config(
        self,
        v: impl Into<BlockAssignable<ElastictranscoderPipelineThumbnailConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thumbnail_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.thumbnail_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `thumbnail_config_permissions`.\n"]
    pub fn set_thumbnail_config_permissions(
        self,
        v: impl Into<BlockAssignable<ElastictranscoderPipelineThumbnailConfigPermissionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thumbnail_config_permissions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .thumbnail_config_permissions = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `aws_kms_key_arn` after provisioning.\n"]
    pub fn aws_kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input_bucket` after provisioning.\n"]
    pub fn input_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_bucket` after provisioning.\n"]
    pub fn output_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_config` after provisioning.\n"]
    pub fn content_config(&self) -> ListRef<ElastictranscoderPipelineContentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.content_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notifications` after provisioning.\n"]
    pub fn notifications(&self) -> ListRef<ElastictranscoderPipelineNotificationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notifications", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `thumbnail_config` after provisioning.\n"]
    pub fn thumbnail_config(&self) -> ListRef<ElastictranscoderPipelineThumbnailConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.thumbnail_config", self.extract_ref()),
        )
    }
}
impl Referable for ElastictranscoderPipeline {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ElastictranscoderPipeline {}
impl ToListMappable for ElastictranscoderPipeline {
    type O = ListRef<ElastictranscoderPipelineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ElastictranscoderPipeline_ {
    fn extract_resource_type(&self) -> String {
        "aws_elastictranscoder_pipeline".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildElastictranscoderPipeline {
    pub tf_id: String,
    #[doc = ""]
    pub input_bucket: PrimField<String>,
    #[doc = ""]
    pub role: PrimField<String>,
}
impl BuildElastictranscoderPipeline {
    pub fn build(self, stack: &mut Stack) -> ElastictranscoderPipeline {
        let out = ElastictranscoderPipeline(Rc::new(ElastictranscoderPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ElastictranscoderPipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                aws_kms_key_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                input_bucket: self.input_bucket,
                name: core::default::Default::default(),
                output_bucket: core::default::Default::default(),
                region: core::default::Default::default(),
                role: self.role,
                content_config: core::default::Default::default(),
                content_config_permissions: core::default::Default::default(),
                notifications: core::default::Default::default(),
                thumbnail_config: core::default::Default::default(),
                thumbnail_config_permissions: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ElastictranscoderPipelineRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ElastictranscoderPipelineRef {
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
    #[doc = "Get a reference to the value of field `aws_kms_key_arn` after provisioning.\n"]
    pub fn aws_kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input_bucket` after provisioning.\n"]
    pub fn input_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_bucket` after provisioning.\n"]
    pub fn output_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `content_config` after provisioning.\n"]
    pub fn content_config(&self) -> ListRef<ElastictranscoderPipelineContentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.content_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notifications` after provisioning.\n"]
    pub fn notifications(&self) -> ListRef<ElastictranscoderPipelineNotificationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notifications", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `thumbnail_config` after provisioning.\n"]
    pub fn thumbnail_config(&self) -> ListRef<ElastictranscoderPipelineThumbnailConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.thumbnail_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ElastictranscoderPipelineContentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
}
impl ElastictranscoderPipelineContentConfigEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_class = Some(v.into());
        self
    }
}
impl ToListMappable for ElastictranscoderPipelineContentConfigEl {
    type O = BlockAssignable<ElastictranscoderPipelineContentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElastictranscoderPipelineContentConfigEl {}
impl BuildElastictranscoderPipelineContentConfigEl {
    pub fn build(self) -> ElastictranscoderPipelineContentConfigEl {
        ElastictranscoderPipelineContentConfigEl {
            bucket: core::default::Default::default(),
            storage_class: core::default::Default::default(),
        }
    }
}
pub struct ElastictranscoderPipelineContentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPipelineContentConfigElRef {
    fn new(shared: StackShared, base: String) -> ElastictranscoderPipelineContentConfigElRef {
        ElastictranscoderPipelineContentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElastictranscoderPipelineContentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ElastictranscoderPipelineContentConfigPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grantee: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grantee_type: Option<PrimField<String>>,
}
impl ElastictranscoderPipelineContentConfigPermissionsEl {
    #[doc = "Set the field `access`.\n"]
    pub fn set_access(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.access = Some(v.into());
        self
    }
    #[doc = "Set the field `grantee`.\n"]
    pub fn set_grantee(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.grantee = Some(v.into());
        self
    }
    #[doc = "Set the field `grantee_type`.\n"]
    pub fn set_grantee_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.grantee_type = Some(v.into());
        self
    }
}
impl ToListMappable for ElastictranscoderPipelineContentConfigPermissionsEl {
    type O = BlockAssignable<ElastictranscoderPipelineContentConfigPermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElastictranscoderPipelineContentConfigPermissionsEl {}
impl BuildElastictranscoderPipelineContentConfigPermissionsEl {
    pub fn build(self) -> ElastictranscoderPipelineContentConfigPermissionsEl {
        ElastictranscoderPipelineContentConfigPermissionsEl {
            access: core::default::Default::default(),
            grantee: core::default::Default::default(),
            grantee_type: core::default::Default::default(),
        }
    }
}
pub struct ElastictranscoderPipelineContentConfigPermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPipelineContentConfigPermissionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ElastictranscoderPipelineContentConfigPermissionsElRef {
        ElastictranscoderPipelineContentConfigPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElastictranscoderPipelineContentConfigPermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access` after provisioning.\n"]
    pub fn access(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.access", self.base))
    }
    #[doc = "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grantee", self.base))
    }
    #[doc = "Get a reference to the value of field `grantee_type` after provisioning.\n"]
    pub fn grantee_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grantee_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ElastictranscoderPipelineNotificationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    completed: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    progressing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning: Option<PrimField<String>>,
}
impl ElastictranscoderPipelineNotificationsEl {
    #[doc = "Set the field `completed`.\n"]
    pub fn set_completed(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.completed = Some(v.into());
        self
    }
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error = Some(v.into());
        self
    }
    #[doc = "Set the field `progressing`.\n"]
    pub fn set_progressing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.progressing = Some(v.into());
        self
    }
    #[doc = "Set the field `warning`.\n"]
    pub fn set_warning(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.warning = Some(v.into());
        self
    }
}
impl ToListMappable for ElastictranscoderPipelineNotificationsEl {
    type O = BlockAssignable<ElastictranscoderPipelineNotificationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElastictranscoderPipelineNotificationsEl {}
impl BuildElastictranscoderPipelineNotificationsEl {
    pub fn build(self) -> ElastictranscoderPipelineNotificationsEl {
        ElastictranscoderPipelineNotificationsEl {
            completed: core::default::Default::default(),
            error: core::default::Default::default(),
            progressing: core::default::Default::default(),
            warning: core::default::Default::default(),
        }
    }
}
pub struct ElastictranscoderPipelineNotificationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPipelineNotificationsElRef {
    fn new(shared: StackShared, base: String) -> ElastictranscoderPipelineNotificationsElRef {
        ElastictranscoderPipelineNotificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElastictranscoderPipelineNotificationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `completed` after provisioning.\n"]
    pub fn completed(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.completed", self.base))
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error", self.base))
    }
    #[doc = "Get a reference to the value of field `progressing` after provisioning.\n"]
    pub fn progressing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.progressing", self.base))
    }
    #[doc = "Get a reference to the value of field `warning` after provisioning.\n"]
    pub fn warning(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.warning", self.base))
    }
}
#[derive(Serialize)]
pub struct ElastictranscoderPipelineThumbnailConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
}
impl ElastictranscoderPipelineThumbnailConfigEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_class = Some(v.into());
        self
    }
}
impl ToListMappable for ElastictranscoderPipelineThumbnailConfigEl {
    type O = BlockAssignable<ElastictranscoderPipelineThumbnailConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElastictranscoderPipelineThumbnailConfigEl {}
impl BuildElastictranscoderPipelineThumbnailConfigEl {
    pub fn build(self) -> ElastictranscoderPipelineThumbnailConfigEl {
        ElastictranscoderPipelineThumbnailConfigEl {
            bucket: core::default::Default::default(),
            storage_class: core::default::Default::default(),
        }
    }
}
pub struct ElastictranscoderPipelineThumbnailConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPipelineThumbnailConfigElRef {
    fn new(shared: StackShared, base: String) -> ElastictranscoderPipelineThumbnailConfigElRef {
        ElastictranscoderPipelineThumbnailConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElastictranscoderPipelineThumbnailConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ElastictranscoderPipelineThumbnailConfigPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grantee: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grantee_type: Option<PrimField<String>>,
}
impl ElastictranscoderPipelineThumbnailConfigPermissionsEl {
    #[doc = "Set the field `access`.\n"]
    pub fn set_access(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.access = Some(v.into());
        self
    }
    #[doc = "Set the field `grantee`.\n"]
    pub fn set_grantee(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.grantee = Some(v.into());
        self
    }
    #[doc = "Set the field `grantee_type`.\n"]
    pub fn set_grantee_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.grantee_type = Some(v.into());
        self
    }
}
impl ToListMappable for ElastictranscoderPipelineThumbnailConfigPermissionsEl {
    type O = BlockAssignable<ElastictranscoderPipelineThumbnailConfigPermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElastictranscoderPipelineThumbnailConfigPermissionsEl {}
impl BuildElastictranscoderPipelineThumbnailConfigPermissionsEl {
    pub fn build(self) -> ElastictranscoderPipelineThumbnailConfigPermissionsEl {
        ElastictranscoderPipelineThumbnailConfigPermissionsEl {
            access: core::default::Default::default(),
            grantee: core::default::Default::default(),
            grantee_type: core::default::Default::default(),
        }
    }
}
pub struct ElastictranscoderPipelineThumbnailConfigPermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElastictranscoderPipelineThumbnailConfigPermissionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ElastictranscoderPipelineThumbnailConfigPermissionsElRef {
        ElastictranscoderPipelineThumbnailConfigPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElastictranscoderPipelineThumbnailConfigPermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access` after provisioning.\n"]
    pub fn access(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.access", self.base))
    }
    #[doc = "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grantee", self.base))
    }
    #[doc = "Get a reference to the value of field `grantee_type` after provisioning.\n"]
    pub fn grantee_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grantee_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct ElastictranscoderPipelineDynamic {
    content_config: Option<DynamicBlock<ElastictranscoderPipelineContentConfigEl>>,
    content_config_permissions:
        Option<DynamicBlock<ElastictranscoderPipelineContentConfigPermissionsEl>>,
    notifications: Option<DynamicBlock<ElastictranscoderPipelineNotificationsEl>>,
    thumbnail_config: Option<DynamicBlock<ElastictranscoderPipelineThumbnailConfigEl>>,
    thumbnail_config_permissions:
        Option<DynamicBlock<ElastictranscoderPipelineThumbnailConfigPermissionsEl>>,
}
