use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BinaryAuthorizationPolicyData {
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
    global_policy_evaluation_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admission_whitelist_patterns:
        Option<Vec<BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_admission_rules: Option<Vec<BinaryAuthorizationPolicyClusterAdmissionRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_admission_rule: Option<Vec<BinaryAuthorizationPolicyDefaultAdmissionRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BinaryAuthorizationPolicyTimeoutsEl>,
    dynamic: BinaryAuthorizationPolicyDynamic,
}
struct BinaryAuthorizationPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BinaryAuthorizationPolicyData>,
}
#[derive(Clone)]
pub struct BinaryAuthorizationPolicy(Rc<BinaryAuthorizationPolicy_>);
impl BinaryAuthorizationPolicy {
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
    #[doc = "Set the field `description`.\nA descriptive comment."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `global_policy_evaluation_mode`.\nControls the evaluation of a Google-maintained global admission policy\nfor common system-level images. Images not covered by the global\npolicy will be subject to the project admission policy. Possible values: [\"ENABLE\", \"DISABLE\"]"]
    pub fn set_global_policy_evaluation_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().global_policy_evaluation_mode = Some(v.into());
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
    #[doc = "Set the field `admission_whitelist_patterns`.\n"]
    pub fn set_admission_whitelist_patterns(
        self,
        v: impl Into<BlockAssignable<BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().admission_whitelist_patterns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .admission_whitelist_patterns = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cluster_admission_rules`.\n"]
    pub fn set_cluster_admission_rules(
        self,
        v: impl Into<BlockAssignable<BinaryAuthorizationPolicyClusterAdmissionRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster_admission_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster_admission_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_admission_rule`.\n"]
    pub fn set_default_admission_rule(
        self,
        v: impl Into<BlockAssignable<BinaryAuthorizationPolicyDefaultAdmissionRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_admission_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_admission_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BinaryAuthorizationPolicyTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA descriptive comment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `global_policy_evaluation_mode` after provisioning.\nControls the evaluation of a Google-maintained global admission policy\nfor common system-level images. Images not covered by the global\npolicy will be subject to the project admission policy. Possible values: [\"ENABLE\", \"DISABLE\"]"]
    pub fn global_policy_evaluation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_policy_evaluation_mode", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `admission_whitelist_patterns` after provisioning.\n"]
    pub fn admission_whitelist_patterns(
        &self,
    ) -> ListRef<BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admission_whitelist_patterns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_admission_rule` after provisioning.\n"]
    pub fn default_admission_rule(
        &self,
    ) -> ListRef<BinaryAuthorizationPolicyDefaultAdmissionRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_admission_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BinaryAuthorizationPolicyTimeoutsElRef {
        BinaryAuthorizationPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BinaryAuthorizationPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BinaryAuthorizationPolicy {}
impl ToListMappable for BinaryAuthorizationPolicy {
    type O = ListRef<BinaryAuthorizationPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BinaryAuthorizationPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_binary_authorization_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBinaryAuthorizationPolicy {
    pub tf_id: String,
}
impl BuildBinaryAuthorizationPolicy {
    pub fn build(self, stack: &mut Stack) -> BinaryAuthorizationPolicy {
        let out = BinaryAuthorizationPolicy(Rc::new(BinaryAuthorizationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BinaryAuthorizationPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                global_policy_evaluation_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                admission_whitelist_patterns: core::default::Default::default(),
                cluster_admission_rules: core::default::Default::default(),
                default_admission_rule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BinaryAuthorizationPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for BinaryAuthorizationPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BinaryAuthorizationPolicyRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA descriptive comment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `global_policy_evaluation_mode` after provisioning.\nControls the evaluation of a Google-maintained global admission policy\nfor common system-level images. Images not covered by the global\npolicy will be subject to the project admission policy. Possible values: [\"ENABLE\", \"DISABLE\"]"]
    pub fn global_policy_evaluation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_policy_evaluation_mode", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `admission_whitelist_patterns` after provisioning.\n"]
    pub fn admission_whitelist_patterns(
        &self,
    ) -> ListRef<BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admission_whitelist_patterns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_admission_rule` after provisioning.\n"]
    pub fn default_admission_rule(
        &self,
    ) -> ListRef<BinaryAuthorizationPolicyDefaultAdmissionRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_admission_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BinaryAuthorizationPolicyTimeoutsElRef {
        BinaryAuthorizationPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {
    name_pattern: PrimField<String>,
}
impl BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {}
impl ToListMappable for BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {
    type O = BlockAssignable<BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {
    #[doc = "An image name pattern to whitelist, in the form\n'registry/path/to/image'. This supports a trailing * as a\nwildcard, but this is allowed only in text after the registry/\npart."]
    pub name_pattern: PrimField<String>,
}
impl BuildBinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {
    pub fn build(self) -> BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {
        BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {
            name_pattern: self.name_pattern,
        }
    }
}
pub struct BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef {
        BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name_pattern` after provisioning.\nAn image name pattern to whitelist, in the form\n'registry/path/to/image'. This supports a trailing * as a\nwildcard, but this is allowed only in text after the registry/\npart."]
    pub fn name_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_pattern", self.base))
    }
}
#[derive(Serialize)]
pub struct BinaryAuthorizationPolicyClusterAdmissionRulesEl {
    cluster: PrimField<String>,
    enforcement_mode: PrimField<String>,
    evaluation_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_attestations_by: Option<SetField<PrimField<String>>>,
}
impl BinaryAuthorizationPolicyClusterAdmissionRulesEl {
    #[doc = "Set the field `require_attestations_by`.\nThe resource names of the attestors that must attest to a\ncontainer image. If the attestor is in a different project from the\npolicy, it should be specified in the format 'projects/*/attestors/*'.\nEach attestor must exist before a policy can reference it. To add an\nattestor to a policy the principal issuing the policy change\nrequest must be able to read the attestor resource.\n\nNote: this field must be non-empty when the evaluation_mode field\nspecifies REQUIRE_ATTESTATION, otherwise it must be empty."]
    pub fn set_require_attestations_by(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.require_attestations_by = Some(v.into());
        self
    }
}
impl ToListMappable for BinaryAuthorizationPolicyClusterAdmissionRulesEl {
    type O = BlockAssignable<BinaryAuthorizationPolicyClusterAdmissionRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBinaryAuthorizationPolicyClusterAdmissionRulesEl {
    #[doc = ""]
    pub cluster: PrimField<String>,
    #[doc = "The action when a pod creation is denied by the admission rule. Possible values: [\"ENFORCED_BLOCK_AND_AUDIT_LOG\", \"DRYRUN_AUDIT_LOG_ONLY\"]"]
    pub enforcement_mode: PrimField<String>,
    #[doc = "How this admission rule will be evaluated. Possible values: [\"ALWAYS_ALLOW\", \"REQUIRE_ATTESTATION\", \"ALWAYS_DENY\"]"]
    pub evaluation_mode: PrimField<String>,
}
impl BuildBinaryAuthorizationPolicyClusterAdmissionRulesEl {
    pub fn build(self) -> BinaryAuthorizationPolicyClusterAdmissionRulesEl {
        BinaryAuthorizationPolicyClusterAdmissionRulesEl {
            cluster: self.cluster,
            enforcement_mode: self.enforcement_mode,
            evaluation_mode: self.evaluation_mode,
            require_attestations_by: core::default::Default::default(),
        }
    }
}
pub struct BinaryAuthorizationPolicyClusterAdmissionRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BinaryAuthorizationPolicyClusterAdmissionRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BinaryAuthorizationPolicyClusterAdmissionRulesElRef {
        BinaryAuthorizationPolicyClusterAdmissionRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BinaryAuthorizationPolicyClusterAdmissionRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `enforcement_mode` after provisioning.\nThe action when a pod creation is denied by the admission rule. Possible values: [\"ENFORCED_BLOCK_AND_AUDIT_LOG\", \"DRYRUN_AUDIT_LOG_ONLY\"]"]
    pub fn enforcement_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcement_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `evaluation_mode` after provisioning.\nHow this admission rule will be evaluated. Possible values: [\"ALWAYS_ALLOW\", \"REQUIRE_ATTESTATION\", \"ALWAYS_DENY\"]"]
    pub fn evaluation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `require_attestations_by` after provisioning.\nThe resource names of the attestors that must attest to a\ncontainer image. If the attestor is in a different project from the\npolicy, it should be specified in the format 'projects/*/attestors/*'.\nEach attestor must exist before a policy can reference it. To add an\nattestor to a policy the principal issuing the policy change\nrequest must be able to read the attestor resource.\n\nNote: this field must be non-empty when the evaluation_mode field\nspecifies REQUIRE_ATTESTATION, otherwise it must be empty."]
    pub fn require_attestations_by(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.require_attestations_by", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BinaryAuthorizationPolicyDefaultAdmissionRuleEl {
    enforcement_mode: PrimField<String>,
    evaluation_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_attestations_by: Option<SetField<PrimField<String>>>,
}
impl BinaryAuthorizationPolicyDefaultAdmissionRuleEl {
    #[doc = "Set the field `require_attestations_by`.\nThe resource names of the attestors that must attest to a\ncontainer image. If the attestor is in a different project from the\npolicy, it should be specified in the format 'projects/*/attestors/*'.\nEach attestor must exist before a policy can reference it. To add an\nattestor to a policy the principal issuing the policy change\nrequest must be able to read the attestor resource.\n\nNote: this field must be non-empty when the evaluation_mode field\nspecifies REQUIRE_ATTESTATION, otherwise it must be empty."]
    pub fn set_require_attestations_by(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.require_attestations_by = Some(v.into());
        self
    }
}
impl ToListMappable for BinaryAuthorizationPolicyDefaultAdmissionRuleEl {
    type O = BlockAssignable<BinaryAuthorizationPolicyDefaultAdmissionRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBinaryAuthorizationPolicyDefaultAdmissionRuleEl {
    #[doc = "The action when a pod creation is denied by the admission rule. Possible values: [\"ENFORCED_BLOCK_AND_AUDIT_LOG\", \"DRYRUN_AUDIT_LOG_ONLY\"]"]
    pub enforcement_mode: PrimField<String>,
    #[doc = "How this admission rule will be evaluated. Possible values: [\"ALWAYS_ALLOW\", \"REQUIRE_ATTESTATION\", \"ALWAYS_DENY\"]"]
    pub evaluation_mode: PrimField<String>,
}
impl BuildBinaryAuthorizationPolicyDefaultAdmissionRuleEl {
    pub fn build(self) -> BinaryAuthorizationPolicyDefaultAdmissionRuleEl {
        BinaryAuthorizationPolicyDefaultAdmissionRuleEl {
            enforcement_mode: self.enforcement_mode,
            evaluation_mode: self.evaluation_mode,
            require_attestations_by: core::default::Default::default(),
        }
    }
}
pub struct BinaryAuthorizationPolicyDefaultAdmissionRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BinaryAuthorizationPolicyDefaultAdmissionRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BinaryAuthorizationPolicyDefaultAdmissionRuleElRef {
        BinaryAuthorizationPolicyDefaultAdmissionRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BinaryAuthorizationPolicyDefaultAdmissionRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforcement_mode` after provisioning.\nThe action when a pod creation is denied by the admission rule. Possible values: [\"ENFORCED_BLOCK_AND_AUDIT_LOG\", \"DRYRUN_AUDIT_LOG_ONLY\"]"]
    pub fn enforcement_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcement_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `evaluation_mode` after provisioning.\nHow this admission rule will be evaluated. Possible values: [\"ALWAYS_ALLOW\", \"REQUIRE_ATTESTATION\", \"ALWAYS_DENY\"]"]
    pub fn evaluation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `require_attestations_by` after provisioning.\nThe resource names of the attestors that must attest to a\ncontainer image. If the attestor is in a different project from the\npolicy, it should be specified in the format 'projects/*/attestors/*'.\nEach attestor must exist before a policy can reference it. To add an\nattestor to a policy the principal issuing the policy change\nrequest must be able to read the attestor resource.\n\nNote: this field must be non-empty when the evaluation_mode field\nspecifies REQUIRE_ATTESTATION, otherwise it must be empty."]
    pub fn require_attestations_by(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.require_attestations_by", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BinaryAuthorizationPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BinaryAuthorizationPolicyTimeoutsEl {
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
impl ToListMappable for BinaryAuthorizationPolicyTimeoutsEl {
    type O = BlockAssignable<BinaryAuthorizationPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBinaryAuthorizationPolicyTimeoutsEl {}
impl BuildBinaryAuthorizationPolicyTimeoutsEl {
    pub fn build(self) -> BinaryAuthorizationPolicyTimeoutsEl {
        BinaryAuthorizationPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BinaryAuthorizationPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BinaryAuthorizationPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BinaryAuthorizationPolicyTimeoutsElRef {
        BinaryAuthorizationPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BinaryAuthorizationPolicyTimeoutsElRef {
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
struct BinaryAuthorizationPolicyDynamic {
    admission_whitelist_patterns:
        Option<DynamicBlock<BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl>>,
    cluster_admission_rules: Option<DynamicBlock<BinaryAuthorizationPolicyClusterAdmissionRulesEl>>,
    default_admission_rule: Option<DynamicBlock<BinaryAuthorizationPolicyDefaultAdmissionRuleEl>>,
}
