use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudSecurityComplianceFrameworkData {
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
    display_name: Option<PrimField<String>>,
    framework_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    organization: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_control_details: Option<Vec<CloudSecurityComplianceFrameworkCloudControlDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudSecurityComplianceFrameworkTimeoutsEl>,
    dynamic: CloudSecurityComplianceFrameworkDynamic,
}
struct CloudSecurityComplianceFramework_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudSecurityComplianceFrameworkData>,
}
#[derive(Clone)]
pub struct CloudSecurityComplianceFramework(Rc<CloudSecurityComplianceFramework_>);
impl CloudSecurityComplianceFramework {
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
    #[doc = "Set the field `description`.\nThe description of the framework. The maximum length is 2000 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nDisplay name of the framework. The maximum length is 200 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_control_details`.\n"]
    pub fn set_cloud_control_details(
        self,
        v: impl Into<BlockAssignable<CloudSecurityComplianceFrameworkCloudControlDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_control_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_control_details = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudSecurityComplianceFrameworkTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `category` after provisioning.\nThe category of the framework."]
    pub fn category(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.category", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the framework. The maximum length is 2000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the framework. The maximum length is 200 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `framework_id` after provisioning.\nID of the framework.\nThis is not the full name of the framework.\nThis is the last part of the full name of the framework."]
    pub fn framework_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framework_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision of the framework incremented in ascending order."]
    pub fn major_revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.major_revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the framework.\nFormat:\norganizations/{organization}/locations/{{location}}/frameworks/{framework_id}"]
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
    #[doc = "Get a reference to the value of field `supported_cloud_providers` after provisioning.\ncloud providers supported"]
    pub fn supported_cloud_providers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_cloud_providers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_enforcement_modes` after provisioning.\nThe supported enforcement modes of the framework."]
    pub fn supported_enforcement_modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_enforcement_modes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_target_resource_types` after provisioning.\ntarget resource types supported by the Framework."]
    pub fn supported_target_resource_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_target_resource_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the framework. The default is TYPE_CUSTOM.\nPossible values:\nBUILT_IN\nCUSTOM"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_control_details` after provisioning.\n"]
    pub fn cloud_control_details(
        &self,
    ) -> ListRef<CloudSecurityComplianceFrameworkCloudControlDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_control_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceFrameworkTimeoutsElRef {
        CloudSecurityComplianceFrameworkTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudSecurityComplianceFramework {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudSecurityComplianceFramework {}
impl ToListMappable for CloudSecurityComplianceFramework {
    type O = ListRef<CloudSecurityComplianceFrameworkRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudSecurityComplianceFramework_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_security_compliance_framework".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudSecurityComplianceFramework {
    pub tf_id: String,
    #[doc = "ID of the framework.\nThis is not the full name of the framework.\nThis is the last part of the full name of the framework."]
    pub framework_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub organization: PrimField<String>,
}
impl BuildCloudSecurityComplianceFramework {
    pub fn build(self, stack: &mut Stack) -> CloudSecurityComplianceFramework {
        let out = CloudSecurityComplianceFramework(Rc::new(CloudSecurityComplianceFramework_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudSecurityComplianceFrameworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                framework_id: self.framework_id,
                id: core::default::Default::default(),
                location: self.location,
                organization: self.organization,
                cloud_control_details: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudSecurityComplianceFrameworkRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudSecurityComplianceFrameworkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `category` after provisioning.\nThe category of the framework."]
    pub fn category(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.category", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the framework. The maximum length is 2000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the framework. The maximum length is 200 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `framework_id` after provisioning.\nID of the framework.\nThis is not the full name of the framework.\nThis is the last part of the full name of the framework."]
    pub fn framework_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framework_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `major_revision_id` after provisioning.\nMajor revision of the framework incremented in ascending order."]
    pub fn major_revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.major_revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the framework.\nFormat:\norganizations/{organization}/locations/{{location}}/frameworks/{framework_id}"]
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
    #[doc = "Get a reference to the value of field `supported_cloud_providers` after provisioning.\ncloud providers supported"]
    pub fn supported_cloud_providers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_cloud_providers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_enforcement_modes` after provisioning.\nThe supported enforcement modes of the framework."]
    pub fn supported_enforcement_modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_enforcement_modes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_target_resource_types` after provisioning.\ntarget resource types supported by the Framework."]
    pub fn supported_target_resource_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_target_resource_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the framework. The default is TYPE_CUSTOM.\nPossible values:\nBUILT_IN\nCUSTOM"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_control_details` after provisioning.\n"]
    pub fn cloud_control_details(
        &self,
    ) -> ListRef<CloudSecurityComplianceFrameworkCloudControlDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_control_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSecurityComplianceFrameworkTimeoutsElRef {
        CloudSecurityComplianceFrameworkTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl
{
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl { }
impl ToListMappable for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl
{
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl { pub fn build (self) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl { CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl { values : self . values , } } }
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef { CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElDynamic { string_list_value : Option < DynamicBlock < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { # [serde (skip_serializing_if = "Option::is_none")] bool_value : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] number_value : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] string_value : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] string_list_value : Option < Vec < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl > > , dynamic : CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElDynamic , }
impl CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { # [doc = "Set the field `bool_value`.\nRepresents a boolean value."] pub fn set_bool_value (mut self , v : impl Into < PrimField < bool > >) -> Self { self . bool_value = Some (v . into ()) ; self } # [doc = "Set the field `number_value`.\nRepresents a double value."] pub fn set_number_value (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . number_value = Some (v . into ()) ; self } # [doc = "Set the field `string_value`.\nRepresents a string value."] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } # [doc = "Set the field `string_list_value`.\n"] pub fn set_string_list_value (mut self , v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . string_list_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . string_list_value = Some (d) ; } } self } }
impl ToListMappable for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl
{}
impl BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { pub fn build (self) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl { bool_value : core :: default :: Default :: default () , number_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef { CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `bool_value` after provisioning.\nRepresents a boolean value."] pub fn bool_value (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.bool_value" , self . base)) } # [doc = "Get a reference to the value of field `number_value` after provisioning.\nRepresents a double value."] pub fn number_value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.number_value" , self . base)) } # [doc = "Get a reference to the value of field `string_value` after provisioning.\nRepresents a string value."] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } # [doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"] pub fn string_list_value (& self) -> ListRef < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElStringListValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.string_list_value" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElDynamic { parameter_value : Option < DynamicBlock < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueEl { # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] parameter_value : Option < Vec < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl > > , dynamic : CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElDynamic , }
impl CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueEl {
    #[doc = "Set the field `name`.\nThe name of the parameter."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_value`.\n"]
    pub fn set_parameter_value(
        mut self,
        v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueEl >>,
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
impl ToListMappable for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueEl
{}
impl BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueEl { pub fn build (self) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueEl { CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueEl { name : core :: default :: Default :: default () , parameter_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElRef { CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElRef { shared : shared , base : base . to_string () , } } }
impl
    CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"]    pub fn parameter_value (& self) -> ListRef < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElParameterValueElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameter_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueEl
{
    values: ListField<PrimField<String>>,
}
impl CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueEl { }
impl ToListMappable for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueEl { type O = BlockAssignable < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueEl
{
    #[doc = "The strings in the list."]
    pub values: ListField<PrimField<String>>,
}
impl BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueEl { pub fn build (self) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueEl { CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueEl { values : self . values , } } }
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueElRef { fn new (shared : StackShared , base : String) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueElRef { CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueElRef { shared : shared , base : base . to_string () , } } }
impl CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `values` after provisioning.\nThe strings in the list."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElDynamic { oneof_value : Option < DynamicBlock < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueEl >> , string_list_value : Option < DynamicBlock < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueEl >> , }
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueEl { # [serde (skip_serializing_if = "Option::is_none")] bool_value : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] number_value : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] string_value : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oneof_value : Option < Vec < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueEl > > , # [serde (skip_serializing_if = "Option::is_none")] string_list_value : Option < Vec < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueEl > > , dynamic : CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElDynamic , }
impl CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueEl {
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
        v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueEl >>,
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
        v : impl Into < BlockAssignable < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueEl >>,
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
    for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueEl
{
    type O = BlockAssignable<
        CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueEl {
}
impl BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueEl {
    pub fn build(
        self,
    ) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueEl {
        CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueEl {
            bool_value: core::default::Default::default(),
            number_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
            oneof_value: core::default::Default::default(),
            string_list_value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElRef {
        CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElRef {
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
    #[doc = "Get a reference to the value of field `oneof_value` after provisioning.\n"]    pub fn oneof_value (& self) -> ListRef < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElOneofValueElRef >{
        ListRef::new(self.shared().clone(), format!("{}.oneof_value", self.base))
    }
    #[doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"]    pub fn string_list_value (& self) -> ListRef < CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElStringListValueElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.string_list_value", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElDynamic {
    parameter_value: Option<
        DynamicBlock<
            CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_value: Option<
        Vec<CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueEl>,
    >,
    dynamic: CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElDynamic,
}
impl CloudSecurityComplianceFrameworkCloudControlDetailsElParametersEl {
    #[doc = "Set the field `parameter_value`.\n"]
    pub fn set_parameter_value(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueEl,
            >,
        >,
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
impl ToListMappable for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersEl {
    type O = BlockAssignable<CloudSecurityComplianceFrameworkCloudControlDetailsElParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersEl {
    #[doc = "The name of the parameter."]
    pub name: PrimField<String>,
}
impl BuildCloudSecurityComplianceFrameworkCloudControlDetailsElParametersEl {
    pub fn build(self) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersEl {
        CloudSecurityComplianceFrameworkCloudControlDetailsElParametersEl {
            name: self.name,
            parameter_value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElRef {
        CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the parameter."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(
        &self,
    ) -> ListRef<CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElParameterValueElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameter_value", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudSecurityComplianceFrameworkCloudControlDetailsElDynamic {
    parameters:
        Option<DynamicBlock<CloudSecurityComplianceFrameworkCloudControlDetailsElParametersEl>>,
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsEl {
    major_revision_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<CloudSecurityComplianceFrameworkCloudControlDetailsElParametersEl>>,
    dynamic: CloudSecurityComplianceFrameworkCloudControlDetailsElDynamic,
}
impl CloudSecurityComplianceFrameworkCloudControlDetailsEl {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl Into<BlockAssignable<CloudSecurityComplianceFrameworkCloudControlDetailsElParametersEl>>,
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
impl ToListMappable for CloudSecurityComplianceFrameworkCloudControlDetailsEl {
    type O = BlockAssignable<CloudSecurityComplianceFrameworkCloudControlDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceFrameworkCloudControlDetailsEl {
    #[doc = "Major revision of cloudcontrol"]
    pub major_revision_id: PrimField<String>,
    #[doc = "The name of the CloudControl in the format:\n“organizations/{organization}/locations/{location}/cloudControls/{cloud-control}”"]
    pub name: PrimField<String>,
}
impl BuildCloudSecurityComplianceFrameworkCloudControlDetailsEl {
    pub fn build(self) -> CloudSecurityComplianceFrameworkCloudControlDetailsEl {
        CloudSecurityComplianceFrameworkCloudControlDetailsEl {
            major_revision_id: self.major_revision_id,
            name: self.name,
            parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceFrameworkCloudControlDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkCloudControlDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudSecurityComplianceFrameworkCloudControlDetailsElRef {
        CloudSecurityComplianceFrameworkCloudControlDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceFrameworkCloudControlDetailsElRef {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the CloudControl in the format:\n“organizations/{organization}/locations/{location}/cloudControls/{cloud-control}”"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> ListRef<CloudSecurityComplianceFrameworkCloudControlDetailsElParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudSecurityComplianceFrameworkTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudSecurityComplianceFrameworkTimeoutsEl {
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
impl ToListMappable for CloudSecurityComplianceFrameworkTimeoutsEl {
    type O = BlockAssignable<CloudSecurityComplianceFrameworkTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudSecurityComplianceFrameworkTimeoutsEl {}
impl BuildCloudSecurityComplianceFrameworkTimeoutsEl {
    pub fn build(self) -> CloudSecurityComplianceFrameworkTimeoutsEl {
        CloudSecurityComplianceFrameworkTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudSecurityComplianceFrameworkTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudSecurityComplianceFrameworkTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudSecurityComplianceFrameworkTimeoutsElRef {
        CloudSecurityComplianceFrameworkTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudSecurityComplianceFrameworkTimeoutsElRef {
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
struct CloudSecurityComplianceFrameworkDynamic {
    cloud_control_details:
        Option<DynamicBlock<CloudSecurityComplianceFrameworkCloudControlDetailsEl>>,
}
