use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudSecurityComplianceFrameworkDeploymentData {
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
    framework_deployment_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    organization: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_control_metadata:
        Option<Vec<CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    framework: Option<Vec<CloudSecurityComplianceFrameworkDeploymentFrameworkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_resource_config:
        Option<Vec<CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudSecurityComplianceFrameworkDeploymentTimeoutsEl>,
    dynamic: CloudSecurityComplianceFrameworkDeploymentDynamic,
}
struct CloudSecurityComplianceFrameworkDeployment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudSecurityComplianceFrameworkDeploymentData>,
}
#[derive(Clone)]
pub struct CloudSecurityComplianceFrameworkDeployment(
    Rc<CloudSecurityComplianceFrameworkDeployment_>,
);
impl CloudSecurityComplianceFrameworkDeployment {
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
    #[doc = "Set the field `description`.\nUser provided description of the Framework deployment"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_control_metadata`.\n"]
    pub fn set_cloud_control_metadata(
        self,
        v: impl Into<BlockAssignable<CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_control_metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_control_metadata = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `framework`.\n"]
    pub fn set_framework(
        self,
        v: impl Into<BlockAssignable<CloudSecurityComplianceFrameworkDeploymentFrameworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().framework = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.framework = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target_resource_config`.\n"]
    pub fn set_target_resource_config(
        self,
        v: impl Into<BlockAssignable<CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_resource_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_resource_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<CloudSecurityComplianceFrameworkDeploymentTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cloud_control_deployment_references` after provisioning.\nThe references to the cloud control deployments. It has all the\nCloudControlDeployments which are either directly added in the framework or\nthrough a CloudControlGroup.\nExample: If a framework deployment deploys two\ncloud controls, cc-deployment-1 and cc-deployment-2, then the\ncloud_control_deployment_references will be:\n{\ncloud_control_deployment_reference: {\ncloud_control_deployment:\n\"organizations/{organization}/locations/{location}/cloudControlDeployments/cc-deployment-1\"\n},\ncloud_control_deployment_reference: {\ncloud_control_deployment:\n\"organizations/{organization}/locations/{location}/cloudControlDeployments/cc-deployment-2\"\n}"]
    pub fn cloud_control_deployment_references(
        &self,
    ) -> ListRef<CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_control_deployment_references", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `computed_target_resource` after provisioning.\nThe resource on which the Framework is deployed based on the provided\nTargetResourceConfig in the following format:\norganizations/{organization}, folders/{folder} or projects/{project}"]
    pub fn computed_target_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.computed_target_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the resource was created."]
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
    #[doc = "Get a reference to the value of field `deployment_state` after provisioning.\nThe deployment state of the framework.\nPossible values:\nDEPLOYMENT_STATE_VALIDATING\nDEPLOYMENT_STATE_CREATING\nDEPLOYMENT_STATE_DELETING\nDEPLOYMENT_STATE_FAILED\nDEPLOYMENT_STATE_READY\nDEPLOYMENT_STATE_PARTIALLY_DEPLOYED\nDEPLOYMENT_STATE_PARTIALLY_DELETED"]
    pub fn deployment_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser provided description of the Framework deployment"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nTo prevent concurrent updates from overwriting each other, always provide\nthe 'etag' when you update a FrameworkDeployment. You can also\nprovide the 'etag' when you delete a FrameworkDeployment, to help\nensure that you're deleting the intended version of the\nFrameworkDeployment."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `framework_deployment_id` after provisioning.\nUser provided identifier. It should be unique in scope of a parent.\nThis is optional and if not provided, a random UUID will be generated."]
    pub fn framework_deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framework_deployment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. FrameworkDeployment name in the following format:\norganizations/{organization}/locations/{location}/frameworkDeployments/{framework_deployment_id}"]
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
    #[doc = "Get a reference to the value of field `target_resource_display_name` after provisioning.\nThe display name of the target resource."]
    pub fn target_resource_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_resource_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the resource last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_control_metadata` after provisioning.\n"]
    pub fn cloud_control_metadata(
        &self,
    ) -> ListRef<CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_control_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `framework` after provisioning.\n"]
    pub fn framework(&self) -> ListRef<CloudSecurityComplianceFrameworkDeploymentFrameworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.framework", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_resource_config` after provisioning.\n"]
    pub fn target_resource_config(
        &self,
    ) -> ListRef<CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_resource_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceFrameworkDeploymentTimeoutsElRef {
        CloudSecurityComplianceFrameworkDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudSecurityComplianceFrameworkDeployment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudSecurityComplianceFrameworkDeployment {}
impl ToListMappable for CloudSecurityComplianceFrameworkDeployment {
    type O = ListRef<CloudSecurityComplianceFrameworkDeploymentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudSecurityComplianceFrameworkDeployment_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_security_compliance_framework_deployment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudSecurityComplianceFrameworkDeployment {
    pub tf_id: String,
    #[doc = "User provided identifier. It should be unique in scope of a parent.\nThis is optional and if not provided, a random UUID will be generated."]
    pub framework_deployment_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub organization: PrimField<String>,
}
impl BuildCloudSecurityComplianceFrameworkDeployment {
    pub fn build(self, stack: &mut Stack) -> CloudSecurityComplianceFrameworkDeployment {
        let out = CloudSecurityComplianceFrameworkDeployment(Rc::new(
            CloudSecurityComplianceFrameworkDeployment_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(CloudSecurityComplianceFrameworkDeploymentData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    framework_deployment_id: self.framework_deployment_id,
                    id: core::default::Default::default(),
                    location: self.location,
                    organization: self.organization,
                    cloud_control_metadata: core::default::Default::default(),
                    framework: core::default::Default::default(),
                    target_resource_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudSecurityComplianceFrameworkDeploymentRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudSecurityComplianceFrameworkDeploymentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_control_deployment_references` after provisioning.\nThe references to the cloud control deployments. It has all the\nCloudControlDeployments which are either directly added in the framework or\nthrough a CloudControlGroup.\nExample: If a framework deployment deploys two\ncloud controls, cc-deployment-1 and cc-deployment-2, then the\ncloud_control_deployment_references will be:\n{\ncloud_control_deployment_reference: {\ncloud_control_deployment:\n\"organizations/{organization}/locations/{location}/cloudControlDeployments/cc-deployment-1\"\n},\ncloud_control_deployment_reference: {\ncloud_control_deployment:\n\"organizations/{organization}/locations/{location}/cloudControlDeployments/cc-deployment-2\"\n}"]
    pub fn cloud_control_deployment_references(
        &self,
    ) -> ListRef<CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_control_deployment_references", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `computed_target_resource` after provisioning.\nThe resource on which the Framework is deployed based on the provided\nTargetResourceConfig in the following format:\norganizations/{organization}, folders/{folder} or projects/{project}"]
    pub fn computed_target_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.computed_target_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the resource was created."]
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
    #[doc = "Get a reference to the value of field `deployment_state` after provisioning.\nThe deployment state of the framework.\nPossible values:\nDEPLOYMENT_STATE_VALIDATING\nDEPLOYMENT_STATE_CREATING\nDEPLOYMENT_STATE_DELETING\nDEPLOYMENT_STATE_FAILED\nDEPLOYMENT_STATE_READY\nDEPLOYMENT_STATE_PARTIALLY_DEPLOYED\nDEPLOYMENT_STATE_PARTIALLY_DELETED"]
    pub fn deployment_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser provided description of the Framework deployment"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nTo prevent concurrent updates from overwriting each other, always provide\nthe 'etag' when you update a FrameworkDeployment. You can also\nprovide the 'etag' when you delete a FrameworkDeployment, to help\nensure that you're deleting the intended version of the\nFrameworkDeployment."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `framework_deployment_id` after provisioning.\nUser provided identifier. It should be unique in scope of a parent.\nThis is optional and if not provided, a random UUID will be generated."]
    pub fn framework_deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framework_deployment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. FrameworkDeployment name in the following format:\norganizations/{organization}/locations/{location}/frameworkDeployments/{framework_deployment_id}"]
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
    #[doc = "Get a reference to the value of field `target_resource_display_name` after provisioning.\nThe display name of the target resource."]
    pub fn target_resource_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_resource_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the resource last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_control_metadata` after provisioning.\n"]
    pub fn cloud_control_metadata(
        &self,
    ) -> ListRef<CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_control_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `framework` after provisioning.\n"]
    pub fn framework(&self) -> ListRef<CloudSecurityComplianceFrameworkDeploymentFrameworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.framework", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_resource_config` after provisioning.\n"]
    pub fn target_resource_config(
        &self,
    ) -> ListRef<CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_resource_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceFrameworkDeploymentTimeoutsElRef {
        CloudSecurityComplianceFrameworkDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_control_deployment: Option<PrimField<String>>,
}
impl CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesEl {
    #[doc = "Set the field `cloud_control_deployment`.\n"]
    pub fn set_cloud_control_deployment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_control_deployment = Some(v.into());
        self
    }
}
impl ToListMappable
    for CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesEl {}
impl BuildCloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesEl {
        CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesEl {
            cloud_control_deployment: core::default::Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesElRef {
        CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceFrameworkDeploymentCloudControlDeploymentReferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_control_deployment` after provisioning.\n"]
    pub fn cloud_control_deployment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_control_deployment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl
{
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl { }
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl
{
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl { pub fn build (self) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl { values : self . values , } } }
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElDynamic { string_list_value : Option < DynamicBlock < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { # [serde (skip_serializing_if = "Option::is_none")] bool_value : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] number_value : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] string_value : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] string_list_value : Option < Vec < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl > > , dynamic : CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElDynamic , }
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { # [doc = "Set the field `bool_value`.\nRepresents a boolean value."] pub fn set_bool_value (mut self , v : impl Into < PrimField < bool > >) -> Self { self . bool_value = Some (v . into ()) ; self } # [doc = "Set the field `number_value`.\nRepresents a double value."] pub fn set_number_value (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . number_value = Some (v . into ()) ; self } # [doc = "Set the field `string_value`.\nRepresents a string value."] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } # [doc = "Set the field `string_list_value`.\n"] pub fn set_string_list_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . string_list_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . string_list_value = Some (d) ; } } self } }
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl
{}
impl BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { pub fn build (self) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { bool_value : core :: default :: Default :: default () , number_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."] pub fn bool_value (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bool_value" , self . base)) } # [doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."] pub fn number_value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.number_value" , self . base)) } # [doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } # [doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"] pub fn string_list_value (& self) -> ListRef < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.string_list_value" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElDynamic { parameter_value : Option < DynamicBlock < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueEl { # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] parameter_value : Option < Vec < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl > > , dynamic : CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElDynamic , }
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueEl { # [doc = "Set the field `name`.\nThe name of the parameter."] pub fn set_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . name = Some (v . into ()) ; self } # [doc = "Set the field `parameter_value`.\n"] pub fn set_parameter_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . parameter_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . parameter_value = Some (d) ; } } self } }
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueEl
{}
impl BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueEl { pub fn build (self) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueEl { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueEl { name : core :: default :: Default :: default () , parameter_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElRef { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"] pub fn parameter_value (& self) -> ListRef < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.parameter_value" , self . base)) } }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueEl
{
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueEl { }
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueEl
{
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueEl { pub fn build (self) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueEl { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueEl { values : self . values , } } }
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueElRef { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElDynamic { oneof_value : Option < DynamicBlock < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueEl >> , string_list_value : Option < DynamicBlock < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueEl { # [serde (skip_serializing_if = "Option::is_none")] bool_value : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] number_value : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] string_value : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oneof_value : Option < Vec < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueEl > > , # [serde (skip_serializing_if = "Option::is_none")] string_list_value : Option < Vec < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueEl > > , dynamic : CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElDynamic , }
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueEl { # [doc = "Set the field `bool_value`.\nRepresents a boolean value."] pub fn set_bool_value (mut self , v : impl Into < PrimField < bool > >) -> Self { self . bool_value = Some (v . into ()) ; self } # [doc = "Set the field `number_value`.\nRepresents a double value."] pub fn set_number_value (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . number_value = Some (v . into ()) ; self } # [doc = "Set the field `string_value`.\nRepresents a string value."] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } # [doc = "Set the field `oneof_value`.\n"] pub fn set_oneof_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . oneof_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . oneof_value = Some (d) ; } } self } # [doc = "Set the field `string_list_value`.\n"] pub fn set_string_list_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . string_list_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . string_list_value = Some (d) ; } } self } }
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueEl
{}
impl BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueEl { pub fn build (self) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueEl { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueEl { bool_value : core :: default :: Default :: default () , number_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , oneof_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElRef { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."] pub fn bool_value (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bool_value" , self . base)) } # [doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."] pub fn number_value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.number_value" , self . base)) } # [doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } # [doc = "Get a reference to the value of field `oneof_value` after provisioning.\n"] pub fn oneof_value (& self) -> ListRef < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElOneofValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.oneof_value" , self . base)) } # [doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"] pub fn string_list_value (& self) -> ListRef < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElStringListValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.string_list_value" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElDynamic { parameter_value : Option < DynamicBlock < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersEl { name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] parameter_value : Option < Vec < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueEl > > , dynamic : CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElDynamic , }
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersEl { # [doc = "Set the field `parameter_value`.\n"] pub fn set_parameter_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . parameter_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . parameter_value = Some (d) ; } } self } }
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersEl
{
    #[doc = "The name of the parameter."]
    pub name: PrimField<String>,
}
impl BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersEl { pub fn build (self) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersEl { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersEl { name : self . name , parameter_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElRef { CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"] pub fn parameter_value (& self) -> ListRef < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElParameterValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.parameter_value" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElDynamic { parameters : Option < DynamicBlock < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsEl { major_revision_id : PrimField < String > , name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] parameters : Option < Vec < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersEl > > , dynamic : CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElDynamic , }
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsEl {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsEl
{
    #[doc = "Major revision of cloudcontrol"]
    pub major_revision_id: PrimField<String>,
    #[doc = "The name of the CloudControl in the format:\n“organizations/{organization}/locations/{location}/\ncloudControls/{cloud-control}”"]
    pub name: PrimField<String>,
}
impl BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsEl {
        CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsEl {
            major_revision_id: self.major_revision_id,
            name: self.name,
            parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElRef
    {
        CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision of cloudcontrol"]
    pub fn major_revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.major_revision_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the CloudControl in the format:\n“organizations/{organization}/locations/{location}/\ncloudControls/{cloud-control}”"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]    pub fn parameters (& self) -> ListRef < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElParametersElRef >{
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElDynamic {
    cloud_control_details: Option<
        DynamicBlock<
            CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataEl {
    enforcement_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_control_details: Option<
        Vec<CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsEl>,
    >,
    dynamic: CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElDynamic,
}
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataEl {
    #[doc = "Set the field `cloud_control_details`.\n"]
    pub fn set_cloud_control_details(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_control_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_control_details = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataEl {
    type O = BlockAssignable<CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataEl {
    #[doc = "Enforcement mode for the framework deployment.\nPossible values:\nPREVENTIVE\nDETECTIVE\nAUDIT"]
    pub enforcement_mode: PrimField<String>,
}
impl BuildCloudSecurityComplianceFrameworkDeploymentCloudControlMetadataEl {
    pub fn build(self) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataEl {
        CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataEl {
            enforcement_mode: self.enforcement_mode,
            cloud_control_details: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElRef {
        CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforcement_mode` after provisioning.\nEnforcement mode for the framework deployment.\nPossible values:\nPREVENTIVE\nDETECTIVE\nAUDIT"]
    pub fn enforcement_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcement_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_control_details` after provisioning.\n"]
    pub fn cloud_control_details(
        &self,
    ) -> ListRef<
        CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataElCloudControlDetailsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_control_details", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentFrameworkEl {
    framework: PrimField<String>,
    major_revision_id: PrimField<String>,
}
impl CloudSecurityComplianceFrameworkDeploymentFrameworkEl {}
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentFrameworkEl {
    type O = BlockAssignable<CloudSecurityComplianceFrameworkDeploymentFrameworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceFrameworkDeploymentFrameworkEl {
    #[doc = "In the format:\norganizations/{org}/locations/{location}/frameworks/{framework}"]
    pub framework: PrimField<String>,
    #[doc = "Major revision id of the framework."]
    pub major_revision_id: PrimField<String>,
}
impl BuildCloudSecurityComplianceFrameworkDeploymentFrameworkEl {
    pub fn build(self) -> CloudSecurityComplianceFrameworkDeploymentFrameworkEl {
        CloudSecurityComplianceFrameworkDeploymentFrameworkEl {
            framework: self.framework,
            major_revision_id: self.major_revision_id,
        }
    }
}
pub struct CloudSecurityComplianceFrameworkDeploymentFrameworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentFrameworkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceFrameworkDeploymentFrameworkElRef {
        CloudSecurityComplianceFrameworkDeploymentFrameworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceFrameworkDeploymentFrameworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `framework` after provisioning.\nIn the format:\norganizations/{org}/locations/{location}/frameworks/{framework}"]
    pub fn framework(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.framework", self.base))
    }
    #[doc = "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision id of the framework."]
    pub fn major_revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.major_revision_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigEl
{
    folder_display_name: PrimField<String>,
    parent: PrimField<String>,
}
impl CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigEl { }
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigEl
{
    #[doc = "Display name of the folder to be created"]
    pub folder_display_name: PrimField<String>,
    #[doc = "The parent of the folder to be created. It can be an organizations/{org} or\nfolders/{folder}"]
    pub parent: PrimField<String>,
}
impl BuildCloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigEl { pub fn build (self) -> CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigEl { CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigEl { folder_display_name : self . folder_display_name , parent : self . parent , } } }
pub struct CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigElRef { CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `folder_display_name` after provisioning.\nDisplay name of the folder to be created"] pub fn folder_display_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.folder_display_name" , self . base)) } # [doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the folder to be created. It can be an organizations/{org} or\nfolders/{folder}"] pub fn parent (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.parent" , self . base)) } }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigEl
{
    billing_account_id: PrimField<String>,
    parent: PrimField<String>,
    project_display_name: PrimField<String>,
}
impl CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigEl { }
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigEl
{
    #[doc = "Billing account id to be used for the project."]
    pub billing_account_id: PrimField<String>,
    #[doc = "organizations/{org} or folders/{folder}"]
    pub parent: PrimField<String>,
    #[doc = "Display name of the project to be created."]
    pub project_display_name: PrimField<String>,
}
impl BuildCloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigEl { pub fn build (self) -> CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigEl { CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigEl { billing_account_id : self . billing_account_id , parent : self . parent , project_display_name : self . project_display_name , } } }
pub struct CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigElRef { CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `billing_account_id` after provisioning.\nBilling account id to be used for the project."] pub fn billing_account_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.billing_account_id" , self . base)) } # [doc = "Get a reference to the value of field `parent` after provisioning.\norganizations/{org} or folders/{folder}"] pub fn parent (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.parent" , self . base)) } # [doc = "Get a reference to the value of field `project_display_name` after provisioning.\nDisplay name of the project to be created."] pub fn project_display_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.project_display_name" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElDynamic { folder_creation_config : Option < DynamicBlock < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigEl >> , project_creation_config : Option < DynamicBlock < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigEl { # [serde (skip_serializing_if = "Option::is_none")] folder_creation_config : Option < Vec < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] project_creation_config : Option < Vec < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigEl > > , dynamic : CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElDynamic , }
impl
    CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigEl
{
    #[doc = "Set the field `folder_creation_config`.\n"]
    pub fn set_folder_creation_config(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.folder_creation_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.folder_creation_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `project_creation_config`.\n"]
    pub fn set_project_creation_config(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.project_creation_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.project_creation_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigEl
{}
impl BuildCloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigEl { pub fn build (self) -> CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigEl { CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigEl { folder_creation_config : core :: default :: Default :: default () , project_creation_config : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElRef { CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `folder_creation_config` after provisioning.\n"] pub fn folder_creation_config (& self) -> ListRef < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElFolderCreationConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.folder_creation_config" , self . base)) } # [doc = "Get a reference to the value of field `project_creation_config` after provisioning.\n"] pub fn project_creation_config (& self) -> ListRef < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElProjectCreationConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.project_creation_config" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElDynamic { target_resource_creation_config : Option < DynamicBlock < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigEl { # [serde (skip_serializing_if = "Option::is_none")] existing_target_resource : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] target_resource_creation_config : Option < Vec < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigEl > > , dynamic : CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElDynamic , }
impl CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigEl {
    #[doc = "Set the field `existing_target_resource`.\nCRM node in format organizations/{organization}, folders/{folder},\nor projects/{project}"]
    pub fn set_existing_target_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.existing_target_resource = Some(v.into());
        self
    }
    #[doc = "Set the field `target_resource_creation_config`.\n"]
    pub fn set_target_resource_creation_config(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_resource_creation_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_resource_creation_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigEl {
    type O = BlockAssignable<CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceFrameworkDeploymentTargetResourceConfigEl {}
impl BuildCloudSecurityComplianceFrameworkDeploymentTargetResourceConfigEl {
    pub fn build(self) -> CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigEl {
        CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigEl {
            existing_target_resource: core::default::Default::default(),
            target_resource_creation_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElRef {
        CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `existing_target_resource` after provisioning.\nCRM node in format organizations/{organization}, folders/{folder},\nor projects/{project}"]
    pub fn existing_target_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.existing_target_resource", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_resource_creation_config` after provisioning.\n"]    pub fn target_resource_creation_config (& self) -> ListRef < CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigElTargetResourceCreationConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_resource_creation_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkDeploymentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl CloudSecurityComplianceFrameworkDeploymentTimeoutsEl {
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
impl ToListMappable for CloudSecurityComplianceFrameworkDeploymentTimeoutsEl {
    type O = BlockAssignable<CloudSecurityComplianceFrameworkDeploymentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceFrameworkDeploymentTimeoutsEl {}
impl BuildCloudSecurityComplianceFrameworkDeploymentTimeoutsEl {
    pub fn build(self) -> CloudSecurityComplianceFrameworkDeploymentTimeoutsEl {
        CloudSecurityComplianceFrameworkDeploymentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceFrameworkDeploymentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkDeploymentTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceFrameworkDeploymentTimeoutsElRef {
        CloudSecurityComplianceFrameworkDeploymentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceFrameworkDeploymentTimeoutsElRef {
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
struct CloudSecurityComplianceFrameworkDeploymentDynamic {
    cloud_control_metadata:
        Option<DynamicBlock<CloudSecurityComplianceFrameworkDeploymentCloudControlMetadataEl>>,
    framework: Option<DynamicBlock<CloudSecurityComplianceFrameworkDeploymentFrameworkEl>>,
    target_resource_config:
        Option<DynamicBlock<CloudSecurityComplianceFrameworkDeploymentTargetResourceConfigEl>>,
}
