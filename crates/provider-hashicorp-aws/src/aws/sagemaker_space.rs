use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SagemakerSpaceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    space_display_name: Option<PrimField<String>>,
    space_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ownership_settings: Option<Vec<SagemakerSpaceOwnershipSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    space_settings: Option<Vec<SagemakerSpaceSpaceSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    space_sharing_settings: Option<Vec<SagemakerSpaceSpaceSharingSettingsEl>>,
    dynamic: SagemakerSpaceDynamic,
}
struct SagemakerSpace_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerSpaceData>,
}
#[derive(Clone)]
pub struct SagemakerSpace(Rc<SagemakerSpace_>);
impl SagemakerSpace {
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
    #[doc = "Set the field `space_display_name`.\n"]
    pub fn set_space_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().space_display_name = Some(v.into());
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
    #[doc = "Set the field `ownership_settings`.\n"]
    pub fn set_ownership_settings(
        self,
        v: impl Into<BlockAssignable<SagemakerSpaceOwnershipSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ownership_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ownership_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `space_settings`.\n"]
    pub fn set_space_settings(
        self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().space_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.space_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `space_sharing_settings`.\n"]
    pub fn set_space_sharing_settings(
        self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSharingSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().space_sharing_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.space_sharing_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_efs_file_system_uid", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `space_display_name` after provisioning.\n"]
    pub fn space_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.space_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.space_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ownership_settings` after provisioning.\n"]
    pub fn ownership_settings(&self) -> ListRef<SagemakerSpaceOwnershipSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ownership_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `space_settings` after provisioning.\n"]
    pub fn space_settings(&self) -> ListRef<SagemakerSpaceSpaceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.space_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `space_sharing_settings` after provisioning.\n"]
    pub fn space_sharing_settings(&self) -> ListRef<SagemakerSpaceSpaceSharingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.space_sharing_settings", self.extract_ref()),
        )
    }
}
impl Referable for SagemakerSpace {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SagemakerSpace {}
impl ToListMappable for SagemakerSpace {
    type O = ListRef<SagemakerSpaceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SagemakerSpace_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_space".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSagemakerSpace {
    pub tf_id: String,
    #[doc = ""]
    pub domain_id: PrimField<String>,
    #[doc = ""]
    pub space_name: PrimField<String>,
}
impl BuildSagemakerSpace {
    pub fn build(self, stack: &mut Stack) -> SagemakerSpace {
        let out = SagemakerSpace(Rc::new(SagemakerSpace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerSpaceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                domain_id: self.domain_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                space_display_name: core::default::Default::default(),
                space_name: self.space_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                ownership_settings: core::default::Default::default(),
                space_settings: core::default::Default::default(),
                space_sharing_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SagemakerSpaceRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SagemakerSpaceRef {
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
    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_efs_file_system_uid", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `space_display_name` after provisioning.\n"]
    pub fn space_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.space_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.space_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ownership_settings` after provisioning.\n"]
    pub fn ownership_settings(&self) -> ListRef<SagemakerSpaceOwnershipSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ownership_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `space_settings` after provisioning.\n"]
    pub fn space_settings(&self) -> ListRef<SagemakerSpaceSpaceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.space_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `space_sharing_settings` after provisioning.\n"]
    pub fn space_sharing_settings(&self) -> ListRef<SagemakerSpaceSpaceSharingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.space_sharing_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceOwnershipSettingsEl {
    owner_user_profile_name: PrimField<String>,
}
impl SagemakerSpaceOwnershipSettingsEl {}
impl ToListMappable for SagemakerSpaceOwnershipSettingsEl {
    type O = BlockAssignable<SagemakerSpaceOwnershipSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceOwnershipSettingsEl {
    #[doc = ""]
    pub owner_user_profile_name: PrimField<String>,
}
impl BuildSagemakerSpaceOwnershipSettingsEl {
    pub fn build(self) -> SagemakerSpaceOwnershipSettingsEl {
        SagemakerSpaceOwnershipSettingsEl {
            owner_user_profile_name: self.owner_user_profile_name,
        }
    }
}
pub struct SagemakerSpaceOwnershipSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceOwnershipSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerSpaceOwnershipSettingsElRef {
        SagemakerSpaceOwnershipSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceOwnershipSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `owner_user_profile_name` after provisioning.\n"]
    pub fn owner_user_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_user_profile_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_in_minutes: Option<PrimField<f64>>,
}
impl SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    #[doc = "Set the field `idle_timeout_in_minutes`.\n"]
    pub fn set_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout_in_minutes = Some(v.into());
        self
    }
}
impl ToListMappable
    for SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl
{
    type O = BlockAssignable<
        SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl
{}
impl
    BuildSagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl
{
    pub fn build(
        self,
    ) -> SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl
    {
        SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
            idle_timeout_in_minutes: core::default::Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef { fn new (shared : StackShared , base : String) -> SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef { SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef { shared : shared , base : base . to_string () , } } }
impl SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idle_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_timeout_in_minutes", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElDynamic { idle_settings : Option < DynamicBlock < SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl >> , }
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl { # [serde (skip_serializing_if = "Option::is_none")] idle_settings : Option < Vec < SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl > > , dynamic : SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElDynamic , }
impl SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
    #[doc = "Set the field `idle_settings`.\n"]
    pub fn set_idle_settings(
        mut self,
        v : impl Into < BlockAssignable < SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl
{
    type O = BlockAssignable<
        SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {}
impl BuildSagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
    pub fn build(
        self,
    ) -> SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
        SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
            idle_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
        SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idle_settings` after provisioning.\n"]    pub fn idle_settings (& self) -> ListRef < SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.idle_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}
impl SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
    type O =
        BlockAssignable<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {}
impl BuildSagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
    pub fn build(
        self,
    ) -> SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
        SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
        SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_alias", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDynamic {
    app_lifecycle_management: Option<
        DynamicBlock<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl>,
    >,
    default_resource_spec: Option<
        DynamicBlock<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl>,
    >,
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_lifecycle_management:
        Option<Vec<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec:
        Option<Vec<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDynamic,
}
impl SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsEl {
    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.app_lifecycle_management = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.app_lifecycle_management = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElCodeEditorAppSettingsEl {}
impl BuildSagemakerSpaceSpaceSettingsElCodeEditorAppSettingsEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsEl {
        SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsEl {
            app_lifecycle_management: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElRef {
        SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_lifecycle_management", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_resource_spec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemEl {
    file_system_id: PrimField<String>,
}
impl SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemEl {}
impl ToListMappable for SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemEl {
    #[doc = ""]
    pub file_system_id: PrimField<String>,
}
impl BuildSagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemEl {
        SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemEl {
            file_system_id: self.file_system_id,
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemElRef {
        SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElCustomFileSystemElDynamic {
    efs_file_system:
        Option<DynamicBlock<SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemEl>>,
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElCustomFileSystemEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    efs_file_system: Option<Vec<SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElCustomFileSystemElDynamic,
}
impl SagemakerSpaceSpaceSettingsElCustomFileSystemEl {
    #[doc = "Set the field `efs_file_system`.\n"]
    pub fn set_efs_file_system(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.efs_file_system = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_system = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerSpaceSpaceSettingsElCustomFileSystemEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElCustomFileSystemEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElCustomFileSystemEl {}
impl BuildSagemakerSpaceSpaceSettingsElCustomFileSystemEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElCustomFileSystemEl {
        SagemakerSpaceSpaceSettingsElCustomFileSystemEl {
            efs_file_system: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElCustomFileSystemElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElCustomFileSystemElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElCustomFileSystemElRef {
        SagemakerSpaceSpaceSettingsElCustomFileSystemElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElCustomFileSystemElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `efs_file_system` after provisioning.\n"]
    pub fn efs_file_system(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElCustomFileSystemElEfsFileSystemElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.efs_file_system", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_in_minutes: Option<PrimField<f64>>,
}
impl SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    #[doc = "Set the field `idle_timeout_in_minutes`.\n"]
    pub fn set_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout_in_minutes = Some(v.into());
        self
    }
}
impl ToListMappable
    for SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl
{
    type O = BlockAssignable<
        SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl
{}
impl
    BuildSagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl
{
    pub fn build(
        self,
    ) -> SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl
    {
        SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
            idle_timeout_in_minutes: core::default::Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef { fn new (shared : StackShared , base : String) -> SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef { SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef { shared : shared , base : base . to_string () , } } }
impl SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idle_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_timeout_in_minutes", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElDynamic { idle_settings : Option < DynamicBlock < SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl >> , }
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl { # [serde (skip_serializing_if = "Option::is_none")] idle_settings : Option < Vec < SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl > > , dynamic : SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElDynamic , }
impl SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    #[doc = "Set the field `idle_settings`.\n"]
    pub fn set_idle_settings(
        mut self,
        v : impl Into < BlockAssignable < SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl
{
    type O = BlockAssignable<
        SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {}
impl BuildSagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    pub fn build(
        self,
    ) -> SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
        SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
            idle_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
        SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idle_settings` after provisioning.\n"]    pub fn idle_settings (& self) -> ListRef < SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.idle_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}
impl SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {}
impl ToListMappable for SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: PrimField<String>,
}
impl BuildSagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
        SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
        SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_url", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}
impl SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    type O =
        BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {}
impl BuildSagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    pub fn build(
        self,
    ) -> SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
        SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
        SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_alias", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDynamic {
    app_lifecycle_management: Option<
        DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl>,
    >,
    code_repository:
        Option<DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl>,
    >,
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_lifecycle_management:
        Option<Vec<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository:
        Option<Vec<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec:
        Option<Vec<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDynamic,
}
impl SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsEl {
    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.app_lifecycle_management = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.app_lifecycle_management = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElJupyterLabAppSettingsEl {}
impl BuildSagemakerSpaceSpaceSettingsElJupyterLabAppSettingsEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsEl {
        SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsEl {
            app_lifecycle_management: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElRef {
        SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_lifecycle_management", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_resource_spec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}
impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {}
impl ToListMappable for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    type O =
        BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: PrimField<String>,
}
impl BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_url", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}
impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}
impl ToListMappable
    for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl
{
    type O = BlockAssignable<
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {}
impl BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    pub fn build(
        self,
    ) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_alias", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: Option<
        DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>,
    >,
    default_resource_spec: Option<
        DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository:
        Option<Vec<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec:
        Option<Vec<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDynamic,
}
impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }
    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {}
impl BuildSagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
        SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_resource_spec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}
impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}
impl BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_image_config_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }
    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_version_number", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}
impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }
    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}
impl ToListMappable
    for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl
{
    type O = BlockAssignable<
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {}
impl BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    pub fn build(
        self,
    ) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_alias", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image:
        Option<DynamicBlock<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec:
        Option<Vec<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDynamic,
}
impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {}
impl BuildSagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
        SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }
    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_resource_spec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsEl {
    ebs_volume_size_in_gb: PrimField<f64>,
}
impl SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsEl {}
impl ToListMappable for SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsEl {
    type O =
        BlockAssignable<SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsEl {
    #[doc = ""]
    pub ebs_volume_size_in_gb: PrimField<f64>,
}
impl BuildSagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsEl {
        SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsEl {
            ebs_volume_size_in_gb: self.ebs_volume_size_in_gb,
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsElRef {
        SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn ebs_volume_size_in_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_volume_size_in_gb", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElDynamic {
    ebs_storage_settings: Option<
        DynamicBlock<SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsEl>,
    >,
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsElSpaceStorageSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_storage_settings:
        Option<Vec<SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElDynamic,
}
impl SagemakerSpaceSpaceSettingsElSpaceStorageSettingsEl {
    #[doc = "Set the field `ebs_storage_settings`.\n"]
    pub fn set_ebs_storage_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs_storage_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_storage_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerSpaceSpaceSettingsElSpaceStorageSettingsEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsElSpaceStorageSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsElSpaceStorageSettingsEl {}
impl BuildSagemakerSpaceSpaceSettingsElSpaceStorageSettingsEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsElSpaceStorageSettingsEl {
        SagemakerSpaceSpaceSettingsElSpaceStorageSettingsEl {
            ebs_storage_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElRef {
        SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ebs_storage_settings` after provisioning.\n"]
    pub fn ebs_storage_settings(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElEbsStorageSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ebs_storage_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerSpaceSpaceSettingsElDynamic {
    code_editor_app_settings:
        Option<DynamicBlock<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsEl>>,
    custom_file_system: Option<DynamicBlock<SagemakerSpaceSpaceSettingsElCustomFileSystemEl>>,
    jupyter_lab_app_settings:
        Option<DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsEl>>,
    jupyter_server_app_settings:
        Option<DynamicBlock<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>>,
    kernel_gateway_app_settings:
        Option<DynamicBlock<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>>,
    space_storage_settings:
        Option<DynamicBlock<SagemakerSpaceSpaceSettingsElSpaceStorageSettingsEl>>,
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_editor_app_settings: Option<Vec<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_file_system: Option<Vec<SagemakerSpaceSpaceSettingsElCustomFileSystemEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_lab_app_settings: Option<Vec<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_server_app_settings:
        Option<Vec<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_gateway_app_settings:
        Option<Vec<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    space_storage_settings: Option<Vec<SagemakerSpaceSpaceSettingsElSpaceStorageSettingsEl>>,
    dynamic: SagemakerSpaceSpaceSettingsElDynamic,
}
impl SagemakerSpaceSpaceSettingsEl {
    #[doc = "Set the field `app_type`.\n"]
    pub fn set_app_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.app_type = Some(v.into());
        self
    }
    #[doc = "Set the field `code_editor_app_settings`.\n"]
    pub fn set_code_editor_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_editor_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_editor_app_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom_file_system`.\n"]
    pub fn set_custom_file_system(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElCustomFileSystemEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_file_system = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_file_system = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `jupyter_lab_app_settings`.\n"]
    pub fn set_jupyter_lab_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_lab_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_lab_app_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `space_storage_settings`.\n"]
    pub fn set_space_storage_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerSpaceSpaceSettingsElSpaceStorageSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.space_storage_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.space_storage_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerSpaceSpaceSettingsEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSettingsEl {}
impl BuildSagemakerSpaceSpaceSettingsEl {
    pub fn build(self) -> SagemakerSpaceSpaceSettingsEl {
        SagemakerSpaceSpaceSettingsEl {
            app_type: core::default::Default::default(),
            code_editor_app_settings: core::default::Default::default(),
            custom_file_system: core::default::Default::default(),
            jupyter_lab_app_settings: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            space_storage_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerSpaceSpaceSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerSpaceSpaceSettingsElRef {
        SagemakerSpaceSpaceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_type` after provisioning.\n"]
    pub fn app_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_type", self.base))
    }
    #[doc = "Get a reference to the value of field `code_editor_app_settings` after provisioning.\n"]
    pub fn code_editor_app_settings(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElCodeEditorAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code_editor_app_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_file_system` after provisioning.\n"]
    pub fn custom_file_system(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElCustomFileSystemElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_file_system", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `jupyter_lab_app_settings` after provisioning.\n"]
    pub fn jupyter_lab_app_settings(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElJupyterLabAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jupyter_lab_app_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElJupyterServerAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jupyter_server_app_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElKernelGatewayAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kernel_gateway_app_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `space_storage_settings` after provisioning.\n"]
    pub fn space_storage_settings(
        &self,
    ) -> ListRef<SagemakerSpaceSpaceSettingsElSpaceStorageSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.space_storage_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerSpaceSpaceSharingSettingsEl {
    sharing_type: PrimField<String>,
}
impl SagemakerSpaceSpaceSharingSettingsEl {}
impl ToListMappable for SagemakerSpaceSpaceSharingSettingsEl {
    type O = BlockAssignable<SagemakerSpaceSpaceSharingSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerSpaceSpaceSharingSettingsEl {
    #[doc = ""]
    pub sharing_type: PrimField<String>,
}
impl BuildSagemakerSpaceSpaceSharingSettingsEl {
    pub fn build(self) -> SagemakerSpaceSpaceSharingSettingsEl {
        SagemakerSpaceSpaceSharingSettingsEl {
            sharing_type: self.sharing_type,
        }
    }
}
pub struct SagemakerSpaceSpaceSharingSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerSpaceSpaceSharingSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerSpaceSpaceSharingSettingsElRef {
        SagemakerSpaceSpaceSharingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerSpaceSpaceSharingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sharing_type` after provisioning.\n"]
    pub fn sharing_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sharing_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerSpaceDynamic {
    ownership_settings: Option<DynamicBlock<SagemakerSpaceOwnershipSettingsEl>>,
    space_settings: Option<DynamicBlock<SagemakerSpaceSpaceSettingsEl>>,
    space_sharing_settings: Option<DynamicBlock<SagemakerSpaceSpaceSharingSettingsEl>>,
}
