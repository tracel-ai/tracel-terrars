use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeOrganizationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_consumer_data_encryption_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_consumer_data_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorized_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_encryption_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_vpc_peering: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_database_encryption_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<ApigeeOrganizationPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeOrganizationTimeoutsEl>,
    dynamic: ApigeeOrganizationDynamic,
}
struct ApigeeOrganization_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeOrganizationData>,
}
#[derive(Clone)]
pub struct ApigeeOrganization(Rc<ApigeeOrganization_>);
impl ApigeeOrganization {
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
    #[doc = "Set the field `analytics_region`.\nPrimary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org)."]
    pub fn set_analytics_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().analytics_region = Some(v.into());
        self
    }
    #[doc = "Set the field `api_consumer_data_encryption_key_name`.\nCloud KMS key name used for encrypting API consumer data."]
    pub fn set_api_consumer_data_encryption_key_name(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .api_consumer_data_encryption_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `api_consumer_data_location`.\nThis field is needed only for customers using non-default data residency regions.\nApigee stores some control plane data only in single region.\nThis field determines which single region Apigee should use."]
    pub fn set_api_consumer_data_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_consumer_data_location = Some(v.into());
        self
    }
    #[doc = "Set the field `authorized_network`.\nCompute Engine network used for Service Networking to be peered with Apigee runtime instances.\nSee [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started).\nValid only when 'RuntimeType' is set to CLOUD. The value can be updated only when there are no runtime instances. For example: \"default\"."]
    pub fn set_authorized_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authorized_network = Some(v.into());
        self
    }
    #[doc = "Set the field `billing_type`.\nBilling type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing)."]
    pub fn set_billing_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_type = Some(v.into());
        self
    }
    #[doc = "Set the field `control_plane_encryption_key_name`.\nCloud KMS key name used for encrypting control plane data that is stored in a multi region.\nOnly used for the data residency region \"US\" or \"EU\"."]
    pub fn set_control_plane_encryption_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().control_plane_encryption_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nDescription of the Apigee organization."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_vpc_peering`.\nFlag that specifies whether the VPC Peering through Private Google Access should be\ndisabled between the consumer network and Apigee. Required if an 'authorizedNetwork'\non the consumer project is not provided, in which case the flag should be set to 'true'.\nValid only when 'RuntimeType' is set to CLOUD. The value must be set before the creation\nof any Apigee runtime instance and can be updated only when there are no runtime instances."]
    pub fn set_disable_vpc_peering(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_vpc_peering = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the Apigee organization."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `retention`.\nOptional. This setting is applicable only for organizations that are soft-deleted (i.e., BillingType\nis not EVALUATION). It controls how long Organization data will be retained after the initial delete\noperation completes. During this period, the Organization may be restored to its last known state.\nAfter this period, the Organization will no longer be able to be restored. Default value: \"DELETION_RETENTION_UNSPECIFIED\" Possible values: [\"DELETION_RETENTION_UNSPECIFIED\", \"MINIMUM\"]"]
    pub fn set_retention(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().retention = Some(v.into());
        self
    }
    #[doc = "Set the field `runtime_database_encryption_key_name`.\nCloud KMS key name used for encrypting the data that is stored and replicated across runtime instances.\nUpdate is not allowed after the organization is created.\nIf not specified, a Google-Managed encryption key will be used.\nValid only when 'RuntimeType' is CLOUD. For example: 'projects/foo/locations/us/keyRings/bar/cryptoKeys/baz'."]
    pub fn set_runtime_database_encryption_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .runtime_database_encryption_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `runtime_type`.\nRuntime type of the Apigee organization based on the Apigee subscription purchased. Default value: \"CLOUD\" Possible values: [\"CLOUD\", \"HYBRID\"]"]
    pub fn set_runtime_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().runtime_type = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        self,
        v: impl Into<BlockAssignable<ApigeeOrganizationPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeOrganizationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `analytics_region` after provisioning.\nPrimary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org)."]
    pub fn analytics_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.analytics_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `api_consumer_data_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting API consumer data."]
    pub fn api_consumer_data_encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.api_consumer_data_encryption_key_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `api_consumer_data_location` after provisioning.\nThis field is needed only for customers using non-default data residency regions.\nApigee stores some control plane data only in single region.\nThis field determines which single region Apigee should use."]
    pub fn api_consumer_data_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_consumer_data_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `apigee_project_id` after provisioning.\nOutput only. Project ID of the Apigee Tenant Project."]
    pub fn apigee_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.apigee_project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorized_network` after provisioning.\nCompute Engine network used for Service Networking to be peered with Apigee runtime instances.\nSee [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started).\nValid only when 'RuntimeType' is set to CLOUD. The value can be updated only when there are no runtime instances. For example: \"default\"."]
    pub fn authorized_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorized_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_type` after provisioning.\nBilling type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing)."]
    pub fn billing_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\nOutput only. Base64-encoded public certificate for the root CA of the Apigee organization.\nValid only when 'RuntimeType' is CLOUD. A base64-encoded string."]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting control plane data that is stored in a multi region.\nOnly used for the data residency region \"US\" or \"EU\"."]
    pub fn control_plane_encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_encryption_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the Apigee organization."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_vpc_peering` after provisioning.\nFlag that specifies whether the VPC Peering through Private Google Access should be\ndisabled between the consumer network and Apigee. Required if an 'authorizedNetwork'\non the consumer project is not provided, in which case the flag should be set to 'true'.\nValid only when 'RuntimeType' is set to CLOUD. The value must be set before the creation\nof any Apigee runtime instance and can be updated only when there are no runtime instances."]
    pub fn disable_vpc_peering(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_vpc_peering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Apigee organization."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the Apigee organization."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe project ID associated with the Apigee organization."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention` after provisioning.\nOptional. This setting is applicable only for organizations that are soft-deleted (i.e., BillingType\nis not EVALUATION). It controls how long Organization data will be retained after the initial delete\noperation completes. During this period, the Organization may be restored to its last known state.\nAfter this period, the Organization will no longer be able to be restored. Default value: \"DELETION_RETENTION_UNSPECIFIED\" Possible values: [\"DELETION_RETENTION_UNSPECIFIED\", \"MINIMUM\"]"]
    pub fn retention(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_database_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting the data that is stored and replicated across runtime instances.\nUpdate is not allowed after the organization is created.\nIf not specified, a Google-Managed encryption key will be used.\nValid only when 'RuntimeType' is CLOUD. For example: 'projects/foo/locations/us/keyRings/bar/cryptoKeys/baz'."]
    pub fn runtime_database_encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.runtime_database_encryption_key_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_type` after provisioning.\nRuntime type of the Apigee organization based on the Apigee subscription purchased. Default value: \"CLOUD\" Possible values: [\"CLOUD\", \"HYBRID\"]"]
    pub fn runtime_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_type` after provisioning.\nOutput only. Subscription type of the Apigee organization.\nValid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased)."]
    pub fn subscription_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> ListRef<ApigeeOrganizationPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeOrganizationTimeoutsElRef {
        ApigeeOrganizationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeOrganization {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeOrganization {}
impl ToListMappable for ApigeeOrganization {
    type O = ListRef<ApigeeOrganizationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeOrganization_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_organization".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeOrganization {
    pub tf_id: String,
    #[doc = "The project ID associated with the Apigee organization."]
    pub project_id: PrimField<String>,
}
impl BuildApigeeOrganization {
    pub fn build(self, stack: &mut Stack) -> ApigeeOrganization {
        let out = ApigeeOrganization(Rc::new(ApigeeOrganization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeOrganizationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                analytics_region: core::default::Default::default(),
                api_consumer_data_encryption_key_name: core::default::Default::default(),
                api_consumer_data_location: core::default::Default::default(),
                authorized_network: core::default::Default::default(),
                billing_type: core::default::Default::default(),
                control_plane_encryption_key_name: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_vpc_peering: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                project_id: self.project_id,
                retention: core::default::Default::default(),
                runtime_database_encryption_key_name: core::default::Default::default(),
                runtime_type: core::default::Default::default(),
                properties: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeOrganizationRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeOrganizationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeOrganizationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `analytics_region` after provisioning.\nPrimary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org)."]
    pub fn analytics_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.analytics_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `api_consumer_data_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting API consumer data."]
    pub fn api_consumer_data_encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.api_consumer_data_encryption_key_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `api_consumer_data_location` after provisioning.\nThis field is needed only for customers using non-default data residency regions.\nApigee stores some control plane data only in single region.\nThis field determines which single region Apigee should use."]
    pub fn api_consumer_data_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_consumer_data_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `apigee_project_id` after provisioning.\nOutput only. Project ID of the Apigee Tenant Project."]
    pub fn apigee_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.apigee_project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorized_network` after provisioning.\nCompute Engine network used for Service Networking to be peered with Apigee runtime instances.\nSee [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started).\nValid only when 'RuntimeType' is set to CLOUD. The value can be updated only when there are no runtime instances. For example: \"default\"."]
    pub fn authorized_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorized_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_type` after provisioning.\nBilling type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing)."]
    pub fn billing_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\nOutput only. Base64-encoded public certificate for the root CA of the Apigee organization.\nValid only when 'RuntimeType' is CLOUD. A base64-encoded string."]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting control plane data that is stored in a multi region.\nOnly used for the data residency region \"US\" or \"EU\"."]
    pub fn control_plane_encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_encryption_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the Apigee organization."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_vpc_peering` after provisioning.\nFlag that specifies whether the VPC Peering through Private Google Access should be\ndisabled between the consumer network and Apigee. Required if an 'authorizedNetwork'\non the consumer project is not provided, in which case the flag should be set to 'true'.\nValid only when 'RuntimeType' is set to CLOUD. The value must be set before the creation\nof any Apigee runtime instance and can be updated only when there are no runtime instances."]
    pub fn disable_vpc_peering(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_vpc_peering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Apigee organization."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the Apigee organization."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe project ID associated with the Apigee organization."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention` after provisioning.\nOptional. This setting is applicable only for organizations that are soft-deleted (i.e., BillingType\nis not EVALUATION). It controls how long Organization data will be retained after the initial delete\noperation completes. During this period, the Organization may be restored to its last known state.\nAfter this period, the Organization will no longer be able to be restored. Default value: \"DELETION_RETENTION_UNSPECIFIED\" Possible values: [\"DELETION_RETENTION_UNSPECIFIED\", \"MINIMUM\"]"]
    pub fn retention(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_database_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting the data that is stored and replicated across runtime instances.\nUpdate is not allowed after the organization is created.\nIf not specified, a Google-Managed encryption key will be used.\nValid only when 'RuntimeType' is CLOUD. For example: 'projects/foo/locations/us/keyRings/bar/cryptoKeys/baz'."]
    pub fn runtime_database_encryption_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.runtime_database_encryption_key_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_type` after provisioning.\nRuntime type of the Apigee organization based on the Apigee subscription purchased. Default value: \"CLOUD\" Possible values: [\"CLOUD\", \"HYBRID\"]"]
    pub fn runtime_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_type` after provisioning.\nOutput only. Subscription type of the Apigee organization.\nValid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased)."]
    pub fn subscription_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> ListRef<ApigeeOrganizationPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeOrganizationTimeoutsElRef {
        ApigeeOrganizationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeOrganizationPropertiesElPropertyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ApigeeOrganizationPropertiesElPropertyEl {
    #[doc = "Set the field `name`.\nName of the property."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nValue of the property."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeOrganizationPropertiesElPropertyEl {
    type O = BlockAssignable<ApigeeOrganizationPropertiesElPropertyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeOrganizationPropertiesElPropertyEl {}
impl BuildApigeeOrganizationPropertiesElPropertyEl {
    pub fn build(self) -> ApigeeOrganizationPropertiesElPropertyEl {
        ApigeeOrganizationPropertiesElPropertyEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ApigeeOrganizationPropertiesElPropertyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeOrganizationPropertiesElPropertyElRef {
    fn new(shared: StackShared, base: String) -> ApigeeOrganizationPropertiesElPropertyElRef {
        ApigeeOrganizationPropertiesElPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeOrganizationPropertiesElPropertyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the property."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the property."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApigeeOrganizationPropertiesElDynamic {
    property: Option<DynamicBlock<ApigeeOrganizationPropertiesElPropertyEl>>,
}
#[derive(Serialize)]
pub struct ApigeeOrganizationPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    property: Option<Vec<ApigeeOrganizationPropertiesElPropertyEl>>,
    dynamic: ApigeeOrganizationPropertiesElDynamic,
}
impl ApigeeOrganizationPropertiesEl {
    #[doc = "Set the field `property`.\n"]
    pub fn set_property(
        mut self,
        v: impl Into<BlockAssignable<ApigeeOrganizationPropertiesElPropertyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.property = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.property = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeOrganizationPropertiesEl {
    type O = BlockAssignable<ApigeeOrganizationPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeOrganizationPropertiesEl {}
impl BuildApigeeOrganizationPropertiesEl {
    pub fn build(self) -> ApigeeOrganizationPropertiesEl {
        ApigeeOrganizationPropertiesEl {
            property: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeOrganizationPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeOrganizationPropertiesElRef {
    fn new(shared: StackShared, base: String) -> ApigeeOrganizationPropertiesElRef {
        ApigeeOrganizationPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeOrganizationPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `property` after provisioning.\n"]
    pub fn property(&self) -> ListRef<ApigeeOrganizationPropertiesElPropertyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.property", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeOrganizationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApigeeOrganizationTimeoutsEl {
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
impl ToListMappable for ApigeeOrganizationTimeoutsEl {
    type O = BlockAssignable<ApigeeOrganizationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeOrganizationTimeoutsEl {}
impl BuildApigeeOrganizationTimeoutsEl {
    pub fn build(self) -> ApigeeOrganizationTimeoutsEl {
        ApigeeOrganizationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApigeeOrganizationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeOrganizationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeOrganizationTimeoutsElRef {
        ApigeeOrganizationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeOrganizationTimeoutsElRef {
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
struct ApigeeOrganizationDynamic {
    properties: Option<DynamicBlock<ApigeeOrganizationPropertiesEl>>,
}
