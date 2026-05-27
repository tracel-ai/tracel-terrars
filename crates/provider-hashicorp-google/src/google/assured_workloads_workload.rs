use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AssuredWorkloadsWorkloadData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_account: Option<PrimField<String>>,
    compliance_regime: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_sovereign_controls: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    organization: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partner_services_billing_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_resources_parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    violation_notifications_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_settings: Option<Vec<AssuredWorkloadsWorkloadKmsSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partner_permissions: Option<Vec<AssuredWorkloadsWorkloadPartnerPermissionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_settings: Option<Vec<AssuredWorkloadsWorkloadResourceSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AssuredWorkloadsWorkloadTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_options: Option<Vec<AssuredWorkloadsWorkloadWorkloadOptionsEl>>,
    dynamic: AssuredWorkloadsWorkloadDynamic,
}
struct AssuredWorkloadsWorkload_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AssuredWorkloadsWorkloadData>,
}
#[derive(Clone)]
pub struct AssuredWorkloadsWorkload(Rc<AssuredWorkloadsWorkload_>);
impl AssuredWorkloadsWorkload {
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
    #[doc = "Set the field `billing_account`.\nOptional. Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."]
    pub fn set_billing_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_account = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_sovereign_controls`.\nOptional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers."]
    pub fn set_enable_sovereign_controls(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_sovereign_controls = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Labels applied to the workload.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `partner`.\nOptional. Partner regime associated with this workload. Possible values: PARTNER_UNSPECIFIED, LOCAL_CONTROLS_BY_S3NS, SOVEREIGN_CONTROLS_BY_T_SYSTEMS, SOVEREIGN_CONTROLS_BY_SIA_MINSAIT, SOVEREIGN_CONTROLS_BY_PSN, SOVEREIGN_CONTROLS_BY_CNTXT, SOVEREIGN_CONTROLS_BY_CNTXT_NO_EKM, SPAIN_DATA_BOUNDARY_BY_TELEFONICA"]
    pub fn set_partner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().partner = Some(v.into());
        self
    }
    #[doc = "Set the field `partner_services_billing_account`.\nOptional. Input only. Billing account necessary for purchasing services from Sovereign Partners. This field is required for creating SIA/PSN/CNTXT partner workloads. The caller should have 'billing.resourceAssociations.create' IAM permission on this billing-account. The format of this string is billingAccounts/AAAAAA-BBBBBB-CCCCCC."]
    pub fn set_partner_services_billing_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().partner_services_billing_account = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_resources_parent`.\nInput only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}"]
    pub fn set_provisioned_resources_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().provisioned_resources_parent = Some(v.into());
        self
    }
    #[doc = "Set the field `violation_notifications_enabled`.\nOptional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload."]
    pub fn set_violation_notifications_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().violation_notifications_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_settings`.\n"]
    pub fn set_kms_settings(
        self,
        v: impl Into<BlockAssignable<AssuredWorkloadsWorkloadKmsSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kms_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kms_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `partner_permissions`.\n"]
    pub fn set_partner_permissions(
        self,
        v: impl Into<BlockAssignable<AssuredWorkloadsWorkloadPartnerPermissionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().partner_permissions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.partner_permissions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_settings`.\n"]
    pub fn set_resource_settings(
        self,
        v: impl Into<BlockAssignable<AssuredWorkloadsWorkloadResourceSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AssuredWorkloadsWorkloadTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `workload_options`.\n"]
    pub fn set_workload_options(
        self,
        v: impl Into<BlockAssignable<AssuredWorkloadsWorkloadWorkloadOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workload_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workload_options = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `billing_account` after provisioning.\nOptional. Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."]
    pub fn billing_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compliance_regime` after provisioning.\nRequired. Immutable. Compliance Regime associated with this workload. Possible values: COMPLIANCE_REGIME_UNSPECIFIED, IL4, CJIS, FEDRAMP_HIGH, FEDRAMP_MODERATE, US_REGIONAL_ACCESS, HIPAA, HITRUST, EU_REGIONS_AND_SUPPORT, CA_REGIONS_AND_SUPPORT, ITAR, AU_REGIONS_AND_US_SUPPORT, ASSURED_WORKLOADS_FOR_PARTNERS, ISR_REGIONS, ISR_REGIONS_AND_SUPPORT, CA_PROTECTED_B, IL5, IL2, JP_REGIONS_AND_SUPPORT, KSA_REGIONS_AND_SUPPORT_WITH_SOVEREIGNTY_CONTROLS, REGIONAL_CONTROLS, HEALTHCARE_AND_LIFE_SCIENCES_CONTROLS, HEALTHCARE_AND_LIFE_SCIENCES_CONTROLS_US_SUPPORT, IRS_1075"]
    pub fn compliance_regime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compliance_regime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compliance_status` after provisioning.\nOutput only. Count of active Violations in the Workload."]
    pub fn compliance_status(&self) -> ListRef<AssuredWorkloadsWorkloadComplianceStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compliance_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compliant_but_disallowed_services` after provisioning.\nOutput only. Urls for services which are compliant for this Assured Workload, but which are currently disallowed by the ResourceUsageRestriction org policy. Invoke workloads.restrictAllowedResources endpoint to allow your project developers to use these services in their environment."]
    pub fn compliant_but_disallowed_services(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compliant_but_disallowed_services", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Immutable. The Workload creation timestamp."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ekm_provisioning_response` after provisioning.\nOptional. Represents the Ekm Provisioning State of the given workload."]
    pub fn ekm_provisioning_response(
        &self,
    ) -> ListRef<AssuredWorkloadsWorkloadEkmProvisioningResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ekm_provisioning_response", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_sovereign_controls` after provisioning.\nOptional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers."]
    pub fn enable_sovereign_controls(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_sovereign_controls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kaj_enrollment_state` after provisioning.\nOutput only. Represents the KAJ enrollment state of the given workload. Possible values: KAJ_ENROLLMENT_STATE_UNSPECIFIED, KAJ_ENROLLMENT_STATE_PENDING, KAJ_ENROLLMENT_STATE_COMPLETE"]
    pub fn kaj_enrollment_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kaj_enrollment_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels applied to the workload.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the workload."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe organization for the resource"]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partner` after provisioning.\nOptional. Partner regime associated with this workload. Possible values: PARTNER_UNSPECIFIED, LOCAL_CONTROLS_BY_S3NS, SOVEREIGN_CONTROLS_BY_T_SYSTEMS, SOVEREIGN_CONTROLS_BY_SIA_MINSAIT, SOVEREIGN_CONTROLS_BY_PSN, SOVEREIGN_CONTROLS_BY_CNTXT, SOVEREIGN_CONTROLS_BY_CNTXT_NO_EKM, SPAIN_DATA_BOUNDARY_BY_TELEFONICA"]
    pub fn partner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partner_services_billing_account` after provisioning.\nOptional. Input only. Billing account necessary for purchasing services from Sovereign Partners. This field is required for creating SIA/PSN/CNTXT partner workloads. The caller should have 'billing.resourceAssociations.create' IAM permission on this billing-account. The format of this string is billingAccounts/AAAAAA-BBBBBB-CCCCCC."]
    pub fn partner_services_billing_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partner_services_billing_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_resources_parent` after provisioning.\nInput only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}"]
    pub fn provisioned_resources_parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_resources_parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nOutput only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only."]
    pub fn resources(&self) -> ListRef<AssuredWorkloadsWorkloadResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `saa_enrollment_response` after provisioning.\nOutput only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during workloads.get call. In failure cases, user friendly error message is shown in SAA details page."]
    pub fn saa_enrollment_response(
        &self,
    ) -> ListRef<AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saa_enrollment_response", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `violation_notifications_enabled` after provisioning.\nOptional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload."]
    pub fn violation_notifications_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.violation_notifications_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_settings` after provisioning.\n"]
    pub fn kms_settings(&self) -> ListRef<AssuredWorkloadsWorkloadKmsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kms_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partner_permissions` after provisioning.\n"]
    pub fn partner_permissions(&self) -> ListRef<AssuredWorkloadsWorkloadPartnerPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partner_permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_settings` after provisioning.\n"]
    pub fn resource_settings(&self) -> ListRef<AssuredWorkloadsWorkloadResourceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AssuredWorkloadsWorkloadTimeoutsElRef {
        AssuredWorkloadsWorkloadTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_options` after provisioning.\n"]
    pub fn workload_options(&self) -> ListRef<AssuredWorkloadsWorkloadWorkloadOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_options", self.extract_ref()),
        )
    }
}
impl Referable for AssuredWorkloadsWorkload {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AssuredWorkloadsWorkload {}
impl ToListMappable for AssuredWorkloadsWorkload {
    type O = ListRef<AssuredWorkloadsWorkloadRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AssuredWorkloadsWorkload_ {
    fn extract_resource_type(&self) -> String {
        "google_assured_workloads_workload".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAssuredWorkloadsWorkload {
    pub tf_id: String,
    #[doc = "Required. Immutable. Compliance Regime associated with this workload. Possible values: COMPLIANCE_REGIME_UNSPECIFIED, IL4, CJIS, FEDRAMP_HIGH, FEDRAMP_MODERATE, US_REGIONAL_ACCESS, HIPAA, HITRUST, EU_REGIONS_AND_SUPPORT, CA_REGIONS_AND_SUPPORT, ITAR, AU_REGIONS_AND_US_SUPPORT, ASSURED_WORKLOADS_FOR_PARTNERS, ISR_REGIONS, ISR_REGIONS_AND_SUPPORT, CA_PROTECTED_B, IL5, IL2, JP_REGIONS_AND_SUPPORT, KSA_REGIONS_AND_SUPPORT_WITH_SOVEREIGNTY_CONTROLS, REGIONAL_CONTROLS, HEALTHCARE_AND_LIFE_SCIENCES_CONTROLS, HEALTHCARE_AND_LIFE_SCIENCES_CONTROLS_US_SUPPORT, IRS_1075"]
    pub compliance_regime: PrimField<String>,
    #[doc = "Required. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload"]
    pub display_name: PrimField<String>,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "The organization for the resource"]
    pub organization: PrimField<String>,
}
impl BuildAssuredWorkloadsWorkload {
    pub fn build(self, stack: &mut Stack) -> AssuredWorkloadsWorkload {
        let out = AssuredWorkloadsWorkload(Rc::new(AssuredWorkloadsWorkload_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AssuredWorkloadsWorkloadData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                billing_account: core::default::Default::default(),
                compliance_regime: self.compliance_regime,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                enable_sovereign_controls: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                organization: self.organization,
                partner: core::default::Default::default(),
                partner_services_billing_account: core::default::Default::default(),
                provisioned_resources_parent: core::default::Default::default(),
                violation_notifications_enabled: core::default::Default::default(),
                kms_settings: core::default::Default::default(),
                partner_permissions: core::default::Default::default(),
                resource_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workload_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AssuredWorkloadsWorkloadRef {
    shared: StackShared,
    base: String,
}
impl Ref for AssuredWorkloadsWorkloadRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AssuredWorkloadsWorkloadRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `billing_account` after provisioning.\nOptional. Input only. The billing account used for the resources which are direct children of workload. This billing account is initially associated with the resources created as part of Workload creation. After the initial creation of these resources, the customer can change the assigned billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF`."]
    pub fn billing_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compliance_regime` after provisioning.\nRequired. Immutable. Compliance Regime associated with this workload. Possible values: COMPLIANCE_REGIME_UNSPECIFIED, IL4, CJIS, FEDRAMP_HIGH, FEDRAMP_MODERATE, US_REGIONAL_ACCESS, HIPAA, HITRUST, EU_REGIONS_AND_SUPPORT, CA_REGIONS_AND_SUPPORT, ITAR, AU_REGIONS_AND_US_SUPPORT, ASSURED_WORKLOADS_FOR_PARTNERS, ISR_REGIONS, ISR_REGIONS_AND_SUPPORT, CA_PROTECTED_B, IL5, IL2, JP_REGIONS_AND_SUPPORT, KSA_REGIONS_AND_SUPPORT_WITH_SOVEREIGNTY_CONTROLS, REGIONAL_CONTROLS, HEALTHCARE_AND_LIFE_SCIENCES_CONTROLS, HEALTHCARE_AND_LIFE_SCIENCES_CONTROLS_US_SUPPORT, IRS_1075"]
    pub fn compliance_regime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compliance_regime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compliance_status` after provisioning.\nOutput only. Count of active Violations in the Workload."]
    pub fn compliance_status(&self) -> ListRef<AssuredWorkloadsWorkloadComplianceStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compliance_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compliant_but_disallowed_services` after provisioning.\nOutput only. Urls for services which are compliant for this Assured Workload, but which are currently disallowed by the ResourceUsageRestriction org policy. Invoke workloads.restrictAllowedResources endpoint to allow your project developers to use these services in their environment."]
    pub fn compliant_but_disallowed_services(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compliant_but_disallowed_services", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Immutable. The Workload creation timestamp."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The user-assigned display name of the Workload. When present it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, and spaces. Example: My Workload"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ekm_provisioning_response` after provisioning.\nOptional. Represents the Ekm Provisioning State of the given workload."]
    pub fn ekm_provisioning_response(
        &self,
    ) -> ListRef<AssuredWorkloadsWorkloadEkmProvisioningResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ekm_provisioning_response", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_sovereign_controls` after provisioning.\nOptional. Indicates the sovereignty status of the given workload. Currently meant to be used by Europe/Canada customers."]
    pub fn enable_sovereign_controls(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_sovereign_controls", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kaj_enrollment_state` after provisioning.\nOutput only. Represents the KAJ enrollment state of the given workload. Possible values: KAJ_ENROLLMENT_STATE_UNSPECIFIED, KAJ_ENROLLMENT_STATE_PENDING, KAJ_ENROLLMENT_STATE_COMPLETE"]
    pub fn kaj_enrollment_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kaj_enrollment_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels applied to the workload.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the workload."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe organization for the resource"]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partner` after provisioning.\nOptional. Partner regime associated with this workload. Possible values: PARTNER_UNSPECIFIED, LOCAL_CONTROLS_BY_S3NS, SOVEREIGN_CONTROLS_BY_T_SYSTEMS, SOVEREIGN_CONTROLS_BY_SIA_MINSAIT, SOVEREIGN_CONTROLS_BY_PSN, SOVEREIGN_CONTROLS_BY_CNTXT, SOVEREIGN_CONTROLS_BY_CNTXT_NO_EKM, SPAIN_DATA_BOUNDARY_BY_TELEFONICA"]
    pub fn partner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partner_services_billing_account` after provisioning.\nOptional. Input only. Billing account necessary for purchasing services from Sovereign Partners. This field is required for creating SIA/PSN/CNTXT partner workloads. The caller should have 'billing.resourceAssociations.create' IAM permission on this billing-account. The format of this string is billingAccounts/AAAAAA-BBBBBB-CCCCCC."]
    pub fn partner_services_billing_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partner_services_billing_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_resources_parent` after provisioning.\nInput only. The parent resource for the resources managed by this Assured Workload. May be either empty or a folder resource which is a child of the Workload parent. If not specified all resources are created under the parent organization. Format: folders/{folder_id}"]
    pub fn provisioned_resources_parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_resources_parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nOutput only. The resources associated with this workload. These resources will be created when creating the workload. If any of the projects already exist, the workload creation will fail. Always read only."]
    pub fn resources(&self) -> ListRef<AssuredWorkloadsWorkloadResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `saa_enrollment_response` after provisioning.\nOutput only. Represents the SAA enrollment response of the given workload. SAA enrollment response is queried during workloads.get call. In failure cases, user friendly error message is shown in SAA details page."]
    pub fn saa_enrollment_response(
        &self,
    ) -> ListRef<AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saa_enrollment_response", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `violation_notifications_enabled` after provisioning.\nOptional. Indicates whether the e-mail notification for a violation is enabled for a workload. This value will be by default True, and if not present will be considered as true. This should only be updated via updateWorkload call. Any Changes to this field during the createWorkload call will not be honored. This will always be true while creating the workload."]
    pub fn violation_notifications_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.violation_notifications_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_settings` after provisioning.\n"]
    pub fn kms_settings(&self) -> ListRef<AssuredWorkloadsWorkloadKmsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kms_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partner_permissions` after provisioning.\n"]
    pub fn partner_permissions(&self) -> ListRef<AssuredWorkloadsWorkloadPartnerPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partner_permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_settings` after provisioning.\n"]
    pub fn resource_settings(&self) -> ListRef<AssuredWorkloadsWorkloadResourceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AssuredWorkloadsWorkloadTimeoutsElRef {
        AssuredWorkloadsWorkloadTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_options` after provisioning.\n"]
    pub fn workload_options(&self) -> ListRef<AssuredWorkloadsWorkloadWorkloadOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_options", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadComplianceStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acknowledged_violation_count: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_violation_count: Option<ListField<PrimField<f64>>>,
}
impl AssuredWorkloadsWorkloadComplianceStatusEl {
    #[doc = "Set the field `acknowledged_violation_count`.\n"]
    pub fn set_acknowledged_violation_count(
        mut self,
        v: impl Into<ListField<PrimField<f64>>>,
    ) -> Self {
        self.acknowledged_violation_count = Some(v.into());
        self
    }
    #[doc = "Set the field `active_violation_count`.\n"]
    pub fn set_active_violation_count(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.active_violation_count = Some(v.into());
        self
    }
}
impl ToListMappable for AssuredWorkloadsWorkloadComplianceStatusEl {
    type O = BlockAssignable<AssuredWorkloadsWorkloadComplianceStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAssuredWorkloadsWorkloadComplianceStatusEl {}
impl BuildAssuredWorkloadsWorkloadComplianceStatusEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadComplianceStatusEl {
        AssuredWorkloadsWorkloadComplianceStatusEl {
            acknowledged_violation_count: core::default::Default::default(),
            active_violation_count: core::default::Default::default(),
        }
    }
}
pub struct AssuredWorkloadsWorkloadComplianceStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AssuredWorkloadsWorkloadComplianceStatusElRef {
    fn new(shared: StackShared, base: String) -> AssuredWorkloadsWorkloadComplianceStatusElRef {
        AssuredWorkloadsWorkloadComplianceStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AssuredWorkloadsWorkloadComplianceStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `acknowledged_violation_count` after provisioning.\n"]
    pub fn acknowledged_violation_count(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.acknowledged_violation_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `active_violation_count` after provisioning.\n"]
    pub fn active_violation_count(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.active_violation_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadEkmProvisioningResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ekm_provisioning_error_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ekm_provisioning_error_mapping: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ekm_provisioning_state: Option<PrimField<String>>,
}
impl AssuredWorkloadsWorkloadEkmProvisioningResponseEl {
    #[doc = "Set the field `ekm_provisioning_error_domain`.\n"]
    pub fn set_ekm_provisioning_error_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ekm_provisioning_error_domain = Some(v.into());
        self
    }
    #[doc = "Set the field `ekm_provisioning_error_mapping`.\n"]
    pub fn set_ekm_provisioning_error_mapping(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ekm_provisioning_error_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `ekm_provisioning_state`.\n"]
    pub fn set_ekm_provisioning_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ekm_provisioning_state = Some(v.into());
        self
    }
}
impl ToListMappable for AssuredWorkloadsWorkloadEkmProvisioningResponseEl {
    type O = BlockAssignable<AssuredWorkloadsWorkloadEkmProvisioningResponseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAssuredWorkloadsWorkloadEkmProvisioningResponseEl {}
impl BuildAssuredWorkloadsWorkloadEkmProvisioningResponseEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadEkmProvisioningResponseEl {
        AssuredWorkloadsWorkloadEkmProvisioningResponseEl {
            ekm_provisioning_error_domain: core::default::Default::default(),
            ekm_provisioning_error_mapping: core::default::Default::default(),
            ekm_provisioning_state: core::default::Default::default(),
        }
    }
}
pub struct AssuredWorkloadsWorkloadEkmProvisioningResponseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AssuredWorkloadsWorkloadEkmProvisioningResponseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AssuredWorkloadsWorkloadEkmProvisioningResponseElRef {
        AssuredWorkloadsWorkloadEkmProvisioningResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AssuredWorkloadsWorkloadEkmProvisioningResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ekm_provisioning_error_domain` after provisioning.\n"]
    pub fn ekm_provisioning_error_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ekm_provisioning_error_domain", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ekm_provisioning_error_mapping` after provisioning.\n"]
    pub fn ekm_provisioning_error_mapping(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ekm_provisioning_error_mapping", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ekm_provisioning_state` after provisioning.\n"]
    pub fn ekm_provisioning_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ekm_provisioning_state", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
}
impl AssuredWorkloadsWorkloadResourcesEl {
    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.resource_id = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }
}
impl ToListMappable for AssuredWorkloadsWorkloadResourcesEl {
    type O = BlockAssignable<AssuredWorkloadsWorkloadResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAssuredWorkloadsWorkloadResourcesEl {}
impl BuildAssuredWorkloadsWorkloadResourcesEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadResourcesEl {
        AssuredWorkloadsWorkloadResourcesEl {
            resource_id: core::default::Default::default(),
            resource_type: core::default::Default::default(),
        }
    }
}
pub struct AssuredWorkloadsWorkloadResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AssuredWorkloadsWorkloadResourcesElRef {
    fn new(shared: StackShared, base: String) -> AssuredWorkloadsWorkloadResourcesElRef {
        AssuredWorkloadsWorkloadResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AssuredWorkloadsWorkloadResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    setup_errors: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    setup_status: Option<PrimField<String>>,
}
impl AssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
    #[doc = "Set the field `setup_errors`.\n"]
    pub fn set_setup_errors(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.setup_errors = Some(v.into());
        self
    }
    #[doc = "Set the field `setup_status`.\n"]
    pub fn set_setup_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.setup_status = Some(v.into());
        self
    }
}
impl ToListMappable for AssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
    type O = BlockAssignable<AssuredWorkloadsWorkloadSaaEnrollmentResponseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAssuredWorkloadsWorkloadSaaEnrollmentResponseEl {}
impl BuildAssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
        AssuredWorkloadsWorkloadSaaEnrollmentResponseEl {
            setup_errors: core::default::Default::default(),
            setup_status: core::default::Default::default(),
        }
    }
}
pub struct AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef {
        AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AssuredWorkloadsWorkloadSaaEnrollmentResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `setup_errors` after provisioning.\n"]
    pub fn setup_errors(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.setup_errors", self.base))
    }
    #[doc = "Get a reference to the value of field `setup_status` after provisioning.\n"]
    pub fn setup_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.setup_status", self.base))
    }
}
#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadKmsSettingsEl {
    next_rotation_time: PrimField<String>,
    rotation_period: PrimField<String>,
}
impl AssuredWorkloadsWorkloadKmsSettingsEl {}
impl ToListMappable for AssuredWorkloadsWorkloadKmsSettingsEl {
    type O = BlockAssignable<AssuredWorkloadsWorkloadKmsSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAssuredWorkloadsWorkloadKmsSettingsEl {
    #[doc = "Required. Input only. Immutable. The time at which the Key Management Service will automatically create a new version of the crypto key and mark it as the primary."]
    pub next_rotation_time: PrimField<String>,
    #[doc = "Required. Input only. Immutable. will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours."]
    pub rotation_period: PrimField<String>,
}
impl BuildAssuredWorkloadsWorkloadKmsSettingsEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadKmsSettingsEl {
        AssuredWorkloadsWorkloadKmsSettingsEl {
            next_rotation_time: self.next_rotation_time,
            rotation_period: self.rotation_period,
        }
    }
}
pub struct AssuredWorkloadsWorkloadKmsSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AssuredWorkloadsWorkloadKmsSettingsElRef {
    fn new(shared: StackShared, base: String) -> AssuredWorkloadsWorkloadKmsSettingsElRef {
        AssuredWorkloadsWorkloadKmsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AssuredWorkloadsWorkloadKmsSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `next_rotation_time` after provisioning.\nRequired. Input only. Immutable. The time at which the Key Management Service will automatically create a new version of the crypto key and mark it as the primary."]
    pub fn next_rotation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_rotation_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rotation_period` after provisioning.\nRequired. Input only. Immutable. will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours."]
    pub fn rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_period", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadPartnerPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assured_workloads_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_logs_viewer: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_access_approver: Option<PrimField<bool>>,
}
impl AssuredWorkloadsWorkloadPartnerPermissionsEl {
    #[doc = "Set the field `assured_workloads_monitoring`.\nOptional. Allow partner to view violation alerts."]
    pub fn set_assured_workloads_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.assured_workloads_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `data_logs_viewer`.\nAllow the partner to view inspectability logs and monitoring violations."]
    pub fn set_data_logs_viewer(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.data_logs_viewer = Some(v.into());
        self
    }
    #[doc = "Set the field `service_access_approver`.\nOptional. Allow partner to view access approval logs."]
    pub fn set_service_access_approver(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.service_access_approver = Some(v.into());
        self
    }
}
impl ToListMappable for AssuredWorkloadsWorkloadPartnerPermissionsEl {
    type O = BlockAssignable<AssuredWorkloadsWorkloadPartnerPermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAssuredWorkloadsWorkloadPartnerPermissionsEl {}
impl BuildAssuredWorkloadsWorkloadPartnerPermissionsEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadPartnerPermissionsEl {
        AssuredWorkloadsWorkloadPartnerPermissionsEl {
            assured_workloads_monitoring: core::default::Default::default(),
            data_logs_viewer: core::default::Default::default(),
            service_access_approver: core::default::Default::default(),
        }
    }
}
pub struct AssuredWorkloadsWorkloadPartnerPermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AssuredWorkloadsWorkloadPartnerPermissionsElRef {
    fn new(shared: StackShared, base: String) -> AssuredWorkloadsWorkloadPartnerPermissionsElRef {
        AssuredWorkloadsWorkloadPartnerPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AssuredWorkloadsWorkloadPartnerPermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `assured_workloads_monitoring` after provisioning.\nOptional. Allow partner to view violation alerts."]
    pub fn assured_workloads_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assured_workloads_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_logs_viewer` after provisioning.\nAllow the partner to view inspectability logs and monitoring violations."]
    pub fn data_logs_viewer(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_logs_viewer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_access_approver` after provisioning.\nOptional. Allow partner to view access approval logs."]
    pub fn service_access_approver(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_access_approver", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadResourceSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
}
impl AssuredWorkloadsWorkloadResourceSettingsEl {
    #[doc = "Set the field `display_name`.\nUser-assigned resource display name. If not empty it will be used to create a resource with the specified name."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_id`.\nResource identifier. For a project this represents projectId. If the project is already taken, the workload creation will fail. For KeyRing, this represents the keyring_id. For a folder, don't set this value as folder_id is assigned by Google."]
    pub fn set_resource_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_id = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_type`.\nIndicates the type of resource. This field should be specified to correspond the id to the right project type (CONSUMER_PROJECT or ENCRYPTION_KEYS_PROJECT) Possible values: RESOURCE_TYPE_UNSPECIFIED, CONSUMER_PROJECT, ENCRYPTION_KEYS_PROJECT, KEYRING, CONSUMER_FOLDER"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }
}
impl ToListMappable for AssuredWorkloadsWorkloadResourceSettingsEl {
    type O = BlockAssignable<AssuredWorkloadsWorkloadResourceSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAssuredWorkloadsWorkloadResourceSettingsEl {}
impl BuildAssuredWorkloadsWorkloadResourceSettingsEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadResourceSettingsEl {
        AssuredWorkloadsWorkloadResourceSettingsEl {
            display_name: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_type: core::default::Default::default(),
        }
    }
}
pub struct AssuredWorkloadsWorkloadResourceSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AssuredWorkloadsWorkloadResourceSettingsElRef {
    fn new(shared: StackShared, base: String) -> AssuredWorkloadsWorkloadResourceSettingsElRef {
        AssuredWorkloadsWorkloadResourceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AssuredWorkloadsWorkloadResourceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-assigned resource display name. If not empty it will be used to create a resource with the specified name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\nResource identifier. For a project this represents projectId. If the project is already taken, the workload creation will fail. For KeyRing, this represents the keyring_id. For a folder, don't set this value as folder_id is assigned by Google."]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nIndicates the type of resource. This field should be specified to correspond the id to the right project type (CONSUMER_PROJECT or ENCRYPTION_KEYS_PROJECT) Possible values: RESOURCE_TYPE_UNSPECIFIED, CONSUMER_PROJECT, ENCRYPTION_KEYS_PROJECT, KEYRING, CONSUMER_FOLDER"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AssuredWorkloadsWorkloadTimeoutsEl {
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
impl ToListMappable for AssuredWorkloadsWorkloadTimeoutsEl {
    type O = BlockAssignable<AssuredWorkloadsWorkloadTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAssuredWorkloadsWorkloadTimeoutsEl {}
impl BuildAssuredWorkloadsWorkloadTimeoutsEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadTimeoutsEl {
        AssuredWorkloadsWorkloadTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AssuredWorkloadsWorkloadTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AssuredWorkloadsWorkloadTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AssuredWorkloadsWorkloadTimeoutsElRef {
        AssuredWorkloadsWorkloadTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AssuredWorkloadsWorkloadTimeoutsElRef {
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
#[derive(Serialize)]
pub struct AssuredWorkloadsWorkloadWorkloadOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kaj_enrollment_type: Option<PrimField<String>>,
}
impl AssuredWorkloadsWorkloadWorkloadOptionsEl {
    #[doc = "Set the field `kaj_enrollment_type`.\nIndicates type of KAJ enrollment for the workload. Currently, only specifiying KEY_ACCESS_TRANSPARENCY_OFF is implemented to not enroll in KAT-level KAJ enrollment for Regional Controls workloads. Possible values: KAJ_ENROLLMENT_TYPE_UNSPECIFIED, FULL_KAJ, EKM_ONLY, KEY_ACCESS_TRANSPARENCY_OFF"]
    pub fn set_kaj_enrollment_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kaj_enrollment_type = Some(v.into());
        self
    }
}
impl ToListMappable for AssuredWorkloadsWorkloadWorkloadOptionsEl {
    type O = BlockAssignable<AssuredWorkloadsWorkloadWorkloadOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAssuredWorkloadsWorkloadWorkloadOptionsEl {}
impl BuildAssuredWorkloadsWorkloadWorkloadOptionsEl {
    pub fn build(self) -> AssuredWorkloadsWorkloadWorkloadOptionsEl {
        AssuredWorkloadsWorkloadWorkloadOptionsEl {
            kaj_enrollment_type: core::default::Default::default(),
        }
    }
}
pub struct AssuredWorkloadsWorkloadWorkloadOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AssuredWorkloadsWorkloadWorkloadOptionsElRef {
    fn new(shared: StackShared, base: String) -> AssuredWorkloadsWorkloadWorkloadOptionsElRef {
        AssuredWorkloadsWorkloadWorkloadOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AssuredWorkloadsWorkloadWorkloadOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kaj_enrollment_type` after provisioning.\nIndicates type of KAJ enrollment for the workload. Currently, only specifiying KEY_ACCESS_TRANSPARENCY_OFF is implemented to not enroll in KAT-level KAJ enrollment for Regional Controls workloads. Possible values: KAJ_ENROLLMENT_TYPE_UNSPECIFIED, FULL_KAJ, EKM_ONLY, KEY_ACCESS_TRANSPARENCY_OFF"]
    pub fn kaj_enrollment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kaj_enrollment_type", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AssuredWorkloadsWorkloadDynamic {
    kms_settings: Option<DynamicBlock<AssuredWorkloadsWorkloadKmsSettingsEl>>,
    partner_permissions: Option<DynamicBlock<AssuredWorkloadsWorkloadPartnerPermissionsEl>>,
    resource_settings: Option<DynamicBlock<AssuredWorkloadsWorkloadResourceSettingsEl>>,
    workload_options: Option<DynamicBlock<AssuredWorkloadsWorkloadWorkloadOptionsEl>>,
}
