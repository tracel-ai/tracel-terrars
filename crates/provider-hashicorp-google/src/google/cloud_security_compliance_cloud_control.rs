use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudSecurityComplianceCloudControlData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<ListField<PrimField<String>>>,
    cloud_control_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_category: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    organization: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remediation_steps: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_cloud_providers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_spec: Option<Vec<CloudSecurityComplianceCloudControlParameterSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<CloudSecurityComplianceCloudControlRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudSecurityComplianceCloudControlTimeoutsEl>,
    dynamic: CloudSecurityComplianceCloudControlDynamic,
}
struct CloudSecurityComplianceCloudControl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudSecurityComplianceCloudControlData>,
}
#[derive(Clone)]
pub struct CloudSecurityComplianceCloudControl(Rc<CloudSecurityComplianceCloudControl_>);
impl CloudSecurityComplianceCloudControl {
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
    #[doc = "Set the field `categories`.\nThe categories of the cloud control."]
    pub fn set_categories(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().categories = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA description of the cloud control. The maximum length is 2000 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the cloud control. The maximum length is 200\ncharacters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `finding_category`.\nThe finding_category of the cloud control. The maximum length is 255\ncharacters."]
    pub fn set_finding_category(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().finding_category = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `remediation_steps`.\nThe remediation steps for the findings generated by the cloud control. The\nmaximum length is 400 characters."]
    pub fn set_remediation_steps(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().remediation_steps = Some(v.into());
        self
    }
    #[doc = "Set the field `severity`.\nPossible values:\nCRITICAL\nHIGH\nMEDIUM\nLOW"]
    pub fn set_severity(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().severity = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_cloud_providers`.\ncloud providers supported"]
    pub fn set_supported_cloud_providers(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().supported_cloud_providers = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_spec`.\n"]
    pub fn set_parameter_spec(
        self,
        v: impl Into<BlockAssignable<CloudSecurityComplianceCloudControlParameterSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameter_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameter_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        self,
        v: impl Into<BlockAssignable<CloudSecurityComplianceCloudControlRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudSecurityComplianceCloudControlTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `categories` after provisioning.\nThe categories of the cloud control."]
    pub fn categories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.categories", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_control_id` after provisioning.\nID of the CloudControl.\nThis is the last segment of the CloudControl resource name.\nFormat: '^a-zA-Z{0,61}[a-zA-Z0-9]$'."]
    pub fn cloud_control_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_control_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe last updated time of the cloud control.\nThe create_time is used because a new CC is created whenever we update an\nexisting CC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the cloud control. The maximum length is 2000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the cloud control. The maximum length is 200\ncharacters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `finding_category` after provisioning.\nThe finding_category of the cloud control. The maximum length is 255\ncharacters."]
    pub fn finding_category(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.finding_category", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Currently, only \"global\" is supported as a location."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision of the cloud control incremented in ascending order."]
    pub fn major_revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.major_revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the cloud control.\nFormat:\norganizations/{organization}/locations/{location}/cloudControls/{cloud_control_id}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `related_frameworks` after provisioning.\nThe Frameworks that include this CloudControl"]
    pub fn related_frameworks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.related_frameworks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remediation_steps` after provisioning.\nThe remediation steps for the findings generated by the cloud control. The\nmaximum length is 400 characters."]
    pub fn remediation_steps(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remediation_steps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\nPossible values:\nCRITICAL\nHIGH\nMEDIUM\nLOW"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.severity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_cloud_providers` after provisioning.\ncloud providers supported"]
    pub fn supported_cloud_providers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_cloud_providers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_enforcement_modes` after provisioning.\nThe supported enforcement mode of the cloud control. Default is DETECTIVE."]
    pub fn supported_enforcement_modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_enforcement_modes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_target_resource_types` after provisioning.\ntarget resource types supported by the CloudControl."]
    pub fn supported_target_resource_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_target_resource_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameter_spec` after provisioning.\n"]
    pub fn parameter_spec(&self) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameter_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<CloudSecurityComplianceCloudControlRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceCloudControlTimeoutsElRef {
        CloudSecurityComplianceCloudControlTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudSecurityComplianceCloudControl {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudSecurityComplianceCloudControl {}
impl ToListMappable for CloudSecurityComplianceCloudControl {
    type O = ListRef<CloudSecurityComplianceCloudControlRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudSecurityComplianceCloudControl_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_security_compliance_cloud_control".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudSecurityComplianceCloudControl {
    pub tf_id: String,
    #[doc = "ID of the CloudControl.\nThis is the last segment of the CloudControl resource name.\nFormat: '^a-zA-Z{0,61}[a-zA-Z0-9]$'."]
    pub cloud_control_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Currently, only \"global\" is supported as a location."]
    pub location: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub organization: PrimField<String>,
}
impl BuildCloudSecurityComplianceCloudControl {
    pub fn build(self, stack: &mut Stack) -> CloudSecurityComplianceCloudControl {
        let out =
            CloudSecurityComplianceCloudControl(Rc::new(CloudSecurityComplianceCloudControl_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(CloudSecurityComplianceCloudControlData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    categories: core::default::Default::default(),
                    cloud_control_id: self.cloud_control_id,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    finding_category: core::default::Default::default(),
                    id: core::default::Default::default(),
                    location: self.location,
                    organization: self.organization,
                    remediation_steps: core::default::Default::default(),
                    severity: core::default::Default::default(),
                    supported_cloud_providers: core::default::Default::default(),
                    parameter_spec: core::default::Default::default(),
                    rules: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudSecurityComplianceCloudControlRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudSecurityComplianceCloudControlRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `categories` after provisioning.\nThe categories of the cloud control."]
    pub fn categories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.categories", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_control_id` after provisioning.\nID of the CloudControl.\nThis is the last segment of the CloudControl resource name.\nFormat: '^a-zA-Z{0,61}[a-zA-Z0-9]$'."]
    pub fn cloud_control_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_control_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe last updated time of the cloud control.\nThe create_time is used because a new CC is created whenever we update an\nexisting CC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the cloud control. The maximum length is 2000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the cloud control. The maximum length is 200\ncharacters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `finding_category` after provisioning.\nThe finding_category of the cloud control. The maximum length is 255\ncharacters."]
    pub fn finding_category(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.finding_category", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Currently, only \"global\" is supported as a location."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision of the cloud control incremented in ascending order."]
    pub fn major_revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.major_revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the cloud control.\nFormat:\norganizations/{organization}/locations/{location}/cloudControls/{cloud_control_id}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `related_frameworks` after provisioning.\nThe Frameworks that include this CloudControl"]
    pub fn related_frameworks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.related_frameworks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remediation_steps` after provisioning.\nThe remediation steps for the findings generated by the cloud control. The\nmaximum length is 400 characters."]
    pub fn remediation_steps(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remediation_steps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\nPossible values:\nCRITICAL\nHIGH\nMEDIUM\nLOW"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.severity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_cloud_providers` after provisioning.\ncloud providers supported"]
    pub fn supported_cloud_providers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_cloud_providers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_enforcement_modes` after provisioning.\nThe supported enforcement mode of the cloud control. Default is DETECTIVE."]
    pub fn supported_enforcement_modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_enforcement_modes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_target_resource_types` after provisioning.\ntarget resource types supported by the CloudControl."]
    pub fn supported_target_resource_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_target_resource_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameter_spec` after provisioning.\n"]
    pub fn parameter_spec(&self) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameter_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<CloudSecurityComplianceCloudControlRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceCloudControlTimeoutsElRef {
        CloudSecurityComplianceCloudControlTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueEl
{
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueEl { }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueEl
{
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueEl { CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueEl { values : self . values , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueElRef { CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElDynamic { string_list_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueEl { # [serde (skip_serializing_if = "Option::is_none")] bool_value : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] number_value : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] string_value : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] string_list_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueEl {
    #[doc = "Set the field `bool_value`.\nRepresents a boolean value."]
    pub fn set_bool_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bool_value = Some(v.into());
        self
    }
    #[doc = "Set the field `number_value`.\nRepresents a double value."]
    pub fn set_number_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number_value = Some(v.into());
        self
    }
    #[doc = "Set the field `string_value`.\nRepresents a string value."]
    pub fn set_string_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.string_value = Some(v.into());
        self
    }
    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.string_list_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list_value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueEl
{
    type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueEl { CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueEl { bool_value : core :: default :: Default :: default () , number_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElRef { CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElRef { shared : shared , base : base . to_string () , } } }
impl
    CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."]
    pub fn bool_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }
    #[doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."]
    pub fn number_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_value", self.base))
    }
    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."]
    pub fn string_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]    pub fn string_list_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElStringListValueElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.string_list_value", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElDynamic { parameter_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueEl { # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] parameter_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueEl {
    #[doc = "Set the field `name`.\nThe name of the parameter."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_value`.\n"]
    pub fn set_parameter_value(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameter_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter_value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueEl {}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueEl {
        CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueEl {
            name: core::default::Default::default(),
            parameter_value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElRef {
        CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"]    pub fn parameter_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElParameterValueElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameter_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueEl {
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueEl {}
impl ToListMappable
    for CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueEl {
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueEl {
        CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueEl {
            values: self.values,
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueElRef {
        CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElDynamic {
    oneof_value: Option<
        DynamicBlock<CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueEl>,
    >,
    string_list_value: Option<
        DynamicBlock<
            CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bool_value: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oneof_value:
        Option<Vec<CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_list_value: Option<
        Vec<CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueEl>,
    >,
    dynamic: CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElDynamic,
}
impl CloudSecurityComplianceCloudControlParameterSpecElDefaultValueEl {
    #[doc = "Set the field `bool_value`.\nRepresents a boolean value."]
    pub fn set_bool_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bool_value = Some(v.into());
        self
    }
    #[doc = "Set the field `number_value`.\nRepresents a double value."]
    pub fn set_number_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number_value = Some(v.into());
        self
    }
    #[doc = "Set the field `string_value`.\nRepresents a string value."]
    pub fn set_string_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.string_value = Some(v.into());
        self
    }
    #[doc = "Set the field `oneof_value`.\n"]
    pub fn set_oneof_value(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oneof_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oneof_value = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.string_list_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list_value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElDefaultValueEl {
    type O = BlockAssignable<CloudSecurityComplianceCloudControlParameterSpecElDefaultValueEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElDefaultValueEl {}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElDefaultValueEl {
    pub fn build(self) -> CloudSecurityComplianceCloudControlParameterSpecElDefaultValueEl {
        CloudSecurityComplianceCloudControlParameterSpecElDefaultValueEl {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            oneof_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElRef {
        CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."]
    pub fn bool_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }
    #[doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."]
    pub fn number_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_value", self.base))
    }
    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."]
    pub fn string_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
    #[doc = "Get a reference to the value of field `oneof_value` after provisioning.\n"]
    pub fn oneof_value(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElOneofValueElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }
    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]
    pub fn string_list_value(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElStringListValueElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.string_list_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueEl
{
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueEl { }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueEl
{
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueEl { values : self . values , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElDynamic { string_list_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueEl { # [serde (skip_serializing_if = "Option::is_none")] bool_value : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] number_value : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] string_value : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] string_list_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueEl { # [doc = "Set the field `bool_value`.\nRepresents a boolean value."] pub fn set_bool_value (mut self , v : impl Into < PrimField < bool > >) -> Self { self . bool_value = Some (v . into ()) ; self } # [doc = "Set the field `number_value`.\nRepresents a double value."] pub fn set_number_value (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . number_value = Some (v . into ()) ; self } # [doc = "Set the field `string_value`.\nRepresents a string value."] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } # [doc = "Set the field `string_list_value`.\n"] pub fn set_string_list_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . string_list_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . string_list_value = Some (d) ; } } self } }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueEl { bool_value : core :: default :: Default :: default () , number_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."] pub fn bool_value (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bool_value" , self . base)) } # [doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."] pub fn number_value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.number_value" , self . base)) } # [doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } # [doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"] pub fn string_list_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElStringListValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.string_list_value" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElDynamic { parameter_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueEl { # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] parameter_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueEl {
    #[doc = "Set the field `name`.\nThe name of the parameter."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_value`.\n"]
    pub fn set_parameter_value(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameter_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter_value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueEl
{}
impl
    BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueEl
{
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueEl
    {
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueEl { name : core :: default :: Default :: default () , parameter_value : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElRef { shared : shared , base : base . to_string () , } } }
impl
    CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"]    pub fn parameter_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElParameterValueElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameter_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueEl
{
    values: ListField<PrimField<String>>,
}
impl
    CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueEl
{
}
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueEl
{
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueEl { values : self . values , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElDynamic { oneof_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueEl >> , string_list_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueEl { # [serde (skip_serializing_if = "Option::is_none")] bool_value : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] number_value : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] string_value : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oneof_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueEl > > , # [serde (skip_serializing_if = "Option::is_none")] string_list_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueEl {
    #[doc = "Set the field `bool_value`.\nRepresents a boolean value."]
    pub fn set_bool_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bool_value = Some(v.into());
        self
    }
    #[doc = "Set the field `number_value`.\nRepresents a double value."]
    pub fn set_number_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number_value = Some(v.into());
        self
    }
    #[doc = "Set the field `string_value`.\nRepresents a string value."]
    pub fn set_string_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.string_value = Some(v.into());
        self
    }
    #[doc = "Set the field `oneof_value`.\n"]
    pub fn set_oneof_value(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oneof_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oneof_value = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.string_list_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list_value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueEl {}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueEl {
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueEl {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            oneof_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElRef {
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."]
    pub fn bool_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }
    #[doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."]
    pub fn number_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_value", self.base))
    }
    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."]
    pub fn string_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
    #[doc = "Get a reference to the value of field `oneof_value` after provisioning.\n"]    pub fn oneof_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElOneofValueElRef >{
        ListRef::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }
    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]    pub fn string_list_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElStringListValueElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.string_list_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleEl { # [doc = "Set the field `attribute`.\nFully qualified proto attribute path (in dot notation).\nExample: rules[0].cel_expression.resource_types_values"] pub fn set_attribute (mut self , v : impl Into < PrimField < String > >) -> Self { self . attribute = Some (v . into ()) ; self } }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleEl { attribute : core :: default :: Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `attribute` after provisioning.\nFully qualified proto attribute path (in dot notation).\nExample: rules[0].cel_expression.resource_types_values"] pub fn attribute (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.attribute" , self . base)) } }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleEl { # [doc = "Set the field `attribute`.\nFully qualified proto attribute path (e.g., dot notation)"] pub fn set_attribute (mut self , v : impl Into < PrimField < String > >) -> Self { self . attribute = Some (v . into ()) ; self } }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleEl { attribute : core :: default :: Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `attribute` after provisioning.\nFully qualified proto attribute path (e.g., dot notation)"] pub fn attribute (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.attribute" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElDynamic { attribute_substitution_rule : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleEl >> , placeholder_substitution_rule : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesEl { # [serde (skip_serializing_if = "Option::is_none")] attribute_substitution_rule : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleEl > > , # [serde (skip_serializing_if = "Option::is_none")] placeholder_substitution_rule : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesEl {
    #[doc = "Set the field `attribute_substitution_rule`.\n"]
    pub fn set_attribute_substitution_rule(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.attribute_substitution_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.attribute_substitution_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `placeholder_substitution_rule`.\n"]
    pub fn set_placeholder_substitution_rule(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.placeholder_substitution_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.placeholder_substitution_rule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesEl {
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesEl {
            attribute_substitution_rule: core::default::Default::default(),
            placeholder_substitution_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElRef
    {
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attribute_substitution_rule` after provisioning.\n"]    pub fn attribute_substitution_rule (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElAttributeSubstitutionRuleElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.attribute_substitution_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `placeholder_substitution_rule` after provisioning.\n"]    pub fn placeholder_substitution_rule (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElPlaceholderSubstitutionRuleElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.placeholder_substitution_rule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl
{
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl { }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl
{
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl { values : self . values , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElDynamic { string_list_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { # [serde (skip_serializing_if = "Option::is_none")] bool_value : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] number_value : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] string_value : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] string_list_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { # [doc = "Set the field `bool_value`.\nRepresents a boolean value."] pub fn set_bool_value (mut self , v : impl Into < PrimField < bool > >) -> Self { self . bool_value = Some (v . into ()) ; self } # [doc = "Set the field `number_value`.\nRepresents a double value."] pub fn set_number_value (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . number_value = Some (v . into ()) ; self } # [doc = "Set the field `string_value`.\nRepresents a string value."] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } # [doc = "Set the field `string_list_value`.\n"] pub fn set_string_list_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . string_list_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . string_list_value = Some (d) ; } } self } }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { bool_value : core :: default :: Default :: default () , number_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."] pub fn bool_value (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bool_value" , self . base)) } # [doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."] pub fn number_value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.number_value" , self . base)) } # [doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } # [doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"] pub fn string_list_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.string_list_value" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElDynamic { parameter_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueEl { # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] parameter_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueEl { # [doc = "Set the field `name`.\nThe name of the parameter."] pub fn set_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . name = Some (v . into ()) ; self } # [doc = "Set the field `parameter_value`.\n"] pub fn set_parameter_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . parameter_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . parameter_value = Some (d) ; } } self } }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueEl { name : core :: default :: Default :: default () , parameter_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"] pub fn parameter_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.parameter_value" , self . base)) } }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueEl
{
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueEl { }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueEl
{
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueEl { values : self . values , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElDynamic { oneof_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueEl >> , string_list_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesEl { # [serde (skip_serializing_if = "Option::is_none")] bool_value : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] number_value : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] string_value : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oneof_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueEl > > , # [serde (skip_serializing_if = "Option::is_none")] string_list_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesEl { # [doc = "Set the field `bool_value`.\nRepresents a boolean value."] pub fn set_bool_value (mut self , v : impl Into < PrimField < bool > >) -> Self { self . bool_value = Some (v . into ()) ; self } # [doc = "Set the field `number_value`.\nRepresents a double value."] pub fn set_number_value (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . number_value = Some (v . into ()) ; self } # [doc = "Set the field `string_value`.\nRepresents a string value."] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } # [doc = "Set the field `oneof_value`.\n"] pub fn set_oneof_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . oneof_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . oneof_value = Some (d) ; } } self } # [doc = "Set the field `string_list_value`.\n"] pub fn set_string_list_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . string_list_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . string_list_value = Some (d) ; } } self } }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesEl { bool_value : core :: default :: Default :: default () , number_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , oneof_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."] pub fn bool_value (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bool_value" , self . base)) } # [doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."] pub fn number_value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.number_value" , self . base)) } # [doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } # [doc = "Get a reference to the value of field `oneof_value` after provisioning.\n"] pub fn oneof_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElOneofValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.oneof_value" , self . base)) } # [doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"] pub fn string_list_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElStringListValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.string_list_value" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElDynamic { values : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesEl { # [serde (skip_serializing_if = "Option::is_none")] values : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesEl {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesEl >>,
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
    for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesEl
{
    type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesEl { values : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElRef { shared : shared , base : base . to_string () , } } }
impl
    CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]    pub fn values (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElValuesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeEl {
    max: PrimField<String>,
    min: PrimField<String>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeEl {}
impl ToListMappable
    for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeEl
{
    #[doc = "Maximum allowed value for the numeric parameter (inclusive)."]
    pub max: PrimField<String>,
    #[doc = "Minimum allowed value for the numeric parameter (inclusive)."]
    pub min: PrimField<String>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeEl
    {
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeEl {
            max: self.max,
            min: self.min,
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeElRef
    {
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max` after provisioning.\nMaximum allowed value for the numeric parameter (inclusive)."]
    pub fn max(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }
    #[doc = "Get a reference to the value of field `min` after provisioning.\nMinimum allowed value for the numeric parameter (inclusive)."]
    pub fn min(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternEl
{
    pattern: PrimField<String>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternEl {}
impl ToListMappable
    for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternEl
{
    type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternEl
{
    #[doc = "Regex Pattern to match the value(s) of parameter."]
    pub pattern: PrimField<String>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternEl { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternEl { pattern : self . pattern , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternElRef { CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternElRef { shared : shared , base : base . to_string () , } } }
impl
    CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pattern` after provisioning.\nRegex Pattern to match the value(s) of parameter."]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElDynamic { allowed_values : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesEl >> , int_range : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeEl >> , regexp_pattern : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationEl { # [serde (skip_serializing_if = "Option::is_none")] allowed_values : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesEl > > , # [serde (skip_serializing_if = "Option::is_none")] int_range : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeEl > > , # [serde (skip_serializing_if = "Option::is_none")] regexp_pattern : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationEl {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowed_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_values = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `int_range`.\n"]
    pub fn set_int_range(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.int_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.int_range = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `regexp_pattern`.\n"]
    pub fn set_regexp_pattern(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.regexp_pattern = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.regexp_pattern = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationEl {}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationEl {
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationEl {
            allowed_values: core::default::Default::default(),
            int_range: core::default::Default::default(),
            regexp_pattern: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRef {
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\n"]    pub fn allowed_values (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElAllowedValuesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `int_range` after provisioning.\n"]
    pub fn int_range(
        &self,
    ) -> ListRef<
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElIntRangeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.int_range", self.base))
    }
    #[doc = "Get a reference to the value of field `regexp_pattern` after provisioning.\n"]    pub fn regexp_pattern (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRegexpPatternElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.regexp_pattern", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDynamic {
    default_value: Option<
        DynamicBlock<
            CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueEl,
        >,
    >,
    substitution_rules: Option<
        DynamicBlock<
            CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesEl,
        >,
    >,
    validation: Option<
        DynamicBlock<CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    is_required: PrimField<bool>,
    name: PrimField<String>,
    value_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<
        Vec<CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    substitution_rules: Option<
        Vec<CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation:
        Option<Vec<CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationEl>>,
    dynamic: CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDynamic,
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersEl {
    #[doc = "Set the field `description`.\nThe description of the parameter. The maximum length is 2000 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the parameter. The maximum length is 200 characters."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `substitution_rules`.\n"]
    pub fn set_substitution_rules(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.substitution_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.substitution_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.validation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubParametersEl {
    type O = BlockAssignable<CloudSecurityComplianceCloudControlParameterSpecElSubParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersEl {
    #[doc = "if the parameter is required"]
    pub is_required: PrimField<bool>,
    #[doc = "The name of the parameter."]
    pub name: PrimField<String>,
    #[doc = "Parameter value type.\nPossible values:\nSTRING\nBOOLEAN\nSTRINGLIST\nNUMBER\nONEOF"]
    pub value_type: PrimField<String>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubParametersEl {
    pub fn build(self) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersEl {
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersEl {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            is_required: self.is_required,
            name: self.name,
            value_type: self.value_type,
            default_value: core::default::Default::default(),
            substitution_rules: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElSubParametersElRef {
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the parameter. The maximum length is 2000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the parameter. The maximum length is 200 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `is_required` after provisioning.\nif the parameter is required"]
    pub fn is_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value_type` after provisioning.\nParameter value type.\nPossible values:\nSTRING\nBOOLEAN\nSTRINGLIST\nNUMBER\nONEOF"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElSubParametersElDefaultValueElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `substitution_rules` after provisioning.\n"]
    pub fn substitution_rules(
        &self,
    ) -> ListRef<
        CloudSecurityComplianceCloudControlParameterSpecElSubParametersElSubstitutionRulesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.substitution_rules", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElSubParametersElValidationElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<PrimField<String>>,
}
impl
    CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleEl
{
    #[doc = "Set the field `attribute`.\nFully qualified proto attribute path (in dot notation).\nExample: rules[0].cel_expression.resource_types_values"]
    pub fn set_attribute(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute = Some(v.into());
        self
    }
}
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleEl { CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleEl { attribute : core :: default :: Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleElRef { CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `attribute` after provisioning.\nFully qualified proto attribute path (in dot notation).\nExample: rules[0].cel_expression.resource_types_values"] pub fn attribute (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.attribute" , self . base)) } }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleEl { # [doc = "Set the field `attribute`.\nFully qualified proto attribute path (e.g., dot notation)"] pub fn set_attribute (mut self , v : impl Into < PrimField < String > >) -> Self { self . attribute = Some (v . into ()) ; self } }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleEl { CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleEl { attribute : core :: default :: Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleElRef { CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `attribute` after provisioning.\nFully qualified proto attribute path (e.g., dot notation)"] pub fn attribute (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.attribute" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElDynamic { attribute_substitution_rule : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleEl >> , placeholder_substitution_rule : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesEl { # [serde (skip_serializing_if = "Option::is_none")] attribute_substitution_rule : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleEl > > , # [serde (skip_serializing_if = "Option::is_none")] placeholder_substitution_rule : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesEl {
    #[doc = "Set the field `attribute_substitution_rule`.\n"]
    pub fn set_attribute_substitution_rule(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.attribute_substitution_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.attribute_substitution_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `placeholder_substitution_rule`.\n"]
    pub fn set_placeholder_substitution_rule(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.placeholder_substitution_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.placeholder_substitution_rule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesEl {
    type O = BlockAssignable<CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesEl {}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesEl {
    pub fn build(self) -> CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesEl {
        CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesEl {
            attribute_substitution_rule: core::default::Default::default(),
            placeholder_substitution_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElRef {
        CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attribute_substitution_rule` after provisioning.\n"]    pub fn attribute_substitution_rule (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElAttributeSubstitutionRuleElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.attribute_substitution_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `placeholder_substitution_rule` after provisioning.\n"]    pub fn placeholder_substitution_rule (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElPlaceholderSubstitutionRuleElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.placeholder_substitution_rule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl
{
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl { }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl
{
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl { CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl { values : self . values , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef { CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElDynamic { string_list_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { # [serde (skip_serializing_if = "Option::is_none")] bool_value : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] number_value : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] string_value : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] string_list_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { # [doc = "Set the field `bool_value`.\nRepresents a boolean value."] pub fn set_bool_value (mut self , v : impl Into < PrimField < bool > >) -> Self { self . bool_value = Some (v . into ()) ; self } # [doc = "Set the field `number_value`.\nRepresents a double value."] pub fn set_number_value (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . number_value = Some (v . into ()) ; self } # [doc = "Set the field `string_value`.\nRepresents a string value."] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } # [doc = "Set the field `string_list_value`.\n"] pub fn set_string_list_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . string_list_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . string_list_value = Some (d) ; } } self } }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl { bool_value : core :: default :: Default :: default () , number_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef { CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."] pub fn bool_value (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bool_value" , self . base)) } # [doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."] pub fn number_value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.number_value" , self . base)) } # [doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } # [doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"] pub fn string_list_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElStringListValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.string_list_value" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElDynamic { parameter_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueEl { # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] parameter_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueEl { # [doc = "Set the field `name`.\nThe name of the parameter."] pub fn set_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . name = Some (v . into ()) ; self } # [doc = "Set the field `parameter_value`.\n"] pub fn set_parameter_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . parameter_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . parameter_value = Some (d) ; } } self } }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueEl { CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueEl { name : core :: default :: Default :: default () , parameter_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElRef { CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"] pub fn parameter_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElParameterValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.parameter_value" , self . base)) } }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueEl
{
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueEl { }
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueEl { type O = BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueEl
{
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueEl { pub fn build (self) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueEl { CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueEl { values : self . values , } } }
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueElRef { CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElDynamic { oneof_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueEl >> , string_list_value : Option < DynamicBlock < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesEl { # [serde (skip_serializing_if = "Option::is_none")] bool_value : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] number_value : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] string_value : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oneof_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueEl > > , # [serde (skip_serializing_if = "Option::is_none")] string_list_value : Option < Vec < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueEl > > , dynamic : CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElDynamic , }
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesEl {
    #[doc = "Set the field `bool_value`.\nRepresents a boolean value."]
    pub fn set_bool_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bool_value = Some(v.into());
        self
    }
    #[doc = "Set the field `number_value`.\nRepresents a double value."]
    pub fn set_number_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number_value = Some(v.into());
        self
    }
    #[doc = "Set the field `string_value`.\nRepresents a string value."]
    pub fn set_string_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.string_value = Some(v.into());
        self
    }
    #[doc = "Set the field `oneof_value`.\n"]
    pub fn set_oneof_value(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oneof_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oneof_value = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `string_list_value`.\n"]
    pub fn set_string_list_value(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.string_list_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list_value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesEl
{}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesEl {
        CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesEl {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            oneof_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElRef
    {
        CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."]
    pub fn bool_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }
    #[doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."]
    pub fn number_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_value", self.base))
    }
    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."]
    pub fn string_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
    #[doc = "Get a reference to the value of field `oneof_value` after provisioning.\n"]    pub fn oneof_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElOneofValueElRef >{
        ListRef::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }
    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]    pub fn string_list_value (& self) -> ListRef < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElStringListValueElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.string_list_value", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElDynamic {
    values: Option<
        DynamicBlock<
            CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<
        Vec<CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesEl>,
    >,
    dynamic: CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElDynamic,
}
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesEl {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesEl >>,
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
    for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesEl {}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesEl {
        CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesEl {
            values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElRef {
        CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(
        &self,
    ) -> ListRef<
        CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElValuesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeEl {
    max: PrimField<String>,
    min: PrimField<String>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeEl {}
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeEl {
    type O =
        BlockAssignable<CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeEl {
    #[doc = "Maximum allowed value for the numeric parameter (inclusive)."]
    pub max: PrimField<String>,
    #[doc = "Minimum allowed value for the numeric parameter (inclusive)."]
    pub min: PrimField<String>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeEl {
    pub fn build(self) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeEl {
        CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeEl {
            max: self.max,
            min: self.min,
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeElRef {
        CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max` after provisioning.\nMaximum allowed value for the numeric parameter (inclusive)."]
    pub fn max(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }
    #[doc = "Get a reference to the value of field `min` after provisioning.\nMinimum allowed value for the numeric parameter (inclusive)."]
    pub fn min(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternEl {
    pattern: PrimField<String>,
}
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternEl {}
impl ToListMappable
    for CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternEl {
    #[doc = "Regex Pattern to match the value(s) of parameter."]
    pub pattern: PrimField<String>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternEl {
        CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternEl {
            pattern: self.pattern,
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternElRef {
        CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pattern` after provisioning.\nRegex Pattern to match the value(s) of parameter."]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElValidationElDynamic {
    allowed_values: Option<
        DynamicBlock<CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesEl>,
    >,
    int_range: Option<
        DynamicBlock<CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeEl>,
    >,
    regexp_pattern: Option<
        DynamicBlock<CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternEl>,
    >,
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_values:
        Option<Vec<CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    int_range:
        Option<Vec<CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regexp_pattern:
        Option<Vec<CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternEl>>,
    dynamic: CloudSecurityComplianceCloudControlParameterSpecElValidationElDynamic,
}
impl CloudSecurityComplianceCloudControlParameterSpecElValidationEl {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowed_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_values = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `int_range`.\n"]
    pub fn set_int_range(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.int_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.int_range = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `regexp_pattern`.\n"]
    pub fn set_regexp_pattern(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.regexp_pattern = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.regexp_pattern = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecElValidationEl {
    type O = BlockAssignable<CloudSecurityComplianceCloudControlParameterSpecElValidationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecElValidationEl {}
impl BuildCloudSecurityComplianceCloudControlParameterSpecElValidationEl {
    pub fn build(self) -> CloudSecurityComplianceCloudControlParameterSpecElValidationEl {
        CloudSecurityComplianceCloudControlParameterSpecElValidationEl {
            allowed_values: core::default::Default::default(),
            int_range: core::default::Default::default(),
            regexp_pattern: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElValidationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElValidationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElValidationElRef {
        CloudSecurityComplianceCloudControlParameterSpecElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElValidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\n"]
    pub fn allowed_values(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElValidationElAllowedValuesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `int_range` after provisioning.\n"]
    pub fn int_range(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElValidationElIntRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.int_range", self.base))
    }
    #[doc = "Get a reference to the value of field `regexp_pattern` after provisioning.\n"]
    pub fn regexp_pattern(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElValidationElRegexpPatternElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.regexp_pattern", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlParameterSpecElDynamic {
    default_value:
        Option<DynamicBlock<CloudSecurityComplianceCloudControlParameterSpecElDefaultValueEl>>,
    sub_parameters:
        Option<DynamicBlock<CloudSecurityComplianceCloudControlParameterSpecElSubParametersEl>>,
    substitution_rules:
        Option<DynamicBlock<CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesEl>>,
    validation:
        Option<DynamicBlock<CloudSecurityComplianceCloudControlParameterSpecElValidationEl>>,
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlParameterSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    is_required: PrimField<bool>,
    name: PrimField<String>,
    value_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<Vec<CloudSecurityComplianceCloudControlParameterSpecElDefaultValueEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_parameters: Option<Vec<CloudSecurityComplianceCloudControlParameterSpecElSubParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    substitution_rules:
        Option<Vec<CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<Vec<CloudSecurityComplianceCloudControlParameterSpecElValidationEl>>,
    dynamic: CloudSecurityComplianceCloudControlParameterSpecElDynamic,
}
impl CloudSecurityComplianceCloudControlParameterSpecEl {
    #[doc = "Set the field `description`.\nThe description of the parameter. The maximum length is 2000 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the parameter. The maximum length is 200 characters."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(
        mut self,
        v: impl Into<BlockAssignable<CloudSecurityComplianceCloudControlParameterSpecElDefaultValueEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sub_parameters`.\n"]
    pub fn set_sub_parameters(
        mut self,
        v: impl Into<BlockAssignable<CloudSecurityComplianceCloudControlParameterSpecElSubParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sub_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sub_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `substitution_rules`.\n"]
    pub fn set_substitution_rules(
        mut self,
        v: impl Into<
            BlockAssignable<CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.substitution_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.substitution_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl Into<BlockAssignable<CloudSecurityComplianceCloudControlParameterSpecElValidationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.validation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudSecurityComplianceCloudControlParameterSpecEl {
    type O = BlockAssignable<CloudSecurityComplianceCloudControlParameterSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlParameterSpecEl {
    #[doc = "if the parameter is required"]
    pub is_required: PrimField<bool>,
    #[doc = "The name of the parameter."]
    pub name: PrimField<String>,
    #[doc = "Parameter value type.\nPossible values:\nSTRING\nBOOLEAN\nSTRINGLIST\nNUMBER\nONEOF"]
    pub value_type: PrimField<String>,
}
impl BuildCloudSecurityComplianceCloudControlParameterSpecEl {
    pub fn build(self) -> CloudSecurityComplianceCloudControlParameterSpecEl {
        CloudSecurityComplianceCloudControlParameterSpecEl {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            is_required: self.is_required,
            name: self.name,
            value_type: self.value_type,
            default_value: core::default::Default::default(),
            sub_parameters: core::default::Default::default(),
            substitution_rules: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlParameterSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlParameterSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlParameterSpecElRef {
        CloudSecurityComplianceCloudControlParameterSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlParameterSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the parameter. The maximum length is 2000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the parameter. The maximum length is 200 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `is_required` after provisioning.\nif the parameter is required"]
    pub fn is_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value_type` after provisioning.\nParameter value type.\nPossible values:\nSTRING\nBOOLEAN\nSTRINGLIST\nNUMBER\nONEOF"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElDefaultValueElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sub_parameters` after provisioning.\n"]
    pub fn sub_parameters(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElSubParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sub_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `substitution_rules` after provisioning.\n"]
    pub fn substitution_rules(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElSubstitutionRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.substitution_rules", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlParameterSpecElValidationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesEl {
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesEl {}
impl ToListMappable
    for CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesEl {
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesEl {
        CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesEl {
            values: self.values,
        }
    }
}
pub struct CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesElRef {
        CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlRulesElCelExpressionElDynamic {
    resource_types_values: Option<
        DynamicBlock<
            CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlRulesElCelExpressionEl {
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_types_values:
        Option<Vec<CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesEl>>,
    dynamic: CloudSecurityComplianceCloudControlRulesElCelExpressionElDynamic,
}
impl CloudSecurityComplianceCloudControlRulesElCelExpressionEl {
    #[doc = "Set the field `resource_types_values`.\n"]
    pub fn set_resource_types_values(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_types_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_types_values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudSecurityComplianceCloudControlRulesElCelExpressionEl {
    type O = BlockAssignable<CloudSecurityComplianceCloudControlRulesElCelExpressionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlRulesElCelExpressionEl {
    #[doc = "Logic expression in CEL language.\nThe max length of the condition is 1000 characters."]
    pub expression: PrimField<String>,
}
impl BuildCloudSecurityComplianceCloudControlRulesElCelExpressionEl {
    pub fn build(self) -> CloudSecurityComplianceCloudControlRulesElCelExpressionEl {
        CloudSecurityComplianceCloudControlRulesElCelExpressionEl {
            expression: self.expression,
            resource_types_values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlRulesElCelExpressionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlRulesElCelExpressionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceCloudControlRulesElCelExpressionElRef {
        CloudSecurityComplianceCloudControlRulesElCelExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlRulesElCelExpressionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nLogic expression in CEL language.\nThe max length of the condition is 1000 characters."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_types_values` after provisioning.\n"]
    pub fn resource_types_values(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlRulesElCelExpressionElResourceTypesValuesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_types_values", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceCloudControlRulesElDynamic {
    cel_expression: Option<DynamicBlock<CloudSecurityComplianceCloudControlRulesElCelExpressionEl>>,
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    rule_action_types: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cel_expression: Option<Vec<CloudSecurityComplianceCloudControlRulesElCelExpressionEl>>,
    dynamic: CloudSecurityComplianceCloudControlRulesElDynamic,
}
impl CloudSecurityComplianceCloudControlRulesEl {
    #[doc = "Set the field `description`.\nDescription of the Rule. The maximum length is 2000 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `cel_expression`.\n"]
    pub fn set_cel_expression(
        mut self,
        v: impl Into<BlockAssignable<CloudSecurityComplianceCloudControlRulesElCelExpressionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cel_expression = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cel_expression = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudSecurityComplianceCloudControlRulesEl {
    type O = BlockAssignable<CloudSecurityComplianceCloudControlRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlRulesEl {
    #[doc = "The functionality enabled by the Rule."]
    pub rule_action_types: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceCloudControlRulesEl {
    pub fn build(self) -> CloudSecurityComplianceCloudControlRulesEl {
        CloudSecurityComplianceCloudControlRulesEl {
            description: core::default::Default::default(),
            rule_action_types: self.rule_action_types,
            cel_expression: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlRulesElRef {
    fn new(shared: StackShared, base: String) -> CloudSecurityComplianceCloudControlRulesElRef {
        CloudSecurityComplianceCloudControlRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the Rule. The maximum length is 2000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `rule_action_types` after provisioning.\nThe functionality enabled by the Rule."]
    pub fn rule_action_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_action_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cel_expression` after provisioning.\n"]
    pub fn cel_expression(
        &self,
    ) -> ListRef<CloudSecurityComplianceCloudControlRulesElCelExpressionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cel_expression", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceCloudControlTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudSecurityComplianceCloudControlTimeoutsEl {
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
impl ToListMappable for CloudSecurityComplianceCloudControlTimeoutsEl {
    type O = BlockAssignable<CloudSecurityComplianceCloudControlTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceCloudControlTimeoutsEl {}
impl BuildCloudSecurityComplianceCloudControlTimeoutsEl {
    pub fn build(self) -> CloudSecurityComplianceCloudControlTimeoutsEl {
        CloudSecurityComplianceCloudControlTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceCloudControlTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceCloudControlTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudSecurityComplianceCloudControlTimeoutsElRef {
        CloudSecurityComplianceCloudControlTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceCloudControlTimeoutsElRef {
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
struct CloudSecurityComplianceCloudControlDynamic {
    parameter_spec: Option<DynamicBlock<CloudSecurityComplianceCloudControlParameterSpecEl>>,
    rules: Option<DynamicBlock<CloudSecurityComplianceCloudControlRulesEl>>,
}
