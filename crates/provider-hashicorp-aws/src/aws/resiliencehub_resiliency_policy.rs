use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ResiliencehubResiliencyPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_location_constraint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    tier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<Vec<ResiliencehubResiliencyPolicyPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ResiliencehubResiliencyPolicyTimeoutsEl>,
    dynamic: ResiliencehubResiliencyPolicyDynamic,
}

struct ResiliencehubResiliencyPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ResiliencehubResiliencyPolicyData>,
}

#[derive(Clone)]
pub struct ResiliencehubResiliencyPolicy(Rc<ResiliencehubResiliencyPolicy_>);

impl ResiliencehubResiliencyPolicy {
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

    #[doc = "Set the field `data_location_constraint`.\nSpecifies a high-level geographical location constraint for where resilience policy data can be stored."]
    pub fn set_data_location_constraint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_location_constraint = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description for the policy."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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

    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(
        self,
        v: impl Into<BlockAssignable<ResiliencehubResiliencyPolicyPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ResiliencehubResiliencyPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_location_constraint` after provisioning.\nSpecifies a high-level geographical location constraint for where resilience policy data can be stored."]
    pub fn data_location_constraint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_location_constraint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description for the policy."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `estimated_cost_tier` after provisioning.\nSpecifies the estimated cost tier of the resiliency policy."]
    pub fn estimated_cost_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.estimated_cost_tier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy."]
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

    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe tier for the resiliency policy, ranging from the highest severity (MissionCritical) to lowest (NonCritical)."]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> ListRef<ResiliencehubResiliencyPolicyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ResiliencehubResiliencyPolicyTimeoutsElRef {
        ResiliencehubResiliencyPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ResiliencehubResiliencyPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ResiliencehubResiliencyPolicy {}

impl ToListMappable for ResiliencehubResiliencyPolicy {
    type O = ListRef<ResiliencehubResiliencyPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ResiliencehubResiliencyPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_resiliencehub_resiliency_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildResiliencehubResiliencyPolicy {
    pub tf_id: String,
    #[doc = "The name of the policy."]
    pub name: PrimField<String>,
    #[doc = "The tier for the resiliency policy, ranging from the highest severity (MissionCritical) to lowest (NonCritical)."]
    pub tier: PrimField<String>,
}

impl BuildResiliencehubResiliencyPolicy {
    pub fn build(self, stack: &mut Stack) -> ResiliencehubResiliencyPolicy {
        let out = ResiliencehubResiliencyPolicy(Rc::new(ResiliencehubResiliencyPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ResiliencehubResiliencyPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_location_constraint: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tier: self.tier,
                policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ResiliencehubResiliencyPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for ResiliencehubResiliencyPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ResiliencehubResiliencyPolicyRef {
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

    #[doc = "Get a reference to the value of field `data_location_constraint` after provisioning.\nSpecifies a high-level geographical location constraint for where resilience policy data can be stored."]
    pub fn data_location_constraint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_location_constraint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description for the policy."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `estimated_cost_tier` after provisioning.\nSpecifies the estimated cost tier of the resiliency policy."]
    pub fn estimated_cost_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.estimated_cost_tier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy."]
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

    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe tier for the resiliency policy, ranging from the highest severity (MissionCritical) to lowest (NonCritical)."]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> ListRef<ResiliencehubResiliencyPolicyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ResiliencehubResiliencyPolicyTimeoutsElRef {
        ResiliencehubResiliencyPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicyPolicyElAzEl {
    rpo: PrimField<String>,
    rto: PrimField<String>,
}

impl ResiliencehubResiliencyPolicyPolicyElAzEl {}

impl ToListMappable for ResiliencehubResiliencyPolicyPolicyElAzEl {
    type O = BlockAssignable<ResiliencehubResiliencyPolicyPolicyElAzEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicyPolicyElAzEl {
    #[doc = "Recovery Point Objective (RPO) as a Go duration."]
    pub rpo: PrimField<String>,
    #[doc = "Recovery Time Objective (RTO) as a Go duration."]
    pub rto: PrimField<String>,
}

impl BuildResiliencehubResiliencyPolicyPolicyElAzEl {
    pub fn build(self) -> ResiliencehubResiliencyPolicyPolicyElAzEl {
        ResiliencehubResiliencyPolicyPolicyElAzEl {
            rpo: self.rpo,
            rto: self.rto,
        }
    }
}

pub struct ResiliencehubResiliencyPolicyPolicyElAzElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ResiliencehubResiliencyPolicyPolicyElAzElRef {
    fn new(shared: StackShared, base: String) -> ResiliencehubResiliencyPolicyPolicyElAzElRef {
        ResiliencehubResiliencyPolicyPolicyElAzElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicyPolicyElAzElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rpo` after provisioning.\nRecovery Point Objective (RPO) as a Go duration."]
    pub fn rpo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.base))
    }

    #[doc = "Get a reference to the value of field `rto` after provisioning.\nRecovery Time Objective (RTO) as a Go duration."]
    pub fn rto(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rto", self.base))
    }
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicyPolicyElHardwareEl {
    rpo: PrimField<String>,
    rto: PrimField<String>,
}

