use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCertificateManagerCertificatesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataCertificateManagerCertificates_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCertificateManagerCertificatesData>,
}
#[derive(Clone)]
pub struct DataCertificateManagerCertificates(Rc<DataCertificateManagerCertificates_>);
impl DataCertificateManagerCertificates {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> ListRef<DataCertificateManagerCertificatesCertificatesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for DataCertificateManagerCertificates {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCertificateManagerCertificates {}
impl ToListMappable for DataCertificateManagerCertificates {
    type O = ListRef<DataCertificateManagerCertificatesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCertificateManagerCertificates_ {
    fn extract_datasource_type(&self) -> String {
        "google_certificate_manager_certificates".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCertificateManagerCertificates {
    pub tf_id: String,
}
impl BuildDataCertificateManagerCertificates {
    pub fn build(self, stack: &mut Stack) -> DataCertificateManagerCertificates {
        let out =
            DataCertificateManagerCertificates(Rc::new(DataCertificateManagerCertificates_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataCertificateManagerCertificatesData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    filter: core::default::Default::default(),
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCertificateManagerCertificatesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCertificateManagerCertificatesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCertificateManagerCertificatesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> ListRef<DataCertificateManagerCertificatesCertificatesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoEl {
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
    #[doc = "Set the field `failure_reason`.\n"]
    pub fn set_failure_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.failure_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoEl
{
    type O = BlockAssignable<
        DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoEl
{}
impl BuildDataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoEl {
    pub fn build(
        self,
    ) -> DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoEl {
        DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoEl {
            details: core::default::Default::default(),
            domain: core::default::Default::default(),
            failure_reason: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoElRef
    {
        DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_reason", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
}
impl DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueEl {
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueEl
{
    type O = BlockAssignable<
        DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueEl {}
impl BuildDataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueEl {
    pub fn build(
        self,
    ) -> DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueEl {
        DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueEl {
            details: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}
pub struct DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueElRef {
        DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCertificateManagerCertificatesCertificatesElManagedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_attempt_info: Option<
        ListField<
            DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_authorizations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domains: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuance_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioning_issue: Option<
        ListField<DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataCertificateManagerCertificatesCertificatesElManagedEl {
    #[doc = "Set the field `authorization_attempt_info`.\n"]
    pub fn set_authorization_attempt_info(
        mut self,
        v: impl Into<
            ListField<
                DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoEl,
            >,
        >,
    ) -> Self {
        self.authorization_attempt_info = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_authorizations`.\n"]
    pub fn set_dns_authorizations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dns_authorizations = Some(v.into());
        self
    }
    #[doc = "Set the field `domains`.\n"]
    pub fn set_domains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.domains = Some(v.into());
        self
    }
    #[doc = "Set the field `issuance_config`.\n"]
    pub fn set_issuance_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuance_config = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioning_issue`.\n"]
    pub fn set_provisioning_issue(
        mut self,
        v: impl Into<
            ListField<DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueEl>,
        >,
    ) -> Self {
        self.provisioning_issue = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for DataCertificateManagerCertificatesCertificatesElManagedEl {
    type O = BlockAssignable<DataCertificateManagerCertificatesCertificatesElManagedEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCertificateManagerCertificatesCertificatesElManagedEl {}
impl BuildDataCertificateManagerCertificatesCertificatesElManagedEl {
    pub fn build(self) -> DataCertificateManagerCertificatesCertificatesElManagedEl {
        DataCertificateManagerCertificatesCertificatesElManagedEl {
            authorization_attempt_info: core::default::Default::default(),
            dns_authorizations: core::default::Default::default(),
            domains: core::default::Default::default(),
            issuance_config: core::default::Default::default(),
            provisioning_issue: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataCertificateManagerCertificatesCertificatesElManagedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCertificateManagerCertificatesCertificatesElManagedElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCertificateManagerCertificatesCertificatesElManagedElRef {
        DataCertificateManagerCertificatesCertificatesElManagedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCertificateManagerCertificatesCertificatesElManagedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorization_attempt_info` after provisioning.\n"]
    pub fn authorization_attempt_info(
        &self,
    ) -> ListRef<
        DataCertificateManagerCertificatesCertificatesElManagedElAuthorizationAttemptInfoElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization_attempt_info", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dns_authorizations` after provisioning.\n"]
    pub fn dns_authorizations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_authorizations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }
    #[doc = "Get a reference to the value of field `issuance_config` after provisioning.\n"]
    pub fn issuance_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.issuance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provisioning_issue` after provisioning.\n"]
    pub fn provisioning_issue(
        &self,
    ) -> ListRef<DataCertificateManagerCertificatesCertificatesElManagedElProvisioningIssueElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioning_issue", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCertificateManagerCertificatesCertificatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed: Option<ListField<DataCertificateManagerCertificatesCertificatesElManagedEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    san_dnsnames: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terraform_labels: Option<RecField<PrimField<String>>>,
}
impl DataCertificateManagerCertificatesCertificatesEl {
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_labels`.\n"]
    pub fn set_effective_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.effective_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `managed`.\n"]
    pub fn set_managed(
        mut self,
        v: impl Into<ListField<DataCertificateManagerCertificatesCertificatesElManagedEl>>,
    ) -> Self {
        self.managed = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `san_dnsnames`.\n"]
    pub fn set_san_dnsnames(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.san_dnsnames = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.terraform_labels = Some(v.into());
        self
    }
}
impl ToListMappable for DataCertificateManagerCertificatesCertificatesEl {
    type O = BlockAssignable<DataCertificateManagerCertificatesCertificatesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCertificateManagerCertificatesCertificatesEl {}
impl BuildDataCertificateManagerCertificatesCertificatesEl {
    pub fn build(self) -> DataCertificateManagerCertificatesCertificatesEl {
        DataCertificateManagerCertificatesCertificatesEl {
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
            managed: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
            san_dnsnames: core::default::Default::default(),
            scope: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
        }
    }
}
pub struct DataCertificateManagerCertificatesCertificatesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCertificateManagerCertificatesCertificatesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCertificateManagerCertificatesCertificatesElRef {
        DataCertificateManagerCertificatesCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCertificateManagerCertificatesCertificatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `managed` after provisioning.\n"]
    pub fn managed(&self) -> ListRef<DataCertificateManagerCertificatesCertificatesElManagedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.managed", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `san_dnsnames` after provisioning.\n"]
    pub fn san_dnsnames(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.san_dnsnames", self.base))
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.base),
        )
    }
}
