use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CleanroomsCollaborationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_engine: Option<PrimField<String>>,
    creator_display_name: PrimField<String>,
    creator_member_abilities: ListField<PrimField<String>>,
    description: PrimField<String>,
    name: PrimField<String>,
    query_log_status: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_encryption_metadata: Option<Vec<CleanroomsCollaborationDataEncryptionMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<Vec<CleanroomsCollaborationMemberEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CleanroomsCollaborationTimeoutsEl>,
    dynamic: CleanroomsCollaborationDynamic,
}

struct CleanroomsCollaboration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CleanroomsCollaborationData>,
}

#[derive(Clone)]
pub struct CleanroomsCollaboration(Rc<CleanroomsCollaboration_>);

impl CleanroomsCollaboration {
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

    #[doc = "Set the field `analytics_engine`.\n"]
    pub fn set_analytics_engine(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().analytics_engine = Some(v.into());
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

    #[doc = "Set the field `data_encryption_metadata`.\n"]
    pub fn set_data_encryption_metadata(
        self,
        v: impl Into<BlockAssignable<CleanroomsCollaborationDataEncryptionMetadataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_encryption_metadata = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_encryption_metadata = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `member`.\n"]
    pub fn set_member(self, v: impl Into<BlockAssignable<CleanroomsCollaborationMemberEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().member = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.member = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CleanroomsCollaborationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `analytics_engine` after provisioning.\n"]
    pub fn analytics_engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.analytics_engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_display_name` after provisioning.\n"]
    pub fn creator_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creator_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_member_abilities` after provisioning.\n"]
    pub fn creator_member_abilities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.creator_member_abilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_log_status` after provisioning.\n"]
    pub fn query_log_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_log_status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_encryption_metadata` after provisioning.\n"]
    pub fn data_encryption_metadata(&self) -> ListRef<CleanroomsCollaborationDataEncryptionMetadataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_encryption_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CleanroomsCollaborationTimeoutsElRef {
        CleanroomsCollaborationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for CleanroomsCollaboration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CleanroomsCollaboration { }

impl ToListMappable for CleanroomsCollaboration {
    type O = ListRef<CleanroomsCollaborationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CleanroomsCollaboration_ {
    fn extract_resource_type(&self) -> String {
        "aws_cleanrooms_collaboration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCleanroomsCollaboration {
    pub tf_id: String,
    #[doc = ""]
    pub creator_display_name: PrimField<String>,
    #[doc = ""]
    pub creator_member_abilities: ListField<PrimField<String>>,
    #[doc = ""]
    pub description: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub query_log_status: PrimField<String>,
}

impl BuildCleanroomsCollaboration {
    pub fn build(self, stack: &mut Stack) -> CleanroomsCollaboration {
        let out = CleanroomsCollaboration(Rc::new(CleanroomsCollaboration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CleanroomsCollaborationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                analytics_engine: core::default::Default::default(),
                creator_display_name: self.creator_display_name,
                creator_member_abilities: self.creator_member_abilities,
                description: self.description,
                name: self.name,
                query_log_status: self.query_log_status,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                data_encryption_metadata: core::default::Default::default(),
                member: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CleanroomsCollaborationRef {
    shared: StackShared,
    base: String,
}

impl Ref for CleanroomsCollaborationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CleanroomsCollaborationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analytics_engine` after provisioning.\n"]
    pub fn analytics_engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.analytics_engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_display_name` after provisioning.\n"]
    pub fn creator_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creator_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_member_abilities` after provisioning.\n"]
    pub fn creator_member_abilities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.creator_member_abilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_log_status` after provisioning.\n"]
    pub fn query_log_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_log_status", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_encryption_metadata` after provisioning.\n"]
    pub fn data_encryption_metadata(&self) -> ListRef<CleanroomsCollaborationDataEncryptionMetadataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_encryption_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CleanroomsCollaborationTimeoutsElRef {
        CleanroomsCollaborationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CleanroomsCollaborationDataEncryptionMetadataEl {
    allow_clear_text: PrimField<bool>,
    allow_duplicates: PrimField<bool>,
    allow_joins_on_columns_with_different_names: PrimField<bool>,
    preserve_nulls: PrimField<bool>,
}

impl CleanroomsCollaborationDataEncryptionMetadataEl { }

impl ToListMappable for CleanroomsCollaborationDataEncryptionMetadataEl {
    type O = BlockAssignable<CleanroomsCollaborationDataEncryptionMetadataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsCollaborationDataEncryptionMetadataEl {
    #[doc = ""]
    pub allow_clear_text: PrimField<bool>,
    #[doc = ""]
    pub allow_duplicates: PrimField<bool>,
    #[doc = ""]
    pub allow_joins_on_columns_with_different_names: PrimField<bool>,
    #[doc = ""]
    pub preserve_nulls: PrimField<bool>,
}

impl BuildCleanroomsCollaborationDataEncryptionMetadataEl {
    pub fn build(self) -> CleanroomsCollaborationDataEncryptionMetadataEl {
        CleanroomsCollaborationDataEncryptionMetadataEl {
            allow_clear_text: self.allow_clear_text,
            allow_duplicates: self.allow_duplicates,
            allow_joins_on_columns_with_different_names: self.allow_joins_on_columns_with_different_names,
            preserve_nulls: self.preserve_nulls,
        }
    }
}

pub struct CleanroomsCollaborationDataEncryptionMetadataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CleanroomsCollaborationDataEncryptionMetadataElRef {
    fn new(shared: StackShared, base: String) -> CleanroomsCollaborationDataEncryptionMetadataElRef {
        CleanroomsCollaborationDataEncryptionMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsCollaborationDataEncryptionMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_clear_text` after provisioning.\n"]
    pub fn allow_clear_text(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_clear_text", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_duplicates` after provisioning.\n"]
    pub fn allow_duplicates(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_duplicates", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_joins_on_columns_with_different_names` after provisioning.\n"]
    pub fn allow_joins_on_columns_with_different_names(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_joins_on_columns_with_different_names", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_nulls` after provisioning.\n"]
    pub fn preserve_nulls(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_nulls", self.base))
    }
}

#[derive(Serialize)]
pub struct CleanroomsCollaborationMemberEl {
    account_id: PrimField<String>,
    display_name: PrimField<String>,
    member_abilities: ListField<PrimField<String>>,
}

impl CleanroomsCollaborationMemberEl { }

impl ToListMappable for CleanroomsCollaborationMemberEl {
    type O = BlockAssignable<CleanroomsCollaborationMemberEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsCollaborationMemberEl {
    #[doc = ""]
    pub account_id: PrimField<String>,
    #[doc = ""]
    pub display_name: PrimField<String>,
    #[doc = ""]
    pub member_abilities: ListField<PrimField<String>>,
}

impl BuildCleanroomsCollaborationMemberEl {
    pub fn build(self) -> CleanroomsCollaborationMemberEl {
        CleanroomsCollaborationMemberEl {
            account_id: self.account_id,
            display_name: self.display_name,
            member_abilities: self.member_abilities,
        }
    }
}

pub struct CleanroomsCollaborationMemberElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CleanroomsCollaborationMemberElRef {
    fn new(shared: StackShared, base: String) -> CleanroomsCollaborationMemberElRef {
        CleanroomsCollaborationMemberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsCollaborationMemberElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `member_abilities` after provisioning.\n"]
    pub fn member_abilities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.member_abilities", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct CleanroomsCollaborationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl CleanroomsCollaborationTimeoutsEl {
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

impl ToListMappable for CleanroomsCollaborationTimeoutsEl {
    type O = BlockAssignable<CleanroomsCollaborationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsCollaborationTimeoutsEl {}

impl BuildCleanroomsCollaborationTimeoutsEl {
    pub fn build(self) -> CleanroomsCollaborationTimeoutsEl {
        CleanroomsCollaborationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CleanroomsCollaborationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CleanroomsCollaborationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CleanroomsCollaborationTimeoutsElRef {
        CleanroomsCollaborationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsCollaborationTimeoutsElRef {
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
struct CleanroomsCollaborationDynamic {
    data_encryption_metadata: Option<DynamicBlock<CleanroomsCollaborationDataEncryptionMetadataEl>>,
    member: Option<DynamicBlock<CleanroomsCollaborationMemberEl>>,
}
