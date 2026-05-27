use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChronicleRuleData {
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
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ChronicleRuleTimeoutsEl>,
}
struct ChronicleRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChronicleRuleData>,
}
#[derive(Clone)]
pub struct ChronicleRule(Rc<ChronicleRule_>);
impl ChronicleRule {
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
    #[doc = "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_rule.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\nThe etag for this rule.\nIf this is provided on update, the request will succeed if and only if it\nmatches the server-computed value, and will fail with an ABORTED error\notherwise.\nPopulated in BASIC view and FULL view."]
    pub fn set_etag(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().etag = Some(v.into());
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
    #[doc = "Set the field `rule_id`.\nRule Id is the ID of the Rule."]
    pub fn set_rule_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().rule_id = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\nResource name of the DataAccessScope bound to this rule.\nPopulated in BASIC view and FULL view.\nIf reference lists are used in the rule, validations will be performed\nagainst this scope to ensure that the reference lists are compatible with\nboth the user's and the rule's scopes.\nThe scope should be in the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope}\"."]
    pub fn set_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scope = Some(v.into());
        self
    }
    #[doc = "Set the field `text`.\nThe YARA-L content of the rule.\nPopulated in FULL view."]
    pub fn set_text(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().text = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ChronicleRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allowed_run_frequencies` after provisioning.\nOutput only. The run frequencies that are allowed for the rule.\nPopulated in BASIC view and FULL view."]
    pub fn allowed_run_frequencies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_run_frequencies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `author` after provisioning.\nOutput only. The author of the rule. Extracted from the meta section of text.\nPopulated in BASIC view and FULL view."]
    pub fn author(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.author", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compilation_diagnostics` after provisioning.\nOutput only. A list of a rule's corresponding compilation diagnostic messages\nsuch as compilation errors and compilation warnings.\nPopulated in FULL view."]
    pub fn compilation_diagnostics(&self) -> ListRef<ChronicleRuleCompilationDiagnosticsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compilation_diagnostics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compilation_state` after provisioning.\nOutput only. The current compilation state of the rule.\nPopulated in FULL view.\nPossible values:\nCOMPILATION_STATE_UNSPECIFIED\nSUCCEEDED\nFAILED"]
    pub fn compilation_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compilation_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp of when the rule was created.\nPopulated in FULL view."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_tables` after provisioning.\nOutput only. Resource names of the data tables used in this rule."]
    pub fn data_tables(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_tables", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_rule.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOutput only. Display name of the rule.\nPopulated in BASIC view and FULL view."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag for this rule.\nIf this is provided on update, the request will succeed if and only if it\nmatches the server-computed value, and will fail with an ABORTED error\notherwise.\nPopulated in BASIC view and FULL view."]
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nOutput only. Additional metadata specified in the meta section of text.\nPopulated in FULL view."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull resource name for the rule. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `near_real_time_live_rule_eligible` after provisioning.\nOutput only. Indicate the rule can run in near real time live rule.\nIf this is true, the rule uses the near real time live rule when the run\nfrequency is set to LIVE."]
    pub fn near_real_time_live_rule_eligible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.near_real_time_live_rule_eligible", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reference_lists` after provisioning.\nOutput only. Resource names of the reference lists used in this rule.\nPopulated in FULL view."]
    pub fn reference_lists(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reference_lists", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp of when the rule revision was created.\nPopulated in FULL, REVISION_METADATA_ONLY views."]
    pub fn revision_create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_id` after provisioning.\nOutput only. The revision ID of the rule.\nA new revision is created whenever the rule text is changed in any way.\nFormat: v_{10 digits}_{9 digits}\nPopulated in REVISION_METADATA_ONLY view and FULL view."]
    pub fn revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nRule Id is the ID of the Rule."]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nResource name of the DataAccessScope bound to this rule.\nPopulated in BASIC view and FULL view.\nIf reference lists are used in the rule, validations will be performed\nagainst this scope to ensure that the reference lists are compatible with\nboth the user's and the rule's scopes.\nThe scope should be in the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope}\"."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\nSeverity represents the severity level of the rule."]
    pub fn severity(&self) -> ListRef<ChronicleRuleSeverityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.severity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe YARA-L content of the rule.\nPopulated in FULL view."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.text", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nPossible values:\nRULE_TYPE_UNSPECIFIED\nSINGLE_EVENT\nMULTI_EVENT"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleRuleTimeoutsElRef {
        ChronicleRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ChronicleRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChronicleRule {}
impl ToListMappable for ChronicleRule {
    type O = ListRef<ChronicleRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChronicleRule_ {
    fn extract_resource_type(&self) -> String {
        "google_chronicle_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChronicleRule {
    pub tf_id: String,
    #[doc = "The unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub instance: PrimField<String>,
    #[doc = "The location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub location: PrimField<String>,
}
impl BuildChronicleRule {
    pub fn build(self, stack: &mut Stack) -> ChronicleRule {
        let out = ChronicleRule(Rc::new(ChronicleRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChronicleRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                rule_id: core::default::Default::default(),
                scope: core::default::Default::default(),
                text: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChronicleRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChronicleRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_run_frequencies` after provisioning.\nOutput only. The run frequencies that are allowed for the rule.\nPopulated in BASIC view and FULL view."]
    pub fn allowed_run_frequencies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_run_frequencies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `author` after provisioning.\nOutput only. The author of the rule. Extracted from the meta section of text.\nPopulated in BASIC view and FULL view."]
    pub fn author(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.author", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compilation_diagnostics` after provisioning.\nOutput only. A list of a rule's corresponding compilation diagnostic messages\nsuch as compilation errors and compilation warnings.\nPopulated in FULL view."]
    pub fn compilation_diagnostics(&self) -> ListRef<ChronicleRuleCompilationDiagnosticsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compilation_diagnostics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compilation_state` after provisioning.\nOutput only. The current compilation state of the rule.\nPopulated in FULL view.\nPossible values:\nCOMPILATION_STATE_UNSPECIFIED\nSUCCEEDED\nFAILED"]
    pub fn compilation_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compilation_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp of when the rule was created.\nPopulated in FULL view."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_tables` after provisioning.\nOutput only. Resource names of the data tables used in this rule."]
    pub fn data_tables(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_tables", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_rule.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOutput only. Display name of the rule.\nPopulated in BASIC view and FULL view."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag for this rule.\nIf this is provided on update, the request will succeed if and only if it\nmatches the server-computed value, and will fail with an ABORTED error\notherwise.\nPopulated in BASIC view and FULL view."]
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nOutput only. Additional metadata specified in the meta section of text.\nPopulated in FULL view."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull resource name for the rule. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `near_real_time_live_rule_eligible` after provisioning.\nOutput only. Indicate the rule can run in near real time live rule.\nIf this is true, the rule uses the near real time live rule when the run\nfrequency is set to LIVE."]
    pub fn near_real_time_live_rule_eligible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.near_real_time_live_rule_eligible", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reference_lists` after provisioning.\nOutput only. Resource names of the reference lists used in this rule.\nPopulated in FULL view."]
    pub fn reference_lists(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reference_lists", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp of when the rule revision was created.\nPopulated in FULL, REVISION_METADATA_ONLY views."]
    pub fn revision_create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision_id` after provisioning.\nOutput only. The revision ID of the rule.\nA new revision is created whenever the rule text is changed in any way.\nFormat: v_{10 digits}_{9 digits}\nPopulated in REVISION_METADATA_ONLY view and FULL view."]
    pub fn revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nRule Id is the ID of the Rule."]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nResource name of the DataAccessScope bound to this rule.\nPopulated in BASIC view and FULL view.\nIf reference lists are used in the rule, validations will be performed\nagainst this scope to ensure that the reference lists are compatible with\nboth the user's and the rule's scopes.\nThe scope should be in the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope}\"."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\nSeverity represents the severity level of the rule."]
    pub fn severity(&self) -> ListRef<ChronicleRuleSeverityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.severity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe YARA-L content of the rule.\nPopulated in FULL view."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.text", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nPossible values:\nRULE_TYPE_UNSPECIFIED\nSINGLE_EVENT\nMULTI_EVENT"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleRuleTimeoutsElRef {
        ChronicleRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleRuleCompilationDiagnosticsElPositionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_column: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_line: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_column: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_line: Option<PrimField<f64>>,
}
impl ChronicleRuleCompilationDiagnosticsElPositionEl {
    #[doc = "Set the field `end_column`.\n"]
    pub fn set_end_column(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.end_column = Some(v.into());
        self
    }
    #[doc = "Set the field `end_line`.\n"]
    pub fn set_end_line(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.end_line = Some(v.into());
        self
    }
    #[doc = "Set the field `start_column`.\n"]
    pub fn set_start_column(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.start_column = Some(v.into());
        self
    }
    #[doc = "Set the field `start_line`.\n"]
    pub fn set_start_line(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.start_line = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleRuleCompilationDiagnosticsElPositionEl {
    type O = BlockAssignable<ChronicleRuleCompilationDiagnosticsElPositionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleRuleCompilationDiagnosticsElPositionEl {}
impl BuildChronicleRuleCompilationDiagnosticsElPositionEl {
    pub fn build(self) -> ChronicleRuleCompilationDiagnosticsElPositionEl {
        ChronicleRuleCompilationDiagnosticsElPositionEl {
            end_column: core::default::Default::default(),
            end_line: core::default::Default::default(),
            start_column: core::default::Default::default(),
            start_line: core::default::Default::default(),
        }
    }
}
pub struct ChronicleRuleCompilationDiagnosticsElPositionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleRuleCompilationDiagnosticsElPositionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleRuleCompilationDiagnosticsElPositionElRef {
        ChronicleRuleCompilationDiagnosticsElPositionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleRuleCompilationDiagnosticsElPositionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_column` after provisioning.\n"]
    pub fn end_column(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_column", self.base))
    }
    #[doc = "Get a reference to the value of field `end_line` after provisioning.\n"]
    pub fn end_line(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_line", self.base))
    }
    #[doc = "Get a reference to the value of field `start_column` after provisioning.\n"]
    pub fn start_column(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_column", self.base))
    }
    #[doc = "Get a reference to the value of field `start_line` after provisioning.\n"]
    pub fn start_line(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_line", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleRuleCompilationDiagnosticsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<ListField<ChronicleRuleCompilationDiagnosticsElPositionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl ChronicleRuleCompilationDiagnosticsEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `position`.\n"]
    pub fn set_position(
        mut self,
        v: impl Into<ListField<ChronicleRuleCompilationDiagnosticsElPositionEl>>,
    ) -> Self {
        self.position = Some(v.into());
        self
    }
    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.severity = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleRuleCompilationDiagnosticsEl {
    type O = BlockAssignable<ChronicleRuleCompilationDiagnosticsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleRuleCompilationDiagnosticsEl {}
impl BuildChronicleRuleCompilationDiagnosticsEl {
    pub fn build(self) -> ChronicleRuleCompilationDiagnosticsEl {
        ChronicleRuleCompilationDiagnosticsEl {
            message: core::default::Default::default(),
            position: core::default::Default::default(),
            severity: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct ChronicleRuleCompilationDiagnosticsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleRuleCompilationDiagnosticsElRef {
    fn new(shared: StackShared, base: String) -> ChronicleRuleCompilationDiagnosticsElRef {
        ChronicleRuleCompilationDiagnosticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleRuleCompilationDiagnosticsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> ListRef<ChronicleRuleCompilationDiagnosticsElPositionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.position", self.base))
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleRuleSeverityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
}
impl ChronicleRuleSeverityEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleRuleSeverityEl {
    type O = BlockAssignable<ChronicleRuleSeverityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleRuleSeverityEl {}
impl BuildChronicleRuleSeverityEl {
    pub fn build(self) -> ChronicleRuleSeverityEl {
        ChronicleRuleSeverityEl {
            display_name: core::default::Default::default(),
        }
    }
}
pub struct ChronicleRuleSeverityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleRuleSeverityElRef {
    fn new(shared: StackShared, base: String) -> ChronicleRuleSeverityElRef {
        ChronicleRuleSeverityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleRuleSeverityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ChronicleRuleTimeoutsEl {
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
impl ToListMappable for ChronicleRuleTimeoutsEl {
    type O = BlockAssignable<ChronicleRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleRuleTimeoutsEl {}
impl BuildChronicleRuleTimeoutsEl {
    pub fn build(self) -> ChronicleRuleTimeoutsEl {
        ChronicleRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ChronicleRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ChronicleRuleTimeoutsElRef {
        ChronicleRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleRuleTimeoutsElRef {
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
