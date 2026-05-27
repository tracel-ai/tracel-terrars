use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DnsResponsePolicyRuleData {
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
    dns_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    response_policy: PrimField<String>,
    rule_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_data: Option<Vec<DnsResponsePolicyRuleLocalDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DnsResponsePolicyRuleTimeoutsEl>,
    dynamic: DnsResponsePolicyRuleDynamic,
}
struct DnsResponsePolicyRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DnsResponsePolicyRuleData>,
}
#[derive(Clone)]
pub struct DnsResponsePolicyRule(Rc<DnsResponsePolicyRule_>);
impl DnsResponsePolicyRule {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `local_data`.\n"]
    pub fn set_local_data(
        self,
        v: impl Into<BlockAssignable<DnsResponsePolicyRuleLocalDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().local_data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.local_data = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DnsResponsePolicyRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\nThe DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule."]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `response_policy` after provisioning.\nIdentifies the response policy addressed by this request."]
    pub fn response_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\nAn identifier for this rule. Must be unique with the ResponsePolicy."]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `local_data` after provisioning.\n"]
    pub fn local_data(&self) -> ListRef<DnsResponsePolicyRuleLocalDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.local_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsResponsePolicyRuleTimeoutsElRef {
        DnsResponsePolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DnsResponsePolicyRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DnsResponsePolicyRule {}
impl ToListMappable for DnsResponsePolicyRule {
    type O = ListRef<DnsResponsePolicyRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DnsResponsePolicyRule_ {
    fn extract_resource_type(&self) -> String {
        "google_dns_response_policy_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDnsResponsePolicyRule {
    pub tf_id: String,
    #[doc = "The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule."]
    pub dns_name: PrimField<String>,
    #[doc = "Identifies the response policy addressed by this request."]
    pub response_policy: PrimField<String>,
    #[doc = "An identifier for this rule. Must be unique with the ResponsePolicy."]
    pub rule_name: PrimField<String>,
}
impl BuildDnsResponsePolicyRule {
    pub fn build(self, stack: &mut Stack) -> DnsResponsePolicyRule {
        let out = DnsResponsePolicyRule(Rc::new(DnsResponsePolicyRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DnsResponsePolicyRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                dns_name: self.dns_name,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                response_policy: self.response_policy,
                rule_name: self.rule_name,
                local_data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DnsResponsePolicyRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsResponsePolicyRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DnsResponsePolicyRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\nThe DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule."]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `response_policy` after provisioning.\nIdentifies the response policy addressed by this request."]
    pub fn response_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\nAn identifier for this rule. Must be unique with the ResponsePolicy."]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `local_data` after provisioning.\n"]
    pub fn local_data(&self) -> ListRef<DnsResponsePolicyRuleLocalDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.local_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsResponsePolicyRuleTimeoutsElRef {
        DnsResponsePolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DnsResponsePolicyRuleLocalDataElLocalDatasEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rrdatas: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl DnsResponsePolicyRuleLocalDataElLocalDatasEl {
    #[doc = "Set the field `rrdatas`.\nAs defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1)"]
    pub fn set_rrdatas(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.rrdatas = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\nNumber of seconds that this ResourceRecordSet can be cached by\nresolvers."]
    pub fn set_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DnsResponsePolicyRuleLocalDataElLocalDatasEl {
    type O = BlockAssignable<DnsResponsePolicyRuleLocalDataElLocalDatasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsResponsePolicyRuleLocalDataElLocalDatasEl {
    #[doc = "For example, www.example.com."]
    pub name: PrimField<String>,
    #[doc = "One of valid DNS resource types. Possible values: [\"A\", \"AAAA\", \"CAA\", \"CNAME\", \"DNSKEY\", \"DS\", \"HTTPS\", \"IPSECVPNKEY\", \"MX\", \"NAPTR\", \"NS\", \"PTR\", \"SOA\", \"SPF\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"TXT\"]"]
    pub type_: PrimField<String>,
}
impl BuildDnsResponsePolicyRuleLocalDataElLocalDatasEl {
    pub fn build(self) -> DnsResponsePolicyRuleLocalDataElLocalDatasEl {
        DnsResponsePolicyRuleLocalDataElLocalDatasEl {
            name: self.name,
            rrdatas: core::default::Default::default(),
            ttl: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct DnsResponsePolicyRuleLocalDataElLocalDatasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsResponsePolicyRuleLocalDataElLocalDatasElRef {
    fn new(shared: StackShared, base: String) -> DnsResponsePolicyRuleLocalDataElLocalDatasElRef {
        DnsResponsePolicyRuleLocalDataElLocalDatasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsResponsePolicyRuleLocalDataElLocalDatasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFor example, www.example.com."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `rrdatas` after provisioning.\nAs defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1)"]
    pub fn rrdatas(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nNumber of seconds that this ResourceRecordSet can be cached by\nresolvers."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOne of valid DNS resource types. Possible values: [\"A\", \"AAAA\", \"CAA\", \"CNAME\", \"DNSKEY\", \"DS\", \"HTTPS\", \"IPSECVPNKEY\", \"MX\", \"NAPTR\", \"NS\", \"PTR\", \"SOA\", \"SPF\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"TXT\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct DnsResponsePolicyRuleLocalDataElDynamic {
    local_datas: Option<DynamicBlock<DnsResponsePolicyRuleLocalDataElLocalDatasEl>>,
}
#[derive(Serialize)]
pub struct DnsResponsePolicyRuleLocalDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    local_datas: Option<Vec<DnsResponsePolicyRuleLocalDataElLocalDatasEl>>,
    dynamic: DnsResponsePolicyRuleLocalDataElDynamic,
}
impl DnsResponsePolicyRuleLocalDataEl {
    #[doc = "Set the field `local_datas`.\n"]
    pub fn set_local_datas(
        mut self,
        v: impl Into<BlockAssignable<DnsResponsePolicyRuleLocalDataElLocalDatasEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.local_datas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.local_datas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsResponsePolicyRuleLocalDataEl {
    type O = BlockAssignable<DnsResponsePolicyRuleLocalDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsResponsePolicyRuleLocalDataEl {}
impl BuildDnsResponsePolicyRuleLocalDataEl {
    pub fn build(self) -> DnsResponsePolicyRuleLocalDataEl {
        DnsResponsePolicyRuleLocalDataEl {
            local_datas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsResponsePolicyRuleLocalDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsResponsePolicyRuleLocalDataElRef {
    fn new(shared: StackShared, base: String) -> DnsResponsePolicyRuleLocalDataElRef {
        DnsResponsePolicyRuleLocalDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsResponsePolicyRuleLocalDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `local_datas` after provisioning.\n"]
    pub fn local_datas(&self) -> ListRef<DnsResponsePolicyRuleLocalDataElLocalDatasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.local_datas", self.base))
    }
}
#[derive(Serialize)]
pub struct DnsResponsePolicyRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DnsResponsePolicyRuleTimeoutsEl {
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
impl ToListMappable for DnsResponsePolicyRuleTimeoutsEl {
    type O = BlockAssignable<DnsResponsePolicyRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsResponsePolicyRuleTimeoutsEl {}
impl BuildDnsResponsePolicyRuleTimeoutsEl {
    pub fn build(self) -> DnsResponsePolicyRuleTimeoutsEl {
        DnsResponsePolicyRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DnsResponsePolicyRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsResponsePolicyRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DnsResponsePolicyRuleTimeoutsElRef {
        DnsResponsePolicyRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsResponsePolicyRuleTimeoutsElRef {
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
struct DnsResponsePolicyRuleDynamic {
    local_data: Option<DynamicBlock<DnsResponsePolicyRuleLocalDataEl>>,
}
