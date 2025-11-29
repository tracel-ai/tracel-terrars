use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontFieldLevelEncryptionConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type_profile_config: Option<Vec<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_arg_profile_config: Option<Vec<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl>>,
    dynamic: CloudfrontFieldLevelEncryptionConfigDynamic,
}

struct CloudfrontFieldLevelEncryptionConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontFieldLevelEncryptionConfigData>,
}

#[derive(Clone)]
pub struct CloudfrontFieldLevelEncryptionConfig(Rc<CloudfrontFieldLevelEncryptionConfig_>);

impl CloudfrontFieldLevelEncryptionConfig {
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

    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().comment = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `content_type_profile_config`.\n"]
    pub fn set_content_type_profile_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_type_profile_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_type_profile_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_arg_profile_config`.\n"]
    pub fn set_query_arg_profile_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().query_arg_profile_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.query_arg_profile_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `caller_reference` after provisioning.\n"]
    pub fn caller_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.caller_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_type_profile_config` after provisioning.\n"]
    pub fn content_type_profile_config(
        &self,
    ) -> ListRef<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.content_type_profile_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_arg_profile_config` after provisioning.\n"]
    pub fn query_arg_profile_config(&self) -> ListRef<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query_arg_profile_config", self.extract_ref()))
    }
}

impl Referable for CloudfrontFieldLevelEncryptionConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CloudfrontFieldLevelEncryptionConfig { }

impl ToListMappable for CloudfrontFieldLevelEncryptionConfig {
    type O = ListRef<CloudfrontFieldLevelEncryptionConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudfrontFieldLevelEncryptionConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfront_field_level_encryption_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfig {
    pub tf_id: String,
}

impl BuildCloudfrontFieldLevelEncryptionConfig {
    pub fn build(self, stack: &mut Stack) -> CloudfrontFieldLevelEncryptionConfig {
        let out = CloudfrontFieldLevelEncryptionConfig(Rc::new(CloudfrontFieldLevelEncryptionConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudfrontFieldLevelEncryptionConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                comment: core::default::Default::default(),
                id: core::default::Default::default(),
                content_type_profile_config: core::default::Default::default(),
                query_arg_profile_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontFieldLevelEncryptionConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigRef {
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

    #[doc = "Get a reference to the value of field `caller_reference` after provisioning.\n"]
    pub fn caller_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.caller_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_type_profile_config` after provisioning.\n"]
    pub fn content_type_profile_config(
        &self,
    ) -> ListRef<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.content_type_profile_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_arg_profile_config` after provisioning.\n"]
    pub fn query_arg_profile_config(&self) -> ListRef<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query_arg_profile_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
    content_type: PrimField<String>,
    format: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_id: Option<PrimField<String>>,
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
    #[doc = "Set the field `profile_id`.\n"]
    pub fn set_profile_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.profile_id = Some(v.into());
        self
    }
}

impl ToListMappable for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
    type O =
        BlockAssignable<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
    #[doc = ""]
    pub content_type: PrimField<String>,
    #[doc = ""]
    pub format: PrimField<String>,
}

impl BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
            content_type: self.content_type,
            format: self.format,
            profile_id: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsElRef {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\n"]
    pub fn profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElDynamic {
    items: Option<
        DynamicBlock<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl>>,
    dynamic: CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElDynamic,
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.items = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.items = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
    type O = BlockAssignable<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {}

impl BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
            items: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElDynamic {
    content_type_profiles: Option<
        DynamicBlock<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
    forward_when_content_type_is_unknown: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type_profiles: Option<
        Vec<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl>,
    >,
    dynamic: CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElDynamic,
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
    #[doc = "Set the field `content_type_profiles`.\n"]
    pub fn set_content_type_profiles(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.content_type_profiles = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.content_type_profiles = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
    type O = BlockAssignable<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
    #[doc = ""]
    pub forward_when_content_type_is_unknown: PrimField<bool>,
}

impl BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
            forward_when_content_type_is_unknown: self.forward_when_content_type_is_unknown,
            content_type_profiles: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `forward_when_content_type_is_unknown` after provisioning.\n"]
    pub fn forward_when_content_type_is_unknown(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.forward_when_content_type_is_unknown", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type_profiles` after provisioning.\n"]
    pub fn content_type_profiles(
        &self,
    ) -> ListRef<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.content_type_profiles", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
    profile_id: PrimField<String>,
    query_arg: PrimField<String>,
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl { }

impl ToListMappable for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
    type O = BlockAssignable<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
    #[doc = ""]
    pub profile_id: PrimField<String>,
    #[doc = ""]
    pub query_arg: PrimField<String>,
}

impl BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
            profile_id: self.profile_id,
            query_arg: self.query_arg,
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsElRef {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\n"]
    pub fn profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.base))
    }

    #[doc = "Get a reference to the value of field `query_arg` after provisioning.\n"]
    pub fn query_arg(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_arg", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElDynamic {
    items: Option<
        DynamicBlock<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl>>,
    dynamic: CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElDynamic,
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.items = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.items = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
    type O = BlockAssignable<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {}

impl BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
            items: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElDynamic {
    query_arg_profiles: Option<
        DynamicBlock<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
    forward_when_query_arg_profile_is_unknown: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_arg_profiles: Option<Vec<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl>>,
    dynamic: CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElDynamic,
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
    #[doc = "Set the field `query_arg_profiles`.\n"]
    pub fn set_query_arg_profiles(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_arg_profiles = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_arg_profiles = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
    type O = BlockAssignable<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
    #[doc = ""]
    pub forward_when_query_arg_profile_is_unknown: PrimField<bool>,
}

impl BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
            forward_when_query_arg_profile_is_unknown: self.forward_when_query_arg_profile_is_unknown,
            query_arg_profiles: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `forward_when_query_arg_profile_is_unknown` after provisioning.\n"]
    pub fn forward_when_query_arg_profile_is_unknown(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.forward_when_query_arg_profile_is_unknown", self.base))
    }

    #[doc = "Get a reference to the value of field `query_arg_profiles` after provisioning.\n"]
    pub fn query_arg_profiles(
        &self,
    ) -> ListRef<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query_arg_profiles", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionConfigDynamic {
    content_type_profile_config: Option<DynamicBlock<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl>>,
    query_arg_profile_config: Option<DynamicBlock<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl>>,
}
