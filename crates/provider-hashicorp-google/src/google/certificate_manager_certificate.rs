use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CertificateManagerCertificateData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed: Option<Vec<CertificateManagerCertificateManagedEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_managed: Option<Vec<CertificateManagerCertificateSelfManagedEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CertificateManagerCertificateTimeoutsEl>,
    dynamic: CertificateManagerCertificateDynamic,
}
struct CertificateManagerCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CertificateManagerCertificateData>,
}
#[derive(Clone)]
pub struct CertificateManagerCertificate(Rc<CertificateManagerCertificate_>);
impl CertificateManagerCertificate {
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
    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of label tags associated with the Certificate resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\nThe scope of the certificate.\n\nDEFAULT: Certificates with default scope are served from core Google data centers.\nIf unsure, choose this option.\n\nEDGE_CACHE: Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence.\nSee https://cloud.google.com/vpc/docs/edge-locations.\n\nALL_REGIONS: Certificates with ALL_REGIONS scope are served from all GCP regions (You can only use ALL_REGIONS with global certs).\nSee https://cloud.google.com/compute/docs/regions-zones.\n\nCLIENT_AUTH: Certificates with CLIENT_AUTH scope are used by a load balancer (TLS client) to be presented to the backend (TLS server) when backend mTLS is configured.\nSee https://cloud.google.com/load-balancing/docs/backend-authenticated-tls-backend-mtls#client-certificate."]
    pub fn set_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scope = Some(v.into());
        self
    }
    #[doc = "Set the field `managed`.\n"]
    pub fn set_managed(
        self,
        v: impl Into<BlockAssignable<CertificateManagerCertificateManagedEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `self_managed`.\n"]
    pub fn set_self_managed(
        self,
        v: impl Into<BlockAssignable<CertificateManagerCertificateSelfManagedEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().self_managed = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.self_managed = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CertificateManagerCertificateTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the Certificate resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the certificate. Certificate names must be unique\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `san_dnsnames` after provisioning.\nThe list of Subject Alternative Names of dnsName type defined in the certificate (see RFC 5280 4.2.1.6)"]
    pub fn san_dnsnames(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.san_dnsnames", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nThe scope of the certificate.\n\nDEFAULT: Certificates with default scope are served from core Google data centers.\nIf unsure, choose this option.\n\nEDGE_CACHE: Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence.\nSee https://cloud.google.com/vpc/docs/edge-locations.\n\nALL_REGIONS: Certificates with ALL_REGIONS scope are served from all GCP regions (You can only use ALL_REGIONS with global certs).\nSee https://cloud.google.com/compute/docs/regions-zones.\n\nCLIENT_AUTH: Certificates with CLIENT_AUTH scope are used by a load balancer (TLS client) to be presented to the backend (TLS server) when backend mTLS is configured.\nSee https://cloud.google.com/load-balancing/docs/backend-authenticated-tls-backend-mtls#client-certificate."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed` after provisioning.\n"]
    pub fn managed(&self) -> ListRef<CertificateManagerCertificateManagedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_managed` after provisioning.\n"]
    pub fn self_managed(&self) -> ListRef<CertificateManagerCertificateSelfManagedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerCertificateTimeoutsElRef {
        CertificateManagerCertificateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CertificateManagerCertificate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CertificateManagerCertificate {}
impl ToListMappable for CertificateManagerCertificate {
    type O = ListRef<CertificateManagerCertificateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CertificateManagerCertificate_ {
    fn extract_resource_type(&self) -> String {
        "google_certificate_manager_certificate".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCertificateManagerCertificate {
    pub tf_id: String,
    #[doc = "A user-defined name of the certificate. Certificate names must be unique\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub name: PrimField<String>,
}
impl BuildCertificateManagerCertificate {
    pub fn build(self, stack: &mut Stack) -> CertificateManagerCertificate {
        let out = CertificateManagerCertificate(Rc::new(CertificateManagerCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CertificateManagerCertificateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                scope: core::default::Default::default(),
                managed: core::default::Default::default(),
                self_managed: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CertificateManagerCertificateRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CertificateManagerCertificateRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the Certificate resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the certificate. Certificate names must be unique\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `san_dnsnames` after provisioning.\nThe list of Subject Alternative Names of dnsName type defined in the certificate (see RFC 5280 4.2.1.6)"]
    pub fn san_dnsnames(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.san_dnsnames", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nThe scope of the certificate.\n\nDEFAULT: Certificates with default scope are served from core Google data centers.\nIf unsure, choose this option.\n\nEDGE_CACHE: Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence.\nSee https://cloud.google.com/vpc/docs/edge-locations.\n\nALL_REGIONS: Certificates with ALL_REGIONS scope are served from all GCP regions (You can only use ALL_REGIONS with global certs).\nSee https://cloud.google.com/compute/docs/regions-zones.\n\nCLIENT_AUTH: Certificates with CLIENT_AUTH scope are used by a load balancer (TLS client) to be presented to the backend (TLS server) when backend mTLS is configured.\nSee https://cloud.google.com/load-balancing/docs/backend-authenticated-tls-backend-mtls#client-certificate."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed` after provisioning.\n"]
    pub fn managed(&self) -> ListRef<CertificateManagerCertificateManagedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_managed` after provisioning.\n"]
    pub fn self_managed(&self) -> ListRef<CertificateManagerCertificateSelfManagedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CertificateManagerCertificateTimeoutsElRef {
        CertificateManagerCertificateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
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
impl ToListMappable for CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
    type O = BlockAssignable<CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {}
impl BuildCertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
    pub fn build(self) -> CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
        CertificateManagerCertificateManagedElAuthorizationAttemptInfoEl {
            details: core::default::Default::default(),
            domain: core::default::Default::default(),
            failure_reason: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef {
        CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef {
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
pub struct CertificateManagerCertificateManagedElProvisioningIssueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
}
impl CertificateManagerCertificateManagedElProvisioningIssueEl {
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
impl ToListMappable for CertificateManagerCertificateManagedElProvisioningIssueEl {
    type O = BlockAssignable<CertificateManagerCertificateManagedElProvisioningIssueEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCertificateManagerCertificateManagedElProvisioningIssueEl {}
impl BuildCertificateManagerCertificateManagedElProvisioningIssueEl {
    pub fn build(self) -> CertificateManagerCertificateManagedElProvisioningIssueEl {
        CertificateManagerCertificateManagedElProvisioningIssueEl {
            details: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}
pub struct CertificateManagerCertificateManagedElProvisioningIssueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateManagedElProvisioningIssueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CertificateManagerCertificateManagedElProvisioningIssueElRef {
        CertificateManagerCertificateManagedElProvisioningIssueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CertificateManagerCertificateManagedElProvisioningIssueElRef {
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
pub struct CertificateManagerCertificateManagedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_authorizations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domains: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuance_config: Option<PrimField<String>>,
}
impl CertificateManagerCertificateManagedEl {
    #[doc = "Set the field `dns_authorizations`.\nAuthorizations that will be used for performing domain authorization. Either issuanceConfig or dnsAuthorizations should be specificed, but not both."]
    pub fn set_dns_authorizations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dns_authorizations = Some(v.into());
        self
    }
    #[doc = "Set the field `domains`.\nThe domains for which a managed SSL certificate will be generated.\nWildcard domains are only supported with DNS challenge resolution"]
    pub fn set_domains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.domains = Some(v.into());
        self
    }
    #[doc = "Set the field `issuance_config`.\nThe resource name for a CertificateIssuanceConfig used to configure private PKI certificates in the format projects/*/locations/*/certificateIssuanceConfigs/*.\nIf this field is not set, the certificates will instead be publicly signed as documented at https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs#caa.\nEither issuanceConfig or dnsAuthorizations should be specificed, but not both."]
    pub fn set_issuance_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuance_config = Some(v.into());
        self
    }
}
impl ToListMappable for CertificateManagerCertificateManagedEl {
    type O = BlockAssignable<CertificateManagerCertificateManagedEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCertificateManagerCertificateManagedEl {}
impl BuildCertificateManagerCertificateManagedEl {
    pub fn build(self) -> CertificateManagerCertificateManagedEl {
        CertificateManagerCertificateManagedEl {
            dns_authorizations: core::default::Default::default(),
            domains: core::default::Default::default(),
            issuance_config: core::default::Default::default(),
        }
    }
}
pub struct CertificateManagerCertificateManagedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateManagedElRef {
    fn new(shared: StackShared, base: String) -> CertificateManagerCertificateManagedElRef {
        CertificateManagerCertificateManagedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CertificateManagerCertificateManagedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorization_attempt_info` after provisioning.\nDetailed state of the latest authorization attempt for each domain\nspecified for this Managed Certificate."]
    pub fn authorization_attempt_info(
        &self,
    ) -> ListRef<CertificateManagerCertificateManagedElAuthorizationAttemptInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization_attempt_info", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dns_authorizations` after provisioning.\nAuthorizations that will be used for performing domain authorization. Either issuanceConfig or dnsAuthorizations should be specificed, but not both."]
    pub fn dns_authorizations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_authorizations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `domains` after provisioning.\nThe domains for which a managed SSL certificate will be generated.\nWildcard domains are only supported with DNS challenge resolution"]
    pub fn domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }
    #[doc = "Get a reference to the value of field `issuance_config` after provisioning.\nThe resource name for a CertificateIssuanceConfig used to configure private PKI certificates in the format projects/*/locations/*/certificateIssuanceConfigs/*.\nIf this field is not set, the certificates will instead be publicly signed as documented at https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs#caa.\nEither issuanceConfig or dnsAuthorizations should be specificed, but not both."]
    pub fn issuance_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.issuance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provisioning_issue` after provisioning.\nInformation about issues with provisioning this Managed Certificate."]
    pub fn provisioning_issue(
        &self,
    ) -> ListRef<CertificateManagerCertificateManagedElProvisioningIssueElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioning_issue", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nA state of this Managed Certificate."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct CertificateManagerCertificateSelfManagedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_pem: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pem_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pem_private_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key_pem: Option<PrimField<String>>,
}
impl CertificateManagerCertificateSelfManagedEl {
    #[doc = "Set the field `certificate_pem`.\nThe certificate chain in PEM-encoded form.\n\nLeaf certificate comes first, followed by intermediate ones if any."]
    pub fn set_certificate_pem(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_pem = Some(v.into());
        self
    }
    #[doc = "Set the field `pem_certificate`.\nThe certificate chain in PEM-encoded form.\n\nLeaf certificate comes first, followed by intermediate ones if any."]
    pub fn set_pem_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pem_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `pem_private_key`.\nThe private key of the leaf certificate in PEM-encoded form."]
    pub fn set_pem_private_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pem_private_key = Some(v.into());
        self
    }
    #[doc = "Set the field `private_key_pem`.\nThe private key of the leaf certificate in PEM-encoded form."]
    pub fn set_private_key_pem(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_key_pem = Some(v.into());
        self
    }
}
impl ToListMappable for CertificateManagerCertificateSelfManagedEl {
    type O = BlockAssignable<CertificateManagerCertificateSelfManagedEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCertificateManagerCertificateSelfManagedEl {}
impl BuildCertificateManagerCertificateSelfManagedEl {
    pub fn build(self) -> CertificateManagerCertificateSelfManagedEl {
        CertificateManagerCertificateSelfManagedEl {
            certificate_pem: core::default::Default::default(),
            pem_certificate: core::default::Default::default(),
            pem_private_key: core::default::Default::default(),
            private_key_pem: core::default::Default::default(),
        }
    }
}
pub struct CertificateManagerCertificateSelfManagedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateSelfManagedElRef {
    fn new(shared: StackShared, base: String) -> CertificateManagerCertificateSelfManagedElRef {
        CertificateManagerCertificateSelfManagedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CertificateManagerCertificateSelfManagedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_pem` after provisioning.\nThe certificate chain in PEM-encoded form.\n\nLeaf certificate comes first, followed by intermediate ones if any."]
    pub fn certificate_pem(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_pem", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pem_certificate` after provisioning.\nThe certificate chain in PEM-encoded form.\n\nLeaf certificate comes first, followed by intermediate ones if any."]
    pub fn pem_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pem_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pem_private_key` after provisioning.\nThe private key of the leaf certificate in PEM-encoded form."]
    pub fn pem_private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pem_private_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_key_pem` after provisioning.\nThe private key of the leaf certificate in PEM-encoded form."]
    pub fn private_key_pem(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_key_pem", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CertificateManagerCertificateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CertificateManagerCertificateTimeoutsEl {
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
impl ToListMappable for CertificateManagerCertificateTimeoutsEl {
    type O = BlockAssignable<CertificateManagerCertificateTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCertificateManagerCertificateTimeoutsEl {}
impl BuildCertificateManagerCertificateTimeoutsEl {
    pub fn build(self) -> CertificateManagerCertificateTimeoutsEl {
        CertificateManagerCertificateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CertificateManagerCertificateTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CertificateManagerCertificateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CertificateManagerCertificateTimeoutsElRef {
        CertificateManagerCertificateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CertificateManagerCertificateTimeoutsElRef {
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
struct CertificateManagerCertificateDynamic {
    managed: Option<DynamicBlock<CertificateManagerCertificateManagedEl>>,
    self_managed: Option<DynamicBlock<CertificateManagerCertificateSelfManagedEl>>,
}