impl ResiliencehubResiliencyPolicyPolicyElHardwareEl {}

impl ToListMappable for ResiliencehubResiliencyPolicyPolicyElHardwareEl {
    type O = BlockAssignable<ResiliencehubResiliencyPolicyPolicyElHardwareEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicyPolicyElHardwareEl {
    #[doc = "Recovery Point Objective (RPO) as a Go duration."]
    pub rpo: PrimField<String>,
    #[doc = "Recovery Time Objective (RTO) as a Go duration."]
    pub rto: PrimField<String>,
}

impl BuildResiliencehubResiliencyPolicyPolicyElHardwareEl {
    pub fn build(self) -> ResiliencehubResiliencyPolicyPolicyElHardwareEl {
        ResiliencehubResiliencyPolicyPolicyElHardwareEl {
            rpo: self.rpo,
            rto: self.rto,
        }
    }
}

pub struct ResiliencehubResiliencyPolicyPolicyElHardwareElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ResiliencehubResiliencyPolicyPolicyElHardwareElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ResiliencehubResiliencyPolicyPolicyElHardwareElRef {
        ResiliencehubResiliencyPolicyPolicyElHardwareElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicyPolicyElHardwareElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rpo` after provisioning.\nRecovery Point Objective (RPO) as a Go duration."]
    pub fn rpo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.base))
    }

    #[doc = "Get a reference to the value of field `rto` after provisioning.\nRecovery Time Objective (RTO) as a Go duration."]
    pub fn rto(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rto", self.base))
    }
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicyPolicyElRegionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rpo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rto: Option<PrimField<String>>,
}

impl ResiliencehubResiliencyPolicyPolicyElRegionEl {
    #[doc = "Set the field `rpo`.\nRecovery Point Objective (RPO) as a Go duration."]
    pub fn set_rpo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rpo = Some(v.into());
        self
    }

    #[doc = "Set the field `rto`.\nRecovery Time Objective (RTO) as a Go duration."]
    pub fn set_rto(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rto = Some(v.into());
        self
    }
}

impl ToListMappable for ResiliencehubResiliencyPolicyPolicyElRegionEl {
    type O = BlockAssignable<ResiliencehubResiliencyPolicyPolicyElRegionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicyPolicyElRegionEl {}

impl BuildResiliencehubResiliencyPolicyPolicyElRegionEl {
    pub fn build(self) -> ResiliencehubResiliencyPolicyPolicyElRegionEl {
        ResiliencehubResiliencyPolicyPolicyElRegionEl {
            rpo: core::default::Default::default(),
            rto: core::default::Default::default(),
        }
    }
}

pub struct ResiliencehubResiliencyPolicyPolicyElRegionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ResiliencehubResiliencyPolicyPolicyElRegionElRef {
    fn new(shared: StackShared, base: String) -> ResiliencehubResiliencyPolicyPolicyElRegionElRef {
        ResiliencehubResiliencyPolicyPolicyElRegionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicyPolicyElRegionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rpo` after provisioning.\nRecovery Point Objective (RPO) as a Go duration."]
    pub fn rpo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.base))
    }

    #[doc = "Get a reference to the value of field `rto` after provisioning.\nRecovery Time Objective (RTO) as a Go duration."]
    pub fn rto(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rto", self.base))
    }
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicyPolicyElSoftwareEl {
    rpo: PrimField<String>,
    rto: PrimField<String>,
}

impl ResiliencehubResiliencyPolicyPolicyElSoftwareEl {}

