use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChimesdkvoiceSipRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    trigger_type: PrimField<String>,
    trigger_value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_applications: Option<Vec<ChimesdkvoiceSipRuleTargetApplicationsEl>>,
    dynamic: ChimesdkvoiceSipRuleDynamic,
}
struct ChimesdkvoiceSipRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChimesdkvoiceSipRuleData>,
}
#[derive(Clone)]
pub struct ChimesdkvoiceSipRule(Rc<ChimesdkvoiceSipRule_>);
impl ChimesdkvoiceSipRule {
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
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
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
    #[doc = "Set the field `target_applications`.\n"]
    pub fn set_target_applications(
        self,
        v: impl Into<BlockAssignable<ChimesdkvoiceSipRuleTargetApplicationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_applications = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_applications = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_type` after provisioning.\n"]
    pub fn trigger_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_value` after provisioning.\n"]
    pub fn trigger_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_value", self.extract_ref()),
        )
    }
}
impl Referable for ChimesdkvoiceSipRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChimesdkvoiceSipRule {}
impl ToListMappable for ChimesdkvoiceSipRule {
    type O = ListRef<ChimesdkvoiceSipRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChimesdkvoiceSipRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_chimesdkvoice_sip_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChimesdkvoiceSipRule {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub trigger_type: PrimField<String>,
    #[doc = ""]
    pub trigger_value: PrimField<String>,
}
impl BuildChimesdkvoiceSipRule {
    pub fn build(self, stack: &mut Stack) -> ChimesdkvoiceSipRule {
        let out = ChimesdkvoiceSipRule(Rc::new(ChimesdkvoiceSipRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChimesdkvoiceSipRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                disabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                trigger_type: self.trigger_type,
                trigger_value: self.trigger_value,
                target_applications: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChimesdkvoiceSipRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChimesdkvoiceSipRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChimesdkvoiceSipRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_type` after provisioning.\n"]
    pub fn trigger_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_value` after provisioning.\n"]
    pub fn trigger_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_value", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChimesdkvoiceSipRuleTargetApplicationsEl {
    aws_region: PrimField<String>,
    priority: PrimField<f64>,
    sip_media_application_id: PrimField<String>,
}
impl ChimesdkvoiceSipRuleTargetApplicationsEl {}
impl ToListMappable for ChimesdkvoiceSipRuleTargetApplicationsEl {
    type O = BlockAssignable<ChimesdkvoiceSipRuleTargetApplicationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChimesdkvoiceSipRuleTargetApplicationsEl {
    #[doc = ""]
    pub aws_region: PrimField<String>,
    #[doc = ""]
    pub priority: PrimField<f64>,
    #[doc = ""]
    pub sip_media_application_id: PrimField<String>,
}
impl BuildChimesdkvoiceSipRuleTargetApplicationsEl {
    pub fn build(self) -> ChimesdkvoiceSipRuleTargetApplicationsEl {
        ChimesdkvoiceSipRuleTargetApplicationsEl {
            aws_region: self.aws_region,
            priority: self.priority,
            sip_media_application_id: self.sip_media_application_id,
        }
    }
}
pub struct ChimesdkvoiceSipRuleTargetApplicationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChimesdkvoiceSipRuleTargetApplicationsElRef {
    fn new(shared: StackShared, base: String) -> ChimesdkvoiceSipRuleTargetApplicationsElRef {
        ChimesdkvoiceSipRuleTargetApplicationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChimesdkvoiceSipRuleTargetApplicationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_region` after provisioning.\n"]
    pub fn aws_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `sip_media_application_id` after provisioning.\n"]
    pub fn sip_media_application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sip_media_application_id", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ChimesdkvoiceSipRuleDynamic {
    target_applications: Option<DynamicBlock<ChimesdkvoiceSipRuleTargetApplicationsEl>>,
}
