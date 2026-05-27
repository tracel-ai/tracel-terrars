use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeSecurityActionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_proxies: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    env_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    org_id: PrimField<String>,
    security_action_id: PrimField<String>,
    state: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow: Option<Vec<ApigeeSecurityActionAllowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_config: Option<Vec<ApigeeSecurityActionConditionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny: Option<Vec<ApigeeSecurityActionDenyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flag: Option<Vec<ApigeeSecurityActionFlagEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeSecurityActionTimeoutsEl>,
    dynamic: ApigeeSecurityActionDynamic,
}
struct ApigeeSecurityAction_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeSecurityActionData>,
}
#[derive(Clone)]
pub struct ApigeeSecurityAction(Rc<ApigeeSecurityAction_>);
impl ApigeeSecurityAction {
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
    #[doc = "Set the field `api_proxies`.\nIf unset, this would apply to all proxies in the environment.\nIf set, this action is enforced only if at least one proxy in the repeated\nlist is deployed at the time of enforcement. If set, several restrictions are enforced on SecurityActions.\nThere can be at most 100 enabled actions with proxies set in an env.\nSeveral other restrictions apply on conditions and are detailed later."]
    pub fn set_api_proxies(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().api_proxies = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional user provided description of the SecurityAction."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `expire_time`.\nThe expiration for this SecurityAction.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9\nfractional digits. Offsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn set_expire_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expire_time = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\nThe TTL for this SecurityAction.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_ttl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(self, v: impl Into<BlockAssignable<ApigeeSecurityActionAllowEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allow = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allow = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `condition_config`.\n"]
    pub fn set_condition_config(
        self,
        v: impl Into<BlockAssignable<ApigeeSecurityActionConditionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `deny`.\n"]
    pub fn set_deny(self, v: impl Into<BlockAssignable<ApigeeSecurityActionDenyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deny = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deny = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `flag`.\n"]
    pub fn set_flag(self, v: impl Into<BlockAssignable<ApigeeSecurityActionFlagEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().flag = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.flag = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeSecurityActionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `api_proxies` after provisioning.\nIf unset, this would apply to all proxies in the environment.\nIf set, this action is enforced only if at least one proxy in the repeated\nlist is deployed at the time of enforcement. If set, several restrictions are enforced on SecurityActions.\nThere can be at most 100 enabled actions with proxies set in an env.\nSeveral other restrictions apply on conditions and are detailed later."]
    pub fn api_proxies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_proxies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe create time for this SecurityAction.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional user provided description of the SecurityAction."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `env_id` after provisioning.\nThe Apigee environment that this security action applies to."]
    pub fn env_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.env_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nThe expiration for this SecurityAction.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9\nfractional digits. Offsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe organization that this security action applies to."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_action_id` after provisioning.\nThe ID to use for the SecurityAction, which will become the final component of the action's resource name.\nThis value should be 0-61 characters, and valid format is (^a-z?$)."]
    pub fn security_action_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_action_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOnly an ENABLED SecurityAction is enforced. An ENABLED SecurityAction past its expiration time will not be enforced. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe TTL for this SecurityAction.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe update time for this SecurityAction. This reflects when this SecurityAction changed states.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> ListRef<ApigeeSecurityActionAllowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition_config` after provisioning.\n"]
    pub fn condition_config(&self) -> ListRef<ApigeeSecurityActionConditionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deny` after provisioning.\n"]
    pub fn deny(&self) -> ListRef<ApigeeSecurityActionDenyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deny", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `flag` after provisioning.\n"]
    pub fn flag(&self) -> ListRef<ApigeeSecurityActionFlagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.flag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSecurityActionTimeoutsElRef {
        ApigeeSecurityActionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeSecurityAction {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeSecurityAction {}
impl ToListMappable for ApigeeSecurityAction {
    type O = ListRef<ApigeeSecurityActionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeSecurityAction_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_security_action".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeSecurityAction {
    pub tf_id: String,
    #[doc = "The Apigee environment that this security action applies to."]
    pub env_id: PrimField<String>,
    #[doc = "The organization that this security action applies to."]
    pub org_id: PrimField<String>,
    #[doc = "The ID to use for the SecurityAction, which will become the final component of the action's resource name.\nThis value should be 0-61 characters, and valid format is (^a-z?$)."]
    pub security_action_id: PrimField<String>,
    #[doc = "Only an ENABLED SecurityAction is enforced. An ENABLED SecurityAction past its expiration time will not be enforced. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub state: PrimField<String>,
}
impl BuildApigeeSecurityAction {
    pub fn build(self, stack: &mut Stack) -> ApigeeSecurityAction {
        let out = ApigeeSecurityAction(Rc::new(ApigeeSecurityAction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeSecurityActionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_proxies: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                env_id: self.env_id,
                expire_time: core::default::Default::default(),
                id: core::default::Default::default(),
                org_id: self.org_id,
                security_action_id: self.security_action_id,
                state: self.state,
                ttl: core::default::Default::default(),
                allow: core::default::Default::default(),
                condition_config: core::default::Default::default(),
                deny: core::default::Default::default(),
                flag: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeSecurityActionRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityActionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeSecurityActionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_proxies` after provisioning.\nIf unset, this would apply to all proxies in the environment.\nIf set, this action is enforced only if at least one proxy in the repeated\nlist is deployed at the time of enforcement. If set, several restrictions are enforced on SecurityActions.\nThere can be at most 100 enabled actions with proxies set in an env.\nSeveral other restrictions apply on conditions and are detailed later."]
    pub fn api_proxies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_proxies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe create time for this SecurityAction.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional user provided description of the SecurityAction."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `env_id` after provisioning.\nThe Apigee environment that this security action applies to."]
    pub fn env_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.env_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nThe expiration for this SecurityAction.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9\nfractional digits. Offsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe organization that this security action applies to."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_action_id` after provisioning.\nThe ID to use for the SecurityAction, which will become the final component of the action's resource name.\nThis value should be 0-61 characters, and valid format is (^a-z?$)."]
    pub fn security_action_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_action_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOnly an ENABLED SecurityAction is enforced. An ENABLED SecurityAction past its expiration time will not be enforced. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe TTL for this SecurityAction.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe update time for this SecurityAction. This reflects when this SecurityAction changed states.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> ListRef<ApigeeSecurityActionAllowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition_config` after provisioning.\n"]
    pub fn condition_config(&self) -> ListRef<ApigeeSecurityActionConditionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deny` after provisioning.\n"]
    pub fn deny(&self) -> ListRef<ApigeeSecurityActionDenyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deny", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `flag` after provisioning.\n"]
    pub fn flag(&self) -> ListRef<ApigeeSecurityActionFlagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.flag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSecurityActionTimeoutsElRef {
        ApigeeSecurityActionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeSecurityActionAllowEl {}
impl ApigeeSecurityActionAllowEl {}
impl ToListMappable for ApigeeSecurityActionAllowEl {
    type O = BlockAssignable<ApigeeSecurityActionAllowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSecurityActionAllowEl {}
impl BuildApigeeSecurityActionAllowEl {
    pub fn build(self) -> ApigeeSecurityActionAllowEl {
        ApigeeSecurityActionAllowEl {}
    }
}
pub struct ApigeeSecurityActionAllowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityActionAllowElRef {
    fn new(shared: StackShared, base: String) -> ApigeeSecurityActionAllowElRef {
        ApigeeSecurityActionAllowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSecurityActionAllowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct ApigeeSecurityActionConditionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_tokens: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_keys: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_products: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_reasons: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_apps: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_methods: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agents: Option<ListField<PrimField<String>>>,
}
impl ApigeeSecurityActionConditionConfigEl {
    #[doc = "Set the field `access_tokens`.\nA list of accessTokens. Limit 1000 per action."]
    pub fn set_access_tokens(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.access_tokens = Some(v.into());
        self
    }
    #[doc = "Set the field `api_keys`.\nA list of API keys. Limit 1000 per action."]
    pub fn set_api_keys(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.api_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `api_products`.\nA list of API Products. Limit 1000 per action."]
    pub fn set_api_products(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.api_products = Some(v.into());
        self
    }
    #[doc = "Set the field `asns`.\nA list of ASN numbers to act on, e.g. 23. https://en.wikipedia.org/wiki/Autonomous_system_(Internet)\nThis uses int64 instead of uint32 because of https://linter.aip.dev/141/forbidden-types."]
    pub fn set_asns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.asns = Some(v.into());
        self
    }
    #[doc = "Set the field `bot_reasons`.\nA list of Bot Reasons. Current options: Flooder, Brute Guessor, Static Content Scraper,\nOAuth Abuser, Robot Abuser, TorListRule, Advanced Anomaly Detection, Advanced API Scraper,\nSearch Engine Crawlers, Public Clouds, Public Cloud AWS, Public Cloud Azure, and Public Cloud Google."]
    pub fn set_bot_reasons(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.bot_reasons = Some(v.into());
        self
    }
    #[doc = "Set the field `developer_apps`.\nA list of developer apps. Limit 1000 per action."]
    pub fn set_developer_apps(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.developer_apps = Some(v.into());
        self
    }
    #[doc = "Set the field `developers`.\nA list of developers. Limit 1000 per action."]
    pub fn set_developers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.developers = Some(v.into());
        self
    }
    #[doc = "Set the field `http_methods`.\nAct only on particular HTTP methods. E.g. A read-only API can block POST/PUT/DELETE methods.\nAccepted values are: GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE and PATCH."]
    pub fn set_http_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.http_methods = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address_ranges`.\nA list of IP addresses. This could be either IPv4 or IPv6. Limited to 100 per action."]
    pub fn set_ip_address_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_address_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `region_codes`.\nA list of countries/region codes to act on, e.g. US. This follows https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2."]
    pub fn set_region_codes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.region_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `user_agents`.\nA list of user agents to deny. We look for exact matches. Limit 50 per action."]
    pub fn set_user_agents(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.user_agents = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeSecurityActionConditionConfigEl {
    type O = BlockAssignable<ApigeeSecurityActionConditionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSecurityActionConditionConfigEl {}
impl BuildApigeeSecurityActionConditionConfigEl {
    pub fn build(self) -> ApigeeSecurityActionConditionConfigEl {
        ApigeeSecurityActionConditionConfigEl {
            access_tokens: core::default::Default::default(),
            api_keys: core::default::Default::default(),
            api_products: core::default::Default::default(),
            asns: core::default::Default::default(),
            bot_reasons: core::default::Default::default(),
            developer_apps: core::default::Default::default(),
            developers: core::default::Default::default(),
            http_methods: core::default::Default::default(),
            ip_address_ranges: core::default::Default::default(),
            region_codes: core::default::Default::default(),
            user_agents: core::default::Default::default(),
        }
    }
}
pub struct ApigeeSecurityActionConditionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityActionConditionConfigElRef {
    fn new(shared: StackShared, base: String) -> ApigeeSecurityActionConditionConfigElRef {
        ApigeeSecurityActionConditionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSecurityActionConditionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_tokens` after provisioning.\nA list of accessTokens. Limit 1000 per action."]
    pub fn access_tokens(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_tokens", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `api_keys` after provisioning.\nA list of API keys. Limit 1000 per action."]
    pub fn api_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.api_keys", self.base))
    }
    #[doc = "Get a reference to the value of field `api_products` after provisioning.\nA list of API Products. Limit 1000 per action."]
    pub fn api_products(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.api_products", self.base))
    }
    #[doc = "Get a reference to the value of field `asns` after provisioning.\nA list of ASN numbers to act on, e.g. 23. https://en.wikipedia.org/wiki/Autonomous_system_(Internet)\nThis uses int64 instead of uint32 because of https://linter.aip.dev/141/forbidden-types."]
    pub fn asns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.asns", self.base))
    }
    #[doc = "Get a reference to the value of field `bot_reasons` after provisioning.\nA list of Bot Reasons. Current options: Flooder, Brute Guessor, Static Content Scraper,\nOAuth Abuser, Robot Abuser, TorListRule, Advanced Anomaly Detection, Advanced API Scraper,\nSearch Engine Crawlers, Public Clouds, Public Cloud AWS, Public Cloud Azure, and Public Cloud Google."]
    pub fn bot_reasons(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.bot_reasons", self.base))
    }
    #[doc = "Get a reference to the value of field `developer_apps` after provisioning.\nA list of developer apps. Limit 1000 per action."]
    pub fn developer_apps(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.developer_apps", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `developers` after provisioning.\nA list of developers. Limit 1000 per action."]
    pub fn developers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.developers", self.base))
    }
    #[doc = "Get a reference to the value of field `http_methods` after provisioning.\nAct only on particular HTTP methods. E.g. A read-only API can block POST/PUT/DELETE methods.\nAccepted values are: GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE and PATCH."]
    pub fn http_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.http_methods", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_address_ranges` after provisioning.\nA list of IP addresses. This could be either IPv4 or IPv6. Limited to 100 per action."]
    pub fn ip_address_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_address_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region_codes` after provisioning.\nA list of countries/region codes to act on, e.g. US. This follows https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2."]
    pub fn region_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.region_codes", self.base))
    }
    #[doc = "Get a reference to the value of field `user_agents` after provisioning.\nA list of user agents to deny. We look for exact matches. Limit 50 per action."]
    pub fn user_agents(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.user_agents", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeSecurityActionDenyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    response_code: Option<PrimField<f64>>,
}
impl ApigeeSecurityActionDenyEl {
    #[doc = "Set the field `response_code`.\nThe HTTP response code if the Action = DENY."]
    pub fn set_response_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.response_code = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeSecurityActionDenyEl {
    type O = BlockAssignable<ApigeeSecurityActionDenyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSecurityActionDenyEl {}
impl BuildApigeeSecurityActionDenyEl {
    pub fn build(self) -> ApigeeSecurityActionDenyEl {
        ApigeeSecurityActionDenyEl {
            response_code: core::default::Default::default(),
        }
    }
}
pub struct ApigeeSecurityActionDenyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityActionDenyElRef {
    fn new(shared: StackShared, base: String) -> ApigeeSecurityActionDenyElRef {
        ApigeeSecurityActionDenyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSecurityActionDenyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `response_code` after provisioning.\nThe HTTP response code if the Action = DENY."]
    pub fn response_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_code", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeSecurityActionFlagElHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ApigeeSecurityActionFlagElHeadersEl {
    #[doc = "Set the field `name`.\nThe header name to be sent to the target."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe header value to be sent to the target."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeSecurityActionFlagElHeadersEl {
    type O = BlockAssignable<ApigeeSecurityActionFlagElHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSecurityActionFlagElHeadersEl {}
impl BuildApigeeSecurityActionFlagElHeadersEl {
    pub fn build(self) -> ApigeeSecurityActionFlagElHeadersEl {
        ApigeeSecurityActionFlagElHeadersEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ApigeeSecurityActionFlagElHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityActionFlagElHeadersElRef {
    fn new(shared: StackShared, base: String) -> ApigeeSecurityActionFlagElHeadersElRef {
        ApigeeSecurityActionFlagElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSecurityActionFlagElHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe header name to be sent to the target."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe header value to be sent to the target."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApigeeSecurityActionFlagElDynamic {
    headers: Option<DynamicBlock<ApigeeSecurityActionFlagElHeadersEl>>,
}
#[derive(Serialize)]
pub struct ApigeeSecurityActionFlagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Vec<ApigeeSecurityActionFlagElHeadersEl>>,
    dynamic: ApigeeSecurityActionFlagElDynamic,
}
impl ApigeeSecurityActionFlagEl {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl Into<BlockAssignable<ApigeeSecurityActionFlagElHeadersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeSecurityActionFlagEl {
    type O = BlockAssignable<ApigeeSecurityActionFlagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSecurityActionFlagEl {}
impl BuildApigeeSecurityActionFlagEl {
    pub fn build(self) -> ApigeeSecurityActionFlagEl {
        ApigeeSecurityActionFlagEl {
            headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeSecurityActionFlagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityActionFlagElRef {
    fn new(shared: StackShared, base: String) -> ApigeeSecurityActionFlagElRef {
        ApigeeSecurityActionFlagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSecurityActionFlagElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> ListRef<ApigeeSecurityActionFlagElHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeSecurityActionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ApigeeSecurityActionTimeoutsEl {
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
}
impl ToListMappable for ApigeeSecurityActionTimeoutsEl {
    type O = BlockAssignable<ApigeeSecurityActionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSecurityActionTimeoutsEl {}
impl BuildApigeeSecurityActionTimeoutsEl {
    pub fn build(self) -> ApigeeSecurityActionTimeoutsEl {
        ApigeeSecurityActionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ApigeeSecurityActionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityActionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeSecurityActionTimeoutsElRef {
        ApigeeSecurityActionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSecurityActionTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct ApigeeSecurityActionDynamic {
    allow: Option<DynamicBlock<ApigeeSecurityActionAllowEl>>,
    condition_config: Option<DynamicBlock<ApigeeSecurityActionConditionConfigEl>>,
    deny: Option<DynamicBlock<ApigeeSecurityActionDenyEl>>,
    flag: Option<DynamicBlock<ApigeeSecurityActionFlagEl>>,
}