impl ToListMappable for ResiliencehubResiliencyPolicyPolicyElSoftwareEl {
    type O = BlockAssignable<ResiliencehubResiliencyPolicyPolicyElSoftwareEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicyPolicyElSoftwareEl {
    #[doc = "Recovery Point Objective (RPO) as a Go duration."]
    pub rpo: PrimField<String>,
    #[doc = "Recovery Time Objective (RTO) as a Go duration."]
    pub rto: PrimField<String>,
}

impl BuildResiliencehubResiliencyPolicyPolicyElSoftwareEl {
    pub fn build(self) -> ResiliencehubResiliencyPolicyPolicyElSoftwareEl {
        ResiliencehubResiliencyPolicyPolicyElSoftwareEl {
            rpo: self.rpo,
            rto: self.rto,
        }
    }
}

pub struct ResiliencehubResiliencyPolicyPolicyElSoftwareElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ResiliencehubResiliencyPolicyPolicyElSoftwareElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ResiliencehubResiliencyPolicyPolicyElSoftwareElRef {
        ResiliencehubResiliencyPolicyPolicyElSoftwareElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicyPolicyElSoftwareElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rpo` after provisioning.\nRecovery Point Objective (RPO) as a Go duration."]
    pub fn rpo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.base))
    }

    #[doc = "Get a reference to the value of field `rto` after provisioning.\nRecovery Time Objective (RTO) as a Go duration."]
    pub fn rto(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rto", self.base))
    }
}

#[derive(Serialize, Default)]
struct ResiliencehubResiliencyPolicyPolicyElDynamic {
    az: Option<DynamicBlock<ResiliencehubResiliencyPolicyPolicyElAzEl>>,
    hardware: Option<DynamicBlock<ResiliencehubResiliencyPolicyPolicyElHardwareEl>>,
    region: Option<DynamicBlock<ResiliencehubResiliencyPolicyPolicyElRegionEl>>,
    software: Option<DynamicBlock<ResiliencehubResiliencyPolicyPolicyElSoftwareEl>>,
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicyPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    az: Option<Vec<ResiliencehubResiliencyPolicyPolicyElAzEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hardware: Option<Vec<ResiliencehubResiliencyPolicyPolicyElHardwareEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<Vec<ResiliencehubResiliencyPolicyPolicyElRegionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    software: Option<Vec<ResiliencehubResiliencyPolicyPolicyElSoftwareEl>>,
    dynamic: ResiliencehubResiliencyPolicyPolicyElDynamic,
}

impl ResiliencehubResiliencyPolicyPolicyEl {
    #[doc = "Set the field `az`.\n"]
    pub fn set_az(
        mut self,
        v: impl Into<BlockAssignable<ResiliencehubResiliencyPolicyPolicyElAzEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.az = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.az = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `hardware`.\n"]
    pub fn set_hardware(
        mut self,
        v: impl Into<BlockAssignable<ResiliencehubResiliencyPolicyPolicyElHardwareEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hardware = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hardware = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(
        mut self,
        v: impl Into<BlockAssignable<ResiliencehubResiliencyPolicyPolicyElRegionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.region = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.region = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `software`.\n"]
    pub fn set_software(
        mut self,
        v: impl Into<BlockAssignable<ResiliencehubResiliencyPolicyPolicyElSoftwareEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.software = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.software = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for ResiliencehubResiliencyPolicyPolicyEl {
    type O = BlockAssignable<ResiliencehubResiliencyPolicyPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicyPolicyEl {}

impl BuildResiliencehubResiliencyPolicyPolicyEl {
    pub fn build(self) -> ResiliencehubResiliencyPolicyPolicyEl {
        ResiliencehubResiliencyPolicyPolicyEl {
            az: core::default::Default::default(),
            hardware: core::default::Default::default(),
            region: core::default::Default::default(),
            software: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ResiliencehubResiliencyPolicyPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ResiliencehubResiliencyPolicyPolicyElRef {
    fn new(shared: StackShared, base: String) -> ResiliencehubResiliencyPolicyPolicyElRef {
        ResiliencehubResiliencyPolicyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicyPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `az` after provisioning.\n"]
    pub fn az(&self) -> ListRef<ResiliencehubResiliencyPolicyPolicyElAzElRef> {
        ListRef::new(self.shared().clone(), format!("{}.az", self.base))
    }

    #[doc = "Get a reference to the value of field `hardware` after provisioning.\n"]
    pub fn hardware(&self) -> ListRef<ResiliencehubResiliencyPolicyPolicyElHardwareElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hardware", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> ListRef<ResiliencehubResiliencyPolicyPolicyElRegionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `software` after provisioning.\n"]
    pub fn software(&self) -> ListRef<ResiliencehubResiliencyPolicyPolicyElSoftwareElRef> {
        ListRef::new(self.shared().clone(), format!("{}.software", self.base))
    }
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ResiliencehubResiliencyPolicyTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ResiliencehubResiliencyPolicyTimeoutsEl {
    type O = BlockAssignable<ResiliencehubResiliencyPolicyTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicyTimeoutsEl {}

impl BuildResiliencehubResiliencyPolicyTimeoutsEl {
    pub fn build(self) -> ResiliencehubResiliencyPolicyTimeoutsEl {
        ResiliencehubResiliencyPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ResiliencehubResiliencyPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ResiliencehubResiliencyPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ResiliencehubResiliencyPolicyTimeoutsElRef {
        ResiliencehubResiliencyPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicyTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ResiliencehubResiliencyPolicyDynamic {
    policy: Option<DynamicBlock<ResiliencehubResiliencyPolicyPolicyEl>>,
}
