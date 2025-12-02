use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SsmcontactsPlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    contact_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<Vec<SsmcontactsPlanStageEl>>,
    dynamic: SsmcontactsPlanDynamic,
}
struct SsmcontactsPlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsmcontactsPlanData>,
}
#[derive(Clone)]
pub struct SsmcontactsPlan(Rc<SsmcontactsPlan_>);
impl SsmcontactsPlan {
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
    #[doc = "Set the field `stage`.\n"]
    pub fn set_stage(self, v: impl Into<BlockAssignable<SsmcontactsPlanStageEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stage = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> ListRef<SsmcontactsPlanStageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stage", self.extract_ref()),
        )
    }
}
impl Referable for SsmcontactsPlan {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SsmcontactsPlan {}
impl ToListMappable for SsmcontactsPlan {
    type O = ListRef<SsmcontactsPlanRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SsmcontactsPlan_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssmcontacts_plan".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSsmcontactsPlan {
    pub tf_id: String,
    #[doc = ""]
    pub contact_id: PrimField<String>,
}
impl BuildSsmcontactsPlan {
    pub fn build(self, stack: &mut Stack) -> SsmcontactsPlan {
        let out = SsmcontactsPlan(Rc::new(SsmcontactsPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SsmcontactsPlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                contact_id: self.contact_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                stage: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SsmcontactsPlanRef {
    shared: StackShared,
    base: String,
}
impl Ref for SsmcontactsPlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SsmcontactsPlanRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> ListRef<SsmcontactsPlanStageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stage", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SsmcontactsPlanStageElTargetElChannelTargetInfoEl {
    contact_channel_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_interval_in_minutes: Option<PrimField<f64>>,
}
impl SsmcontactsPlanStageElTargetElChannelTargetInfoEl {
    #[doc = "Set the field `retry_interval_in_minutes`.\n"]
    pub fn set_retry_interval_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry_interval_in_minutes = Some(v.into());
        self
    }
}
impl ToListMappable for SsmcontactsPlanStageElTargetElChannelTargetInfoEl {
    type O = BlockAssignable<SsmcontactsPlanStageElTargetElChannelTargetInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSsmcontactsPlanStageElTargetElChannelTargetInfoEl {
    #[doc = ""]
    pub contact_channel_id: PrimField<String>,
}
impl BuildSsmcontactsPlanStageElTargetElChannelTargetInfoEl {
    pub fn build(self) -> SsmcontactsPlanStageElTargetElChannelTargetInfoEl {
        SsmcontactsPlanStageElTargetElChannelTargetInfoEl {
            contact_channel_id: self.contact_channel_id,
            retry_interval_in_minutes: core::default::Default::default(),
        }
    }
}
pub struct SsmcontactsPlanStageElTargetElChannelTargetInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SsmcontactsPlanStageElTargetElChannelTargetInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmcontactsPlanStageElTargetElChannelTargetInfoElRef {
        SsmcontactsPlanStageElTargetElChannelTargetInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SsmcontactsPlanStageElTargetElChannelTargetInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contact_channel_id` after provisioning.\n"]
    pub fn contact_channel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_channel_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retry_interval_in_minutes` after provisioning.\n"]
    pub fn retry_interval_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retry_interval_in_minutes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SsmcontactsPlanStageElTargetElContactTargetInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_id: Option<PrimField<String>>,
    is_essential: PrimField<bool>,
}
impl SsmcontactsPlanStageElTargetElContactTargetInfoEl {
    #[doc = "Set the field `contact_id`.\n"]
    pub fn set_contact_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contact_id = Some(v.into());
        self
    }
}
impl ToListMappable for SsmcontactsPlanStageElTargetElContactTargetInfoEl {
    type O = BlockAssignable<SsmcontactsPlanStageElTargetElContactTargetInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSsmcontactsPlanStageElTargetElContactTargetInfoEl {
    #[doc = ""]
    pub is_essential: PrimField<bool>,
}
impl BuildSsmcontactsPlanStageElTargetElContactTargetInfoEl {
    pub fn build(self) -> SsmcontactsPlanStageElTargetElContactTargetInfoEl {
        SsmcontactsPlanStageElTargetElContactTargetInfoEl {
            contact_id: core::default::Default::default(),
            is_essential: self.is_essential,
        }
    }
}
pub struct SsmcontactsPlanStageElTargetElContactTargetInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SsmcontactsPlanStageElTargetElContactTargetInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmcontactsPlanStageElTargetElContactTargetInfoElRef {
        SsmcontactsPlanStageElTargetElContactTargetInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SsmcontactsPlanStageElTargetElContactTargetInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.base))
    }
    #[doc = "Get a reference to the value of field `is_essential` after provisioning.\n"]
    pub fn is_essential(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_essential", self.base))
    }
}
#[derive(Serialize, Default)]
struct SsmcontactsPlanStageElTargetElDynamic {
    channel_target_info: Option<DynamicBlock<SsmcontactsPlanStageElTargetElChannelTargetInfoEl>>,
    contact_target_info: Option<DynamicBlock<SsmcontactsPlanStageElTargetElContactTargetInfoEl>>,
}
#[derive(Serialize)]
pub struct SsmcontactsPlanStageElTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_target_info: Option<Vec<SsmcontactsPlanStageElTargetElChannelTargetInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_target_info: Option<Vec<SsmcontactsPlanStageElTargetElContactTargetInfoEl>>,
    dynamic: SsmcontactsPlanStageElTargetElDynamic,
}
impl SsmcontactsPlanStageElTargetEl {
    #[doc = "Set the field `channel_target_info`.\n"]
    pub fn set_channel_target_info(
        mut self,
        v: impl Into<BlockAssignable<SsmcontactsPlanStageElTargetElChannelTargetInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.channel_target_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.channel_target_info = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `contact_target_info`.\n"]
    pub fn set_contact_target_info(
        mut self,
        v: impl Into<BlockAssignable<SsmcontactsPlanStageElTargetElContactTargetInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.contact_target_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.contact_target_info = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SsmcontactsPlanStageElTargetEl {
    type O = BlockAssignable<SsmcontactsPlanStageElTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSsmcontactsPlanStageElTargetEl {}
impl BuildSsmcontactsPlanStageElTargetEl {
    pub fn build(self) -> SsmcontactsPlanStageElTargetEl {
        SsmcontactsPlanStageElTargetEl {
            channel_target_info: core::default::Default::default(),
            contact_target_info: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SsmcontactsPlanStageElTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SsmcontactsPlanStageElTargetElRef {
    fn new(shared: StackShared, base: String) -> SsmcontactsPlanStageElTargetElRef {
        SsmcontactsPlanStageElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SsmcontactsPlanStageElTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel_target_info` after provisioning.\n"]
    pub fn channel_target_info(
        &self,
    ) -> ListRef<SsmcontactsPlanStageElTargetElChannelTargetInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.channel_target_info", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `contact_target_info` after provisioning.\n"]
    pub fn contact_target_info(
        &self,
    ) -> ListRef<SsmcontactsPlanStageElTargetElContactTargetInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contact_target_info", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SsmcontactsPlanStageElDynamic {
    target: Option<DynamicBlock<SsmcontactsPlanStageElTargetEl>>,
}
#[derive(Serialize)]
pub struct SsmcontactsPlanStageEl {
    duration_in_minutes: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<SsmcontactsPlanStageElTargetEl>>,
    dynamic: SsmcontactsPlanStageElDynamic,
}
impl SsmcontactsPlanStageEl {
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl Into<BlockAssignable<SsmcontactsPlanStageElTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SsmcontactsPlanStageEl {
    type O = BlockAssignable<SsmcontactsPlanStageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSsmcontactsPlanStageEl {
    #[doc = ""]
    pub duration_in_minutes: PrimField<f64>,
}
impl BuildSsmcontactsPlanStageEl {
    pub fn build(self) -> SsmcontactsPlanStageEl {
        SsmcontactsPlanStageEl {
            duration_in_minutes: self.duration_in_minutes,
            target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SsmcontactsPlanStageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SsmcontactsPlanStageElRef {
    fn new(shared: StackShared, base: String) -> SsmcontactsPlanStageElRef {
        SsmcontactsPlanStageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SsmcontactsPlanStageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `duration_in_minutes` after provisioning.\n"]
    pub fn duration_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.duration_in_minutes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<SsmcontactsPlanStageElTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize, Default)]
struct SsmcontactsPlanDynamic {
    stage: Option<DynamicBlock<SsmcontactsPlanStageEl>>,
}
