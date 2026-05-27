use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRegionCommitmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_renew: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    existing_reservations: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    plan: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_resource: Option<Vec<ComputeRegionCommitmentLicenseResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ComputeRegionCommitmentParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Vec<ComputeRegionCommitmentResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRegionCommitmentTimeoutsEl>,
    dynamic: ComputeRegionCommitmentDynamic,
}
struct ComputeRegionCommitment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRegionCommitmentData>,
}
#[derive(Clone)]
pub struct ComputeRegionCommitment(Rc<ComputeRegionCommitment_>);
impl ComputeRegionCommitment {
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
    #[doc = "Set the field `auto_renew`.\nSpecifies whether to enable automatic renewal for the commitment.\nThe default value is false if not specified.\nIf the field is set to true, the commitment will be automatically renewed for either\none or three years according to the terms of the existing commitment."]
    pub fn set_auto_renew(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_renew = Some(v.into());
        self
    }
    #[doc = "Set the field `category`.\nThe category of the commitment. Category MACHINE specifies commitments composed of\nmachine resources such as VCPU or MEMORY, listed in resources. Category LICENSE\nspecifies commitments composed of software licenses, listed in licenseResources.\nNote that only MACHINE commitments should have a Type specified. Possible values: [\"LICENSE\", \"MACHINE\"]"]
    pub fn set_category(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().category = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `existing_reservations`.\nSpecifies the already existing reservations to attach to the Commitment."]
    pub fn set_existing_reservations(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().existing_reservations = Some(v.into());
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
    #[doc = "Set the field `region`.\nURL of the region where this commitment may be used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe type of commitment, which affects the discount rate and the eligible resources.\nThe type could be one of the following value: 'MEMORY_OPTIMIZED', 'ACCELERATOR_OPTIMIZED',\n'GENERAL_PURPOSE', 'GENERAL_PURPOSE_N2', 'GENERAL_PURPOSE_N2D', 'GENERAL_PURPOSE_E2',\n'GENERAL_PURPOSE_T2D', 'GENERAL_PURPOSE_C3', 'COMPUTE_OPTIMIZED_C2', 'COMPUTE_OPTIMIZED_C2D' and\n'GRAPHICS_OPTIMIZED_G2'"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `license_resource`.\n"]
    pub fn set_license_resource(
        self,
        v: impl Into<BlockAssignable<ComputeRegionCommitmentLicenseResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().license_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.license_resource = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        self,
        v: impl Into<BlockAssignable<ComputeRegionCommitmentParamsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        self,
        v: impl Into<BlockAssignable<ComputeRegionCommitmentResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRegionCommitmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `auto_renew` after provisioning.\nSpecifies whether to enable automatic renewal for the commitment.\nThe default value is false if not specified.\nIf the field is set to true, the commitment will be automatically renewed for either\none or three years according to the terms of the existing commitment."]
    pub fn auto_renew(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_renew", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `category` after provisioning.\nThe category of the commitment. Category MACHINE specifies commitments composed of\nmachine resources such as VCPU or MEMORY, listed in resources. Category LICENSE\nspecifies commitments composed of software licenses, listed in licenseResources.\nNote that only MACHINE commitments should have a Type specified. Possible values: [\"LICENSE\", \"MACHINE\"]"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.category", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `commitment_id` after provisioning.\nUnique identifier for the resource."]
    pub fn commitment_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commitment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `end_timestamp` after provisioning.\nCommitment end time in RFC3339 text format."]
    pub fn end_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `existing_reservations` after provisioning.\nSpecifies the already existing reservations to attach to the Commitment."]
    pub fn existing_reservations(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.existing_reservations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plan` after provisioning.\nThe plan for this commitment, which determines duration and discount rate.\nThe currently supported plans are TWELVE_MONTH (1 year), and THIRTY_SIX_MONTH (3 years). Possible values: [\"TWELVE_MONTH\", \"THIRTY_SIX_MONTH\"]"]
    pub fn plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plan", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nURL of the region where this commitment may be used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_timestamp` after provisioning.\nCommitment start time in RFC3339 text format."]
    pub fn start_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the commitment with regards to eventual expiration\n(each commitment has an end date defined)."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status_message` after provisioning.\nA human-readable explanation of the status."]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of commitment, which affects the discount rate and the eligible resources.\nThe type could be one of the following value: 'MEMORY_OPTIMIZED', 'ACCELERATOR_OPTIMIZED',\n'GENERAL_PURPOSE', 'GENERAL_PURPOSE_N2', 'GENERAL_PURPOSE_N2D', 'GENERAL_PURPOSE_E2',\n'GENERAL_PURPOSE_T2D', 'GENERAL_PURPOSE_C3', 'COMPUTE_OPTIMIZED_C2', 'COMPUTE_OPTIMIZED_C2D' and\n'GRAPHICS_OPTIMIZED_G2'"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `license_resource` after provisioning.\n"]
    pub fn license_resource(&self) -> ListRef<ComputeRegionCommitmentLicenseResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.license_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeRegionCommitmentParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<ComputeRegionCommitmentResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionCommitmentTimeoutsElRef {
        ComputeRegionCommitmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRegionCommitment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRegionCommitment {}
impl ToListMappable for ComputeRegionCommitment {
    type O = ListRef<ComputeRegionCommitmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRegionCommitment_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_region_commitment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRegionCommitment {
    pub tf_id: String,
    #[doc = "Name of the resource. The name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "The plan for this commitment, which determines duration and discount rate.\nThe currently supported plans are TWELVE_MONTH (1 year), and THIRTY_SIX_MONTH (3 years). Possible values: [\"TWELVE_MONTH\", \"THIRTY_SIX_MONTH\"]"]
    pub plan: PrimField<String>,
}
impl BuildComputeRegionCommitment {
    pub fn build(self, stack: &mut Stack) -> ComputeRegionCommitment {
        let out = ComputeRegionCommitment(Rc::new(ComputeRegionCommitment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRegionCommitmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_renew: core::default::Default::default(),
                category: core::default::Default::default(),
                description: core::default::Default::default(),
                existing_reservations: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                plan: self.plan,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: core::default::Default::default(),
                license_resource: core::default::Default::default(),
                params: core::default::Default::default(),
                resources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRegionCommitmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionCommitmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRegionCommitmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_renew` after provisioning.\nSpecifies whether to enable automatic renewal for the commitment.\nThe default value is false if not specified.\nIf the field is set to true, the commitment will be automatically renewed for either\none or three years according to the terms of the existing commitment."]
    pub fn auto_renew(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_renew", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `category` after provisioning.\nThe category of the commitment. Category MACHINE specifies commitments composed of\nmachine resources such as VCPU or MEMORY, listed in resources. Category LICENSE\nspecifies commitments composed of software licenses, listed in licenseResources.\nNote that only MACHINE commitments should have a Type specified. Possible values: [\"LICENSE\", \"MACHINE\"]"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.category", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `commitment_id` after provisioning.\nUnique identifier for the resource."]
    pub fn commitment_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commitment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `end_timestamp` after provisioning.\nCommitment end time in RFC3339 text format."]
    pub fn end_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `existing_reservations` after provisioning.\nSpecifies the already existing reservations to attach to the Commitment."]
    pub fn existing_reservations(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.existing_reservations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plan` after provisioning.\nThe plan for this commitment, which determines duration and discount rate.\nThe currently supported plans are TWELVE_MONTH (1 year), and THIRTY_SIX_MONTH (3 years). Possible values: [\"TWELVE_MONTH\", \"THIRTY_SIX_MONTH\"]"]
    pub fn plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plan", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nURL of the region where this commitment may be used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_timestamp` after provisioning.\nCommitment start time in RFC3339 text format."]
    pub fn start_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the commitment with regards to eventual expiration\n(each commitment has an end date defined)."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status_message` after provisioning.\nA human-readable explanation of the status."]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of commitment, which affects the discount rate and the eligible resources.\nThe type could be one of the following value: 'MEMORY_OPTIMIZED', 'ACCELERATOR_OPTIMIZED',\n'GENERAL_PURPOSE', 'GENERAL_PURPOSE_N2', 'GENERAL_PURPOSE_N2D', 'GENERAL_PURPOSE_E2',\n'GENERAL_PURPOSE_T2D', 'GENERAL_PURPOSE_C3', 'COMPUTE_OPTIMIZED_C2', 'COMPUTE_OPTIMIZED_C2D' and\n'GRAPHICS_OPTIMIZED_G2'"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `license_resource` after provisioning.\n"]
    pub fn license_resource(&self) -> ListRef<ComputeRegionCommitmentLicenseResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.license_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeRegionCommitmentParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<ComputeRegionCommitmentResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionCommitmentTimeoutsElRef {
        ComputeRegionCommitmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionCommitmentLicenseResourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cores_per_license: Option<PrimField<String>>,
    license: PrimField<String>,
}
impl ComputeRegionCommitmentLicenseResourceEl {
    #[doc = "Set the field `amount`.\nThe number of licenses purchased."]
    pub fn set_amount(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amount = Some(v.into());
        self
    }
    #[doc = "Set the field `cores_per_license`.\nSpecifies the core range of the instance for which this license applies."]
    pub fn set_cores_per_license(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cores_per_license = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionCommitmentLicenseResourceEl {
    type O = BlockAssignable<ComputeRegionCommitmentLicenseResourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionCommitmentLicenseResourceEl {
    #[doc = "Any applicable license URI."]
    pub license: PrimField<String>,
}
impl BuildComputeRegionCommitmentLicenseResourceEl {
    pub fn build(self) -> ComputeRegionCommitmentLicenseResourceEl {
        ComputeRegionCommitmentLicenseResourceEl {
            amount: core::default::Default::default(),
            cores_per_license: core::default::Default::default(),
            license: self.license,
        }
    }
}
pub struct ComputeRegionCommitmentLicenseResourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionCommitmentLicenseResourceElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionCommitmentLicenseResourceElRef {
        ComputeRegionCommitmentLicenseResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionCommitmentLicenseResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `amount` after provisioning.\nThe number of licenses purchased."]
    pub fn amount(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }
    #[doc = "Get a reference to the value of field `cores_per_license` after provisioning.\nSpecifies the core range of the instance for which this license applies."]
    pub fn cores_per_license(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cores_per_license", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `license` after provisioning.\nAny applicable license URI."]
    pub fn license(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionCommitmentParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl ComputeRegionCommitmentParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the commitment. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionCommitmentParamsEl {
    type O = BlockAssignable<ComputeRegionCommitmentParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionCommitmentParamsEl {}
impl BuildComputeRegionCommitmentParamsEl {
    pub fn build(self) -> ComputeRegionCommitmentParamsEl {
        ComputeRegionCommitmentParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionCommitmentParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionCommitmentParamsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionCommitmentParamsElRef {
        ComputeRegionCommitmentParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionCommitmentParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the commitment. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionCommitmentResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl ComputeRegionCommitmentResourcesEl {
    #[doc = "Set the field `accelerator_type`.\nName of the accelerator type resource. Applicable only when the type is ACCELERATOR."]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }
    #[doc = "Set the field `amount`.\nThe amount of the resource purchased (in a type-dependent unit,\nsuch as bytes). For vCPUs, this can just be an integer. For memory,\nthis must be provided in MB. Memory must be a multiple of 256 MB,\nwith up to 6.5GB of memory per every vCPU."]
    pub fn set_amount(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amount = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nType of resource for which this commitment applies.\nPossible values are VCPU, MEMORY, LOCAL_SSD, and ACCELERATOR."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionCommitmentResourcesEl {
    type O = BlockAssignable<ComputeRegionCommitmentResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionCommitmentResourcesEl {}
impl BuildComputeRegionCommitmentResourcesEl {
    pub fn build(self) -> ComputeRegionCommitmentResourcesEl {
        ComputeRegionCommitmentResourcesEl {
            accelerator_type: core::default::Default::default(),
            amount: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionCommitmentResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionCommitmentResourcesElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionCommitmentResourcesElRef {
        ComputeRegionCommitmentResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionCommitmentResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nName of the accelerator type resource. Applicable only when the type is ACCELERATOR."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `amount` after provisioning.\nThe amount of the resource purchased (in a type-dependent unit,\nsuch as bytes). For vCPUs, this can just be an integer. For memory,\nthis must be provided in MB. Memory must be a multiple of 256 MB,\nwith up to 6.5GB of memory per every vCPU."]
    pub fn amount(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of resource for which this commitment applies.\nPossible values are VCPU, MEMORY, LOCAL_SSD, and ACCELERATOR."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionCommitmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ComputeRegionCommitmentTimeoutsEl {
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
impl ToListMappable for ComputeRegionCommitmentTimeoutsEl {
    type O = BlockAssignable<ComputeRegionCommitmentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionCommitmentTimeoutsEl {}
impl BuildComputeRegionCommitmentTimeoutsEl {
    pub fn build(self) -> ComputeRegionCommitmentTimeoutsEl {
        ComputeRegionCommitmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionCommitmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionCommitmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionCommitmentTimeoutsElRef {
        ComputeRegionCommitmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionCommitmentTimeoutsElRef {
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
struct ComputeRegionCommitmentDynamic {
    license_resource: Option<DynamicBlock<ComputeRegionCommitmentLicenseResourceEl>>,
    params: Option<DynamicBlock<ComputeRegionCommitmentParamsEl>>,
    resources: Option<DynamicBlock<ComputeRegionCommitmentResourcesEl>>,
}
