use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SagemakerHubData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    hub_description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hub_display_name: Option<PrimField<String>>,
    hub_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hub_search_keywords: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_storage_config: Option<Vec<SagemakerHubS3StorageConfigEl>>,
    dynamic: SagemakerHubDynamic,
}

struct SagemakerHub_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerHubData>,
}

#[derive(Clone)]
pub struct SagemakerHub(Rc<SagemakerHub_>);

impl SagemakerHub {
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

    #[doc = "Set the field `hub_display_name`.\n"]
    pub fn set_hub_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().hub_display_name = Some(v.into());
        self
    }

    #[doc = "Set the field `hub_search_keywords`.\n"]
    pub fn set_hub_search_keywords(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().hub_search_keywords = Some(v.into());
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

    #[doc = "Set the field `s3_storage_config`.\n"]
    pub fn set_s3_storage_config(
        self,
        v: impl Into<BlockAssignable<SagemakerHubS3StorageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_storage_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_storage_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hub_description` after provisioning.\n"]
    pub fn hub_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hub_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hub_display_name` after provisioning.\n"]
    pub fn hub_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hub_display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hub_name` after provisioning.\n"]
    pub fn hub_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hub_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hub_search_keywords` after provisioning.\n"]
    pub fn hub_search_keywords(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.hub_search_keywords", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `s3_storage_config` after provisioning.\n"]
    pub fn s3_storage_config(&self) -> ListRef<SagemakerHubS3StorageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_storage_config", self.extract_ref()),
        )
    }
}

impl Referable for SagemakerHub {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SagemakerHub {}

impl ToListMappable for SagemakerHub {
    type O = ListRef<SagemakerHubRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerHub_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_hub".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerHub {
    pub tf_id: String,
    #[doc = ""]
    pub hub_description: PrimField<String>,
    #[doc = ""]
    pub hub_name: PrimField<String>,
}

impl BuildSagemakerHub {
    pub fn build(self, stack: &mut Stack) -> SagemakerHub {
        let out = SagemakerHub(Rc::new(SagemakerHub_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerHubData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                hub_description: self.hub_description,
                hub_display_name: core::default::Default::default(),
                hub_name: self.hub_name,
                hub_search_keywords: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                s3_storage_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerHubRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerHubRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SagemakerHubRef {
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

    #[doc = "Get a reference to the value of field `hub_description` after provisioning.\n"]
    pub fn hub_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hub_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hub_display_name` after provisioning.\n"]
    pub fn hub_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hub_display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hub_name` after provisioning.\n"]
    pub fn hub_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hub_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hub_search_keywords` after provisioning.\n"]
    pub fn hub_search_keywords(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.hub_search_keywords", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `s3_storage_config` after provisioning.\n"]
    pub fn s3_storage_config(&self) -> ListRef<SagemakerHubS3StorageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_storage_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerHubS3StorageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_output_path: Option<PrimField<String>>,
}

impl SagemakerHubS3StorageConfigEl {
    #[doc = "Set the field `s3_output_path`.\n"]
    pub fn set_s3_output_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_output_path = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerHubS3StorageConfigEl {
    type O = BlockAssignable<SagemakerHubS3StorageConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerHubS3StorageConfigEl {}

impl BuildSagemakerHubS3StorageConfigEl {
    pub fn build(self) -> SagemakerHubS3StorageConfigEl {
        SagemakerHubS3StorageConfigEl {
            s3_output_path: core::default::Default::default(),
        }
    }
}

pub struct SagemakerHubS3StorageConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerHubS3StorageConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerHubS3StorageConfigElRef {
        SagemakerHubS3StorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerHubS3StorageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_output_path", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerHubDynamic {
    s3_storage_config: Option<DynamicBlock<SagemakerHubS3StorageConfigEl>>,
}
