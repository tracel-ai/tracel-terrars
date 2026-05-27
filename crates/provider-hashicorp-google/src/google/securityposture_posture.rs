use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecurityposturePostureData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    parent: PrimField<String>,
    posture_id: PrimField<String>,
    state: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_sets: Option<Vec<SecurityposturePosturePolicySetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecurityposturePostureTimeoutsEl>,
    dynamic: SecurityposturePostureDynamic,
}
struct SecurityposturePosture_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecurityposturePostureData>,
}
#[derive(Clone)]
pub struct SecurityposturePosture(Rc<SecurityposturePosture_>);
impl SecurityposturePosture {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nDescription of the posture."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_sets`.\n"]
    pub fn set_policy_sets(
        self,
        v: impl Into<BlockAssignable<SecurityposturePosturePolicySetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy_sets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy_sets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SecurityposturePostureTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the Posture was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the posture."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the resource, eg: global."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the posture."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource, an organization. Format should be 'organizations/{organization_id}'."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `posture_id` after provisioning.\nId of the posture. It is an immutable field."]
    pub fn posture_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.posture_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the posture."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_id` after provisioning.\nRevision_id of the posture."]
    pub fn revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the posture. Update to state field should not be triggered along with\nwith other field updates. Possible values: [\"DEPRECATED\", \"DRAFT\", \"ACTIVE\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the Posture was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_sets` after provisioning.\n"]
    pub fn policy_sets(&self) -> ListRef<SecurityposturePosturePolicySetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_sets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecurityposturePostureTimeoutsElRef {
        SecurityposturePostureTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SecurityposturePosture {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecurityposturePosture {}
impl ToListMappable for SecurityposturePosture {
    type O = ListRef<SecurityposturePostureRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecurityposturePosture_ {
    fn extract_resource_type(&self) -> String {
        "google_securityposture_posture".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecurityposturePosture {
    pub tf_id: String,
    #[doc = "Location of the resource, eg: global."]
    pub location: PrimField<String>,
    #[doc = "The parent of the resource, an organization. Format should be 'organizations/{organization_id}'."]
    pub parent: PrimField<String>,
    #[doc = "Id of the posture. It is an immutable field."]
    pub posture_id: PrimField<String>,
    #[doc = "State of the posture. Update to state field should not be triggered along with\nwith other field updates. Possible values: [\"DEPRECATED\", \"DRAFT\", \"ACTIVE\"]"]
    pub state: PrimField<String>,
}
impl BuildSecurityposturePosture {
    pub fn build(self, stack: &mut Stack) -> SecurityposturePosture {
        let out = SecurityposturePosture(Rc::new(SecurityposturePosture_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecurityposturePostureData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
                posture_id: self.posture_id,
                state: self.state,
                policy_sets: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SecurityposturePostureRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePostureRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecurityposturePostureRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the Posture was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the posture."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the resource, eg: global."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the posture."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource, an organization. Format should be 'organizations/{organization_id}'."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `posture_id` after provisioning.\nId of the posture. It is an immutable field."]
    pub fn posture_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.posture_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the posture."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_id` after provisioning.\nRevision_id of the posture."]
    pub fn revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the posture. Update to state field should not be triggered along with\nwith other field updates. Possible values: [\"DEPRECATED\", \"DRAFT\", \"ACTIVE\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the Posture was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_sets` after provisioning.\n"]
    pub fn policy_sets(&self) -> ListRef<SecurityposturePosturePolicySetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_sets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecurityposturePostureTimeoutsElRef {
        SecurityposturePostureTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard: Option<PrimField<String>>,
}
impl SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsEl {
    #[doc = "Set the field `control`.\nMapping of security controls for the policy."]
    pub fn set_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.control = Some(v.into());
        self
    }
    #[doc = "Set the field `standard`.\nMapping of compliance standards for the policy."]
    pub fn set_standard(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.standard = Some(v.into());
        self
    }
}
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsEl {
    type O = BlockAssignable<SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElComplianceStandardsEl {}
impl BuildSecurityposturePosturePolicySetsElPoliciesElComplianceStandardsEl {
    pub fn build(self) -> SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsEl {
        SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsEl {
            control: core::default::Default::default(),
            standard: core::default::Default::default(),
        }
    }
}
pub struct SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsElRef {
        SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `control` after provisioning.\nMapping of security controls for the policy."]
    pub fn control(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control", self.base))
    }
    #[doc = "Get a reference to the value of field `standard` after provisioning.\nMapping of compliance standards for the policy."]
    pub fn standard(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.standard", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionEl { # [doc = "Set the field `description`.\nDescription of the expression"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"] pub fn set_location (mut self , v : impl Into < PrimField < String > >) -> Self { self . location = Some (v . into ()) ; self } # [doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose."] pub fn set_title (mut self , v : impl Into < PrimField < String > >) -> Self { self . title = Some (v . into ()) ; self } }
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionEl
{
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionEl { description : core :: default :: Default :: default () , expression : self . expression , location : core :: default :: Default :: default () , title : core :: default :: Default :: default () , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."] pub fn expression (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.expression" , self . base)) } # [doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"] pub fn location (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.location" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose."] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_values: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denied_values: Option<ListField<PrimField<String>>>,
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesEl { # [doc = "Set the field `allowed_values`.\nList of values allowed at this resource."] pub fn set_allowed_values (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . allowed_values = Some (v . into ()) ; self } # [doc = "Set the field `denied_values`.\nList of values denied at this resource."] pub fn set_denied_values (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . denied_values = Some (v . into ()) ; self } }
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesEl
{}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesEl { allowed_values : core :: default :: Default :: default () , denied_values : core :: default :: Default :: default () , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allowed_values` after provisioning.\nList of values allowed at this resource."] pub fn allowed_values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.allowed_values" , self . base)) } # [doc = "Get a reference to the value of field `denied_values` after provisioning.\nList of values denied at this resource."] pub fn denied_values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.denied_values" , self . base)) } }
#[derive(Serialize, Default)]
struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElDynamic { condition : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionEl >> , values : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesEl >> , }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesEl { # [serde (skip_serializing_if = "Option::is_none")] allow_all : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] deny_all : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] enforce : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] condition : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionEl > > , # [serde (skip_serializing_if = "Option::is_none")] values : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesEl > > , dynamic : SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElDynamic , }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesEl {
    #[doc = "Set the field `allow_all`.\nSetting this to true means that all values are allowed. This field can be set only in policies for list constraints."]
    pub fn set_allow_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_all = Some(v.into());
        self
    }
    #[doc = "Set the field `deny_all`.\nSetting this to true means that all values are denied. This field can be set only in policies for list constraints."]
    pub fn set_deny_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deny_all = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce`.\nIf 'true', then the policy is enforced. If 'false', then any configuration is acceptable.\nThis field can be set only in policies for boolean constraints."]
    pub fn set_enforce(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforce = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesEl
{
    type O = BlockAssignable<
        SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesEl
{}
impl
    BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesEl
{
    pub fn build(
        self,
    ) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesEl
    {
        SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesEl {
            allow_all: core::default::Default::default(),
            deny_all: core::default::Default::default(),
            enforce: core::default::Default::default(),
            condition: core::default::Default::default(),
            values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_all` after provisioning.\nSetting this to true means that all values are allowed. This field can be set only in policies for list constraints."]
    pub fn allow_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_all", self.base))
    }
    #[doc = "Get a reference to the value of field `deny_all` after provisioning.\nSetting this to true means that all values are denied. This field can be set only in policies for list constraints."]
    pub fn deny_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deny_all", self.base))
    }
    #[doc = "Get a reference to the value of field `enforce` after provisioning.\nIf 'true', then the policy is enforced. If 'false', then any configuration is acceptable.\nThis field can be set only in policies for boolean constraints."]
    pub fn enforce(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]    pub fn condition (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElConditionElRef >{
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]    pub fn values (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElValuesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElDynamic { policy_rules : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesEl >> , }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintEl { canned_constraint_id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] policy_rules : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesEl > > , dynamic : SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElDynamic , }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintEl {
    #[doc = "Set the field `policy_rules`.\n"]
    pub fn set_policy_rules(
        mut self,
        v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policy_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_rules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintEl
{
    type O = BlockAssignable<
        SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintEl {
    #[doc = "Organization policy canned constraint Id"]
    pub canned_constraint_id: PrimField<String>,
}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintEl {
    pub fn build(
        self,
    ) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintEl {
        SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintEl {
            canned_constraint_id: self.canned_constraint_id,
            policy_rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElRef {
        SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `canned_constraint_id` after provisioning.\nOrganization policy canned constraint Id"]
    pub fn canned_constraint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.canned_constraint_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_rules` after provisioning.\n"]    pub fn policy_rules (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElPolicyRulesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.policy_rules", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintEl
{
    action_type: PrimField<String>,
    condition: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    method_types: ListField<PrimField<String>>,
    name: PrimField<String>,
    resource_types: ListField<PrimField<String>>,
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintEl { # [doc = "Set the field `description`.\nA human-friendly description of the constraint to display as an error message when the policy is violated."] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `display_name`.\nA human-friendly name for the constraint."] pub fn set_display_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . display_name = Some (v . into ()) ; self } }
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintEl
{
    #[doc = "The action to take if the condition is met. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub action_type: PrimField<String>,
    #[doc = "A CEL condition that refers to a supported service resource, for example 'resource.management.autoUpgrade == false'. For details about CEL usage, see [Common Expression Language](https://docs.cloud.google.com/resource-manager/docs/organization-policy/creating-managing-custom-constraints#common_expression_language)."]
    pub condition: PrimField<String>,
    #[doc = "A list of RESTful methods for which to enforce the constraint. Can be 'CREATE', 'UPDATE', or both. Not all Google Cloud services support both methods. To see supported methods for each service, find the service in [Supported services](https://docs.cloud.google.com/resource-manager/docs/organization-policy/custom-constraint-supported-services)."]
    pub method_types: ListField<PrimField<String>>,
    #[doc = "Immutable. The name of the custom constraint. This is unique within the organization."]
    pub name: PrimField<String>,
    #[doc = "Immutable. The fully qualified name of the Google Cloud REST resource containing the object and field you want to restrict. For example, 'container.googleapis.com/NodePool'."]
    pub resource_types: ListField<PrimField<String>>,
}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintEl { action_type : self . action_type , condition : self . condition , description : core :: default :: Default :: default () , display_name : core :: default :: Default :: default () , method_types : self . method_types , name : self . name , resource_types : self . resource_types , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `action_type` after provisioning.\nThe action to take if the condition is met. Possible values: [\"ALLOW\", \"DENY\"]"] pub fn action_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.action_type" , self . base)) } # [doc = "Get a reference to the value of field `condition` after provisioning.\nA CEL condition that refers to a supported service resource, for example 'resource.management.autoUpgrade == false'. For details about CEL usage, see [Common Expression Language](https://docs.cloud.google.com/resource-manager/docs/organization-policy/creating-managing-custom-constraints#common_expression_language)."] pub fn condition (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.condition" , self . base)) } # [doc = "Get a reference to the value of field `description` after provisioning.\nA human-friendly description of the constraint to display as an error message when the policy is violated."] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `display_name` after provisioning.\nA human-friendly name for the constraint."] pub fn display_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.display_name" , self . base)) } # [doc = "Get a reference to the value of field `method_types` after provisioning.\nA list of RESTful methods for which to enforce the constraint. Can be 'CREATE', 'UPDATE', or both. Not all Google Cloud services support both methods. To see supported methods for each service, find the service in [Supported services](https://docs.cloud.google.com/resource-manager/docs/organization-policy/custom-constraint-supported-services)."] pub fn method_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.method_types" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. The name of the custom constraint. This is unique within the organization."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `resource_types` after provisioning.\nImmutable. The fully qualified name of the Google Cloud REST resource containing the object and field you want to restrict. For example, 'container.googleapis.com/NodePool'."] pub fn resource_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.resource_types" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionEl { # [doc = "Set the field `description`.\nDescription of the expression"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"] pub fn set_location (mut self , v : impl Into < PrimField < String > >) -> Self { self . location = Some (v . into ()) ; self } # [doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose."] pub fn set_title (mut self , v : impl Into < PrimField < String > >) -> Self { self . title = Some (v . into ()) ; self } }
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionEl
{
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionEl { description : core :: default :: Default :: default () , expression : self . expression , location : core :: default :: Default :: default () , title : core :: default :: Default :: default () , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."] pub fn expression (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.expression" , self . base)) } # [doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"] pub fn location (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.location" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose."] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_values: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denied_values: Option<ListField<PrimField<String>>>,
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesEl { # [doc = "Set the field `allowed_values`.\nList of values allowed at this resource."] pub fn set_allowed_values (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . allowed_values = Some (v . into ()) ; self } # [doc = "Set the field `denied_values`.\nList of values denied at this resource."] pub fn set_denied_values (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . denied_values = Some (v . into ()) ; self } }
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesEl
{}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesEl { allowed_values : core :: default :: Default :: default () , denied_values : core :: default :: Default :: default () , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allowed_values` after provisioning.\nList of values allowed at this resource."] pub fn allowed_values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.allowed_values" , self . base)) } # [doc = "Get a reference to the value of field `denied_values` after provisioning.\nList of values denied at this resource."] pub fn denied_values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.denied_values" , self . base)) } }
#[derive(Serialize, Default)]
struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElDynamic { condition : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionEl >> , values : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesEl >> , }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesEl { # [serde (skip_serializing_if = "Option::is_none")] allow_all : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] deny_all : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] enforce : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] condition : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionEl > > , # [serde (skip_serializing_if = "Option::is_none")] values : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesEl > > , dynamic : SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElDynamic , }
impl
    SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesEl
{
    #[doc = "Set the field `allow_all`.\nSetting this to true means that all values are allowed. This field can be set only in policies for list constraints."]
    pub fn set_allow_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_all = Some(v.into());
        self
    }
    #[doc = "Set the field `deny_all`.\nSetting this to true means that all values are denied. This field can be set only in policies for list constraints."]
    pub fn set_deny_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deny_all = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce`.\nIf 'true', then the policy is enforced. If 'false', then any configuration is acceptable.\nThis field can be set only in policies for boolean constraints."]
    pub fn set_enforce(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforce = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesEl
{}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesEl { allow_all : core :: default :: Default :: default () , deny_all : core :: default :: Default :: default () , enforce : core :: default :: Default :: default () , condition : core :: default :: Default :: default () , values : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_all` after provisioning.\nSetting this to true means that all values are allowed. This field can be set only in policies for list constraints."] pub fn allow_all (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_all" , self . base)) } # [doc = "Get a reference to the value of field `deny_all` after provisioning.\nSetting this to true means that all values are denied. This field can be set only in policies for list constraints."] pub fn deny_all (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.deny_all" , self . base)) } # [doc = "Get a reference to the value of field `enforce` after provisioning.\nIf 'true', then the policy is enforced. If 'false', then any configuration is acceptable.\nThis field can be set only in policies for boolean constraints."] pub fn enforce (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enforce" , self . base)) } # [doc = "Get a reference to the value of field `condition` after provisioning.\n"] pub fn condition (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElConditionElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.condition" , self . base)) } # [doc = "Get a reference to the value of field `values` after provisioning.\n"] pub fn values (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElValuesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElDynamic { custom_constraint : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintEl >> , policy_rules : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesEl >> , }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomEl { # [serde (skip_serializing_if = "Option::is_none")] custom_constraint : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintEl > > , # [serde (skip_serializing_if = "Option::is_none")] policy_rules : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesEl > > , dynamic : SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElDynamic , }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomEl {
    #[doc = "Set the field `custom_constraint`.\n"]
    pub fn set_custom_constraint(
        mut self,
        v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_constraint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_constraint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `policy_rules`.\n"]
    pub fn set_policy_rules(
        mut self,
        v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policy_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_rules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomEl
{
    type O = BlockAssignable<
        SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomEl
{}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomEl {
    pub fn build(
        self,
    ) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomEl {
        SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomEl {
            custom_constraint: core::default::Default::default(),
            policy_rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElRef
    {
        SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_constraint` after provisioning.\n"]    pub fn custom_constraint (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElCustomConstraintElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_constraint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_rules` after provisioning.\n"]    pub fn policy_rules (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElPolicyRulesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.policy_rules", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionEl { # [doc = "Set the field `description`.\nDescription of the expression"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"] pub fn set_location (mut self , v : impl Into < PrimField < String > >) -> Self { self . location = Some (v . into ()) ; self } # [doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose."] pub fn set_title (mut self , v : impl Into < PrimField < String > >) -> Self { self . title = Some (v . into ()) ; self } }
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionEl
{
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionEl { description : core :: default :: Default :: default () , expression : self . expression , location : core :: default :: Default :: default () , title : core :: default :: Default :: default () , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."] pub fn expression (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.expression" , self . base)) } # [doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"] pub fn location (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.location" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose."] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } }
#[derive(Serialize, Default)]
struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElDynamic { value_expression : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionEl >> , }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesEl { name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] value_expression : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionEl > > , dynamic : SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElDynamic , }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesEl { # [doc = "Set the field `value_expression`.\n"] pub fn set_value_expression (mut self , v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . value_expression = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . value_expression = Some (d) ; } } self } }
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesEl
{
    #[doc = "Name of the property for the custom output."]
    pub name: PrimField<String>,
}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesEl { name : self . name , value_expression : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nName of the property for the custom output."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value_expression` after provisioning.\n"] pub fn value_expression (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElValueExpressionElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.value_expression" , self . base)) } }
#[derive(Serialize, Default)]
struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElDynamic { properties : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesEl >> , }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputEl { # [serde (skip_serializing_if = "Option::is_none")] properties : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesEl > > , dynamic : SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElDynamic , }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputEl { # [doc = "Set the field `properties`.\n"] pub fn set_properties (mut self , v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . properties = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . properties = Some (d) ; } } self } }
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputEl
{}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputEl { properties : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `properties` after provisioning.\n"] pub fn properties (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElPropertiesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.properties" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateEl { # [doc = "Set the field `description`.\nDescription of the expression"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"] pub fn set_location (mut self , v : impl Into < PrimField < String > >) -> Self { self . location = Some (v . into ()) ; self } # [doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose."] pub fn set_title (mut self , v : impl Into < PrimField < String > >) -> Self { self . title = Some (v . into ()) ; self } }
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateEl
{
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateEl { description : core :: default :: Default :: default () , expression : self . expression , location : core :: default :: Default :: default () , title : core :: default :: Default :: default () , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."] pub fn expression (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.expression" , self . base)) } # [doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"] pub fn location (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.location" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose."] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorEl
{
    resource_types: ListField<PrimField<String>>,
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorEl { }
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorEl
{
    #[doc = "The resource types to run the detector on."]
    pub resource_types: ListField<PrimField<String>>,
}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorEl { resource_types : self . resource_types , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `resource_types` after provisioning.\nThe resource types to run the detector on."] pub fn resource_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.resource_types" , self . base)) } }
#[derive(Serialize, Default)]
struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElDynamic { custom_output : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputEl >> , predicate : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateEl >> , resource_selector : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorEl >> , }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigEl { # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] recommendation : Option < PrimField < String > > , severity : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] custom_output : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputEl > > , # [serde (skip_serializing_if = "Option::is_none")] predicate : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateEl > > , # [serde (skip_serializing_if = "Option::is_none")] resource_selector : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorEl > > , dynamic : SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElDynamic , }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigEl { # [doc = "Set the field `description`.\nText that describes the vulnerability or misconfiguration that the custom\nmodule detects."] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `recommendation`.\nAn explanation of the recommended steps that security teams can take to\nresolve the detected issue"] pub fn set_recommendation (mut self , v : impl Into < PrimField < String > >) -> Self { self . recommendation = Some (v . into ()) ; self } # [doc = "Set the field `custom_output`.\n"] pub fn set_custom_output (mut self , v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_output = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_output = Some (d) ; } } self } # [doc = "Set the field `predicate`.\n"] pub fn set_predicate (mut self , v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . predicate = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . predicate = Some (d) ; } } self } # [doc = "Set the field `resource_selector`.\n"] pub fn set_resource_selector (mut self , v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . resource_selector = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . resource_selector = Some (d) ; } } self } }
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigEl
{
    #[doc = "The severity to assign to findings generated by the module. Possible values: [\"SEVERITY_UNSPECIFIED\", \"CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub severity: PrimField<String>,
}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigEl { description : core :: default :: Default :: default () , recommendation : core :: default :: Default :: default () , severity : self . severity , custom_output : core :: default :: Default :: default () , predicate : core :: default :: Default :: default () , resource_selector : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElRef { shared : shared , base : base . to_string () , } } }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\nText that describes the vulnerability or misconfiguration that the custom\nmodule detects."] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `recommendation` after provisioning.\nAn explanation of the recommended steps that security teams can take to\nresolve the detected issue"] pub fn recommendation (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.recommendation" , self . base)) } # [doc = "Get a reference to the value of field `severity` after provisioning.\nThe severity to assign to findings generated by the module. Possible values: [\"SEVERITY_UNSPECIFIED\", \"CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"] pub fn severity (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.severity" , self . base)) } # [doc = "Get a reference to the value of field `custom_output` after provisioning.\n"] pub fn custom_output (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElCustomOutputElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_output" , self . base)) } # [doc = "Get a reference to the value of field `predicate` after provisioning.\n"] pub fn predicate (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElPredicateElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.predicate" , self . base)) } # [doc = "Get a reference to the value of field `resource_selector` after provisioning.\n"] pub fn resource_selector (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElResourceSelectorElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.resource_selector" , self . base)) } }
#[derive(Serialize, Default)]
struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElDynamic { config : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigEl >> , }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleEl { # [serde (skip_serializing_if = "Option::is_none")] display_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] module_enablement_state : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] config : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigEl > > , dynamic : SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElDynamic , }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleEl {
    #[doc = "Set the field `display_name`.\nThe display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `module_enablement_state`.\nThe state of enablement for the module at its level of the resource hierarchy. Possible values: [\"ENABLEMENT_STATE_UNSPECIFIED\", \"ENABLED\", \"DISABLED\"]"]
    pub fn set_module_enablement_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.module_enablement_state = Some(v.into());
        self
    }
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleEl { type O = BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleEl
{}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleEl { pub fn build (self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleEl { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleEl { display_name : core :: default :: Default :: default () , module_enablement_state : core :: default :: Default :: default () , config : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElRef { fn new (shared : StackShared , base : String) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElRef { SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElRef { shared : shared , base : base . to_string () , } } }
impl
    SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nA server generated id of custom module."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `module_enablement_state` after provisioning.\nThe state of enablement for the module at its level of the resource hierarchy. Possible values: [\"ENABLEMENT_STATE_UNSPECIFIED\", \"ENABLED\", \"DISABLED\"]"]
    pub fn module_enablement_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.module_enablement_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]    pub fn config (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElConfigElRef >{
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    module_enablement_state: Option<PrimField<String>>,
    module_name: PrimField<String>,
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleEl {
    #[doc = "Set the field `module_enablement_state`.\nThe state of enablement for the module at its level of the resource hierarchy. Possible values: [\"ENABLEMENT_STATE_UNSPECIFIED\", \"ENABLED\", \"DISABLED\"]"]
    pub fn set_module_enablement_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.module_enablement_state = Some(v.into());
        self
    }
}
impl ToListMappable
    for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleEl
{
    type O = BlockAssignable<
        SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleEl
{
    #[doc = "The name of the module eg: BIGQUERY_TABLE_CMEK_DISABLED."]
    pub module_name: PrimField<String>,
}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleEl {
    pub fn build(
        self,
    ) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleEl
    {
        SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleEl {
            module_enablement_state: core::default::Default::default(),
            module_name: self.module_name,
        }
    }
}
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleElRef
    {
        SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `module_enablement_state` after provisioning.\nThe state of enablement for the module at its level of the resource hierarchy. Possible values: [\"ENABLEMENT_STATE_UNSPECIFIED\", \"ENABLED\", \"DISABLED\"]"]
    pub fn module_enablement_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.module_enablement_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `module_name` after provisioning.\nThe name of the module eg: BIGQUERY_TABLE_CMEK_DISABLED."]
    pub fn module_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.module_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityposturePosturePolicySetsElPoliciesElConstraintElDynamic { org_policy_constraint : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintEl >> , org_policy_constraint_custom : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomEl >> , security_health_analytics_custom_module : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleEl >> , security_health_analytics_module : Option < DynamicBlock < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleEl >> , }
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintEl { # [serde (skip_serializing_if = "Option::is_none")] org_policy_constraint : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintEl > > , # [serde (skip_serializing_if = "Option::is_none")] org_policy_constraint_custom : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomEl > > , # [serde (skip_serializing_if = "Option::is_none")] security_health_analytics_custom_module : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleEl > > , # [serde (skip_serializing_if = "Option::is_none")] security_health_analytics_module : Option < Vec < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleEl > > , dynamic : SecurityposturePosturePolicySetsElPoliciesElConstraintElDynamic , }
impl SecurityposturePosturePolicySetsElPoliciesElConstraintEl {
    #[doc = "Set the field `org_policy_constraint`.\n"]
    pub fn set_org_policy_constraint(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.org_policy_constraint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.org_policy_constraint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `org_policy_constraint_custom`.\n"]
    pub fn set_org_policy_constraint_custom(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.org_policy_constraint_custom = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.org_policy_constraint_custom = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `security_health_analytics_custom_module`.\n"]
    pub fn set_security_health_analytics_custom_module(
        mut self,
        v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.security_health_analytics_custom_module = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.security_health_analytics_custom_module = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `security_health_analytics_module`.\n"]
    pub fn set_security_health_analytics_module(
        mut self,
        v : impl Into < BlockAssignable < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.security_health_analytics_module = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.security_health_analytics_module = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesElConstraintEl {
    type O = BlockAssignable<SecurityposturePosturePolicySetsElPoliciesElConstraintEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityposturePosturePolicySetsElPoliciesElConstraintEl {}
impl BuildSecurityposturePosturePolicySetsElPoliciesElConstraintEl {
    pub fn build(self) -> SecurityposturePosturePolicySetsElPoliciesElConstraintEl {
        SecurityposturePosturePolicySetsElPoliciesElConstraintEl {
            org_policy_constraint: core::default::Default::default(),
            org_policy_constraint_custom: core::default::Default::default(),
            security_health_analytics_custom_module: core::default::Default::default(),
            security_health_analytics_module: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityposturePosturePolicySetsElPoliciesElConstraintElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElConstraintElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecurityposturePosturePolicySetsElPoliciesElConstraintElRef {
        SecurityposturePosturePolicySetsElPoliciesElConstraintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityposturePosturePolicySetsElPoliciesElConstraintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `org_policy_constraint` after provisioning.\n"]
    pub fn org_policy_constraint(
        &self,
    ) -> ListRef<SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.org_policy_constraint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `org_policy_constraint_custom` after provisioning.\n"]
    pub fn org_policy_constraint_custom(
        &self,
    ) -> ListRef<
        SecurityposturePosturePolicySetsElPoliciesElConstraintElOrgPolicyConstraintCustomElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.org_policy_constraint_custom", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_health_analytics_custom_module` after provisioning.\n"]    pub fn security_health_analytics_custom_module (& self) -> ListRef < SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsCustomModuleElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_health_analytics_custom_module", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_health_analytics_module` after provisioning.\n"]
    pub fn security_health_analytics_module(
        &self,
    ) -> ListRef<
        SecurityposturePosturePolicySetsElPoliciesElConstraintElSecurityHealthAnalyticsModuleElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_health_analytics_module", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SecurityposturePosturePolicySetsElPoliciesElDynamic {
    compliance_standards:
        Option<DynamicBlock<SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsEl>>,
    constraint: Option<DynamicBlock<SecurityposturePosturePolicySetsElPoliciesElConstraintEl>>,
}
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsElPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    policy_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_standards:
        Option<Vec<SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    constraint: Option<Vec<SecurityposturePosturePolicySetsElPoliciesElConstraintEl>>,
    dynamic: SecurityposturePosturePolicySetsElPoliciesElDynamic,
}
impl SecurityposturePosturePolicySetsElPoliciesEl {
    #[doc = "Set the field `description`.\nDescription of the policy."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `compliance_standards`.\n"]
    pub fn set_compliance_standards(
        mut self,
        v: impl Into<BlockAssignable<SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.compliance_standards = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_standards = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `constraint`.\n"]
    pub fn set_constraint(
        mut self,
        v: impl Into<BlockAssignable<SecurityposturePosturePolicySetsElPoliciesElConstraintEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.constraint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.constraint = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityposturePosturePolicySetsElPoliciesEl {
    type O = BlockAssignable<SecurityposturePosturePolicySetsElPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityposturePosturePolicySetsElPoliciesEl {
    #[doc = "ID of the policy."]
    pub policy_id: PrimField<String>,
}
impl BuildSecurityposturePosturePolicySetsElPoliciesEl {
    pub fn build(self) -> SecurityposturePosturePolicySetsElPoliciesEl {
        SecurityposturePosturePolicySetsElPoliciesEl {
            description: core::default::Default::default(),
            policy_id: self.policy_id,
            compliance_standards: core::default::Default::default(),
            constraint: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityposturePosturePolicySetsElPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElPoliciesElRef {
    fn new(shared: StackShared, base: String) -> SecurityposturePosturePolicySetsElPoliciesElRef {
        SecurityposturePosturePolicySetsElPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityposturePosturePolicySetsElPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the policy."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nID of the policy."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.base))
    }
    #[doc = "Get a reference to the value of field `compliance_standards` after provisioning.\n"]
    pub fn compliance_standards(
        &self,
    ) -> ListRef<SecurityposturePosturePolicySetsElPoliciesElComplianceStandardsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compliance_standards", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `constraint` after provisioning.\n"]
    pub fn constraint(
        &self,
    ) -> ListRef<SecurityposturePosturePolicySetsElPoliciesElConstraintElRef> {
        ListRef::new(self.shared().clone(), format!("{}.constraint", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecurityposturePosturePolicySetsElDynamic {
    policies: Option<DynamicBlock<SecurityposturePosturePolicySetsElPoliciesEl>>,
}
#[derive(Serialize)]
pub struct SecurityposturePosturePolicySetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    policy_set_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policies: Option<Vec<SecurityposturePosturePolicySetsElPoliciesEl>>,
    dynamic: SecurityposturePosturePolicySetsElDynamic,
}
impl SecurityposturePosturePolicySetsEl {
    #[doc = "Set the field `description`.\nDescription of the policy set."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `policies`.\n"]
    pub fn set_policies(
        mut self,
        v: impl Into<BlockAssignable<SecurityposturePosturePolicySetsElPoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policies = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecurityposturePosturePolicySetsEl {
    type O = BlockAssignable<SecurityposturePosturePolicySetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityposturePosturePolicySetsEl {
    #[doc = "ID of the policy set."]
    pub policy_set_id: PrimField<String>,
}
impl BuildSecurityposturePosturePolicySetsEl {
    pub fn build(self) -> SecurityposturePosturePolicySetsEl {
        SecurityposturePosturePolicySetsEl {
            description: core::default::Default::default(),
            policy_set_id: self.policy_set_id,
            policies: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecurityposturePosturePolicySetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePosturePolicySetsElRef {
    fn new(shared: StackShared, base: String) -> SecurityposturePosturePolicySetsElRef {
        SecurityposturePosturePolicySetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityposturePosturePolicySetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the policy set."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `policy_set_id` after provisioning.\nID of the policy set."]
    pub fn policy_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_set_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> ListRef<SecurityposturePosturePolicySetsElPoliciesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policies", self.base))
    }
}
#[derive(Serialize)]
pub struct SecurityposturePostureTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SecurityposturePostureTimeoutsEl {
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
impl ToListMappable for SecurityposturePostureTimeoutsEl {
    type O = BlockAssignable<SecurityposturePostureTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecurityposturePostureTimeoutsEl {}
impl BuildSecurityposturePostureTimeoutsEl {
    pub fn build(self) -> SecurityposturePostureTimeoutsEl {
        SecurityposturePostureTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SecurityposturePostureTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecurityposturePostureTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecurityposturePostureTimeoutsElRef {
        SecurityposturePostureTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecurityposturePostureTimeoutsElRef {
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
struct SecurityposturePostureDynamic {
    policy_sets: Option<DynamicBlock<SecurityposturePosturePolicySetsEl>>,
}
