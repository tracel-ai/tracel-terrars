use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontFieldLevelEncryptionProfileData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_entities: Option<Vec<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesEl>>,
    dynamic: CloudfrontFieldLevelEncryptionProfileDynamic,
}

struct CloudfrontFieldLevelEncryptionProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontFieldLevelEncryptionProfileData>,
}

#[derive(Clone)]
pub struct CloudfrontFieldLevelEncryptionProfile(Rc<CloudfrontFieldLevelEncryptionProfile_>);

impl CloudfrontFieldLevelEncryptionProfile {
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

    #[doc = "Set the field `encryption_entities`.\n"]
    pub fn set_encryption_entities(
        self,
        v: impl Into<BlockAssignable<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_entities = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_entities = Some(d);
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_entities` after provisioning.\n"]
    pub fn encryption_entities(&self) -> ListRef<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_entities", self.extract_ref()))
    }
}

impl Referable for CloudfrontFieldLevelEncryptionProfile {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CloudfrontFieldLevelEncryptionProfile { }

impl ToListMappable for CloudfrontFieldLevelEncryptionProfile {
    type O = ListRef<CloudfrontFieldLevelEncryptionProfileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudfrontFieldLevelEncryptionProfile_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfront_field_level_encryption_profile".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionProfile {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCloudfrontFieldLevelEncryptionProfile {
    pub fn build(self, stack: &mut Stack) -> CloudfrontFieldLevelEncryptionProfile {
        let out = CloudfrontFieldLevelEncryptionProfile(Rc::new(CloudfrontFieldLevelEncryptionProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudfrontFieldLevelEncryptionProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                comment: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                encryption_entities: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontFieldLevelEncryptionProfileRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontFieldLevelEncryptionProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CloudfrontFieldLevelEncryptionProfileRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_entities` after provisioning.\n"]
    pub fn encryption_entities(&self) -> ListRef<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_entities", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}

impl CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable for CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsEl {
    type O = BlockAssignable<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsEl {}

impl BuildCloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsEl {
        CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsElRef {
        CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElDynamic {
    field_patterns: Option<
        DynamicBlock<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsEl {
    provider_id: PrimField<String>,
    public_key_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_patterns: Option<Vec<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsEl>>,
    dynamic: CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElDynamic,
}

impl CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsEl {
    #[doc = "Set the field `field_patterns`.\n"]
    pub fn set_field_patterns(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_patterns = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_patterns = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsEl {
    type O = BlockAssignable<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsEl {
    #[doc = ""]
    pub provider_id: PrimField<String>,
    #[doc = ""]
    pub public_key_id: PrimField<String>,
}

impl BuildCloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsEl {
        CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsEl {
            provider_id: self.provider_id,
            public_key_id: self.public_key_id,
            field_patterns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElRef {
        CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provider_id` after provisioning.\n"]
    pub fn provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `public_key_id` after provisioning.\n"]
    pub fn public_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `field_patterns` after provisioning.\n"]
    pub fn field_patterns(
        &self,
    ) -> ListRef<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsElFieldPatternsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.field_patterns", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElDynamic {
    items: Option<DynamicBlock<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsEl>>,
    dynamic: CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElDynamic,
}

impl CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElItemsEl>>,
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

impl ToListMappable for CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesEl {
    type O = BlockAssignable<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionProfileEncryptionEntitiesEl {}

impl BuildCloudfrontFieldLevelEncryptionProfileEncryptionEntitiesEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesEl {
        CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesEl {
            items: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElRef {
        CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionProfileDynamic {
    encryption_entities: Option<DynamicBlock<CloudfrontFieldLevelEncryptionProfileEncryptionEntitiesEl>>,
}
