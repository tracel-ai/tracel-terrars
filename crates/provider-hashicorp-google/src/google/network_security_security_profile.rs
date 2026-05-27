use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkSecuritySecurityProfileData {
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
    parent: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_intercept_profile: Option<Vec<NetworkSecuritySecurityProfileCustomInterceptProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_mirroring_profile: Option<Vec<NetworkSecuritySecurityProfileCustomMirroringProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threat_prevention_profile: Option<Vec<NetworkSecuritySecurityProfileThreatPreventionProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkSecuritySecurityProfileTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_filtering_profile: Option<Vec<NetworkSecuritySecurityProfileUrlFilteringProfileEl>>,
    dynamic: NetworkSecuritySecurityProfileDynamic,
}
struct NetworkSecuritySecurityProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkSecuritySecurityProfileData>,
}
#[derive(Clone)]
pub struct NetworkSecuritySecurityProfile(Rc<NetworkSecuritySecurityProfile_>);
impl NetworkSecuritySecurityProfile {
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
    #[doc = "Set the field `description`.\nAn optional description of the security profile. The Max length is 512 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nA map of key/value label pairs to assign to the resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location of the security profile.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nThe name of the parent this security profile belongs to.\nFormat: 'organizations/{organization_id}' or 'projects/{project_id}'."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_intercept_profile`.\n"]
    pub fn set_custom_intercept_profile(
        self,
        v: impl Into<BlockAssignable<NetworkSecuritySecurityProfileCustomInterceptProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_intercept_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_intercept_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom_mirroring_profile`.\n"]
    pub fn set_custom_mirroring_profile(
        self,
        v: impl Into<BlockAssignable<NetworkSecuritySecurityProfileCustomMirroringProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_mirroring_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_mirroring_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `threat_prevention_profile`.\n"]
    pub fn set_threat_prevention_profile(
        self,
        v: impl Into<BlockAssignable<NetworkSecuritySecurityProfileThreatPreventionProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().threat_prevention_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.threat_prevention_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkSecuritySecurityProfileTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `url_filtering_profile`.\n"]
    pub fn set_url_filtering_profile(
        self,
        v: impl Into<BlockAssignable<NetworkSecuritySecurityProfileUrlFilteringProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().url_filtering_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.url_filtering_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the security profile was created in UTC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of the security profile. The Max length is 512 characters."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields,\nand may be sent on update and delete requests to ensure the client has an up-to-date\nvalue before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA map of key/value label pairs to assign to the resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the security profile.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the security profile resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe name of the parent this security profile belongs to.\nFormat: 'organizations/{organization_id}' or 'projects/{project_id}'."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of security profile. Possible values: [\"THREAT_PREVENTION\", \"URL_FILTERING\", \"CUSTOM_MIRRORING\", \"CUSTOM_INTERCEPT\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the security profile was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_intercept_profile` after provisioning.\n"]
    pub fn custom_intercept_profile(
        &self,
    ) -> ListRef<NetworkSecuritySecurityProfileCustomInterceptProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_intercept_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_mirroring_profile` after provisioning.\n"]
    pub fn custom_mirroring_profile(
        &self,
    ) -> ListRef<NetworkSecuritySecurityProfileCustomMirroringProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_mirroring_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `threat_prevention_profile` after provisioning.\n"]
    pub fn threat_prevention_profile(
        &self,
    ) -> ListRef<NetworkSecuritySecurityProfileThreatPreventionProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.threat_prevention_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecuritySecurityProfileTimeoutsElRef {
        NetworkSecuritySecurityProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url_filtering_profile` after provisioning.\n"]
    pub fn url_filtering_profile(
        &self,
    ) -> ListRef<NetworkSecuritySecurityProfileUrlFilteringProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.url_filtering_profile", self.extract_ref()),
        )
    }
}
impl Referable for NetworkSecuritySecurityProfile {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkSecuritySecurityProfile {}
impl ToListMappable for NetworkSecuritySecurityProfile {
    type O = ListRef<NetworkSecuritySecurityProfileRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkSecuritySecurityProfile_ {
    fn extract_resource_type(&self) -> String {
        "google_network_security_security_profile".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkSecuritySecurityProfile {
    pub tf_id: String,
    #[doc = "The name of the security profile resource."]
    pub name: PrimField<String>,
    #[doc = "The type of security profile. Possible values: [\"THREAT_PREVENTION\", \"URL_FILTERING\", \"CUSTOM_MIRRORING\", \"CUSTOM_INTERCEPT\"]"]
    pub type_: PrimField<String>,
}
impl BuildNetworkSecuritySecurityProfile {
    pub fn build(self, stack: &mut Stack) -> NetworkSecuritySecurityProfile {
        let out = NetworkSecuritySecurityProfile(Rc::new(NetworkSecuritySecurityProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkSecuritySecurityProfileData {
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
                parent: core::default::Default::default(),
                type_: self.type_,
                custom_intercept_profile: core::default::Default::default(),
                custom_mirroring_profile: core::default::Default::default(),
                threat_prevention_profile: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                url_filtering_profile: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkSecuritySecurityProfileRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecuritySecurityProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkSecuritySecurityProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the security profile was created in UTC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of the security profile. The Max length is 512 characters."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields,\nand may be sent on update and delete requests to ensure the client has an up-to-date\nvalue before proceeding."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA map of key/value label pairs to assign to the resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the security profile.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the security profile resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe name of the parent this security profile belongs to.\nFormat: 'organizations/{organization_id}' or 'projects/{project_id}'."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of security profile. Possible values: [\"THREAT_PREVENTION\", \"URL_FILTERING\", \"CUSTOM_MIRRORING\", \"CUSTOM_INTERCEPT\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the security profile was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_intercept_profile` after provisioning.\n"]
    pub fn custom_intercept_profile(
        &self,
    ) -> ListRef<NetworkSecuritySecurityProfileCustomInterceptProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_intercept_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_mirroring_profile` after provisioning.\n"]
    pub fn custom_mirroring_profile(
        &self,
    ) -> ListRef<NetworkSecuritySecurityProfileCustomMirroringProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_mirroring_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `threat_prevention_profile` after provisioning.\n"]
    pub fn threat_prevention_profile(
        &self,
    ) -> ListRef<NetworkSecuritySecurityProfileThreatPreventionProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.threat_prevention_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecuritySecurityProfileTimeoutsElRef {
        NetworkSecuritySecurityProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url_filtering_profile` after provisioning.\n"]
    pub fn url_filtering_profile(
        &self,
    ) -> ListRef<NetworkSecuritySecurityProfileUrlFilteringProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.url_filtering_profile", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfileCustomInterceptProfileEl {
    intercept_endpoint_group: PrimField<String>,
}
impl NetworkSecuritySecurityProfileCustomInterceptProfileEl {}
impl ToListMappable for NetworkSecuritySecurityProfileCustomInterceptProfileEl {
    type O = BlockAssignable<NetworkSecuritySecurityProfileCustomInterceptProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecuritySecurityProfileCustomInterceptProfileEl {
    #[doc = "The Intercept Endpoint Group to which matching traffic should be intercepted.\nFormat: projects/{project_id}/locations/global/interceptEndpointGroups/{endpoint_group_id}"]
    pub intercept_endpoint_group: PrimField<String>,
}
impl BuildNetworkSecuritySecurityProfileCustomInterceptProfileEl {
    pub fn build(self) -> NetworkSecuritySecurityProfileCustomInterceptProfileEl {
        NetworkSecuritySecurityProfileCustomInterceptProfileEl {
            intercept_endpoint_group: self.intercept_endpoint_group,
        }
    }
}
pub struct NetworkSecuritySecurityProfileCustomInterceptProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecuritySecurityProfileCustomInterceptProfileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecuritySecurityProfileCustomInterceptProfileElRef {
        NetworkSecuritySecurityProfileCustomInterceptProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecuritySecurityProfileCustomInterceptProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `intercept_endpoint_group` after provisioning.\nThe Intercept Endpoint Group to which matching traffic should be intercepted.\nFormat: projects/{project_id}/locations/global/interceptEndpointGroups/{endpoint_group_id}"]
    pub fn intercept_endpoint_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.intercept_endpoint_group", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfileCustomMirroringProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mirroring_deployment_groups: Option<ListField<PrimField<String>>>,
    mirroring_endpoint_group: PrimField<String>,
}
impl NetworkSecuritySecurityProfileCustomMirroringProfileEl {
    #[doc = "Set the field `mirroring_deployment_groups`.\nThe target downstream Mirroring Deployment Groups.\nThis field is used for Packet Broker mirroring endpoint groups to specify\nthe deployment groups that the packet should be mirrored to by the broker.\nFormat: projects/{project_id}/locations/global/mirroringDeploymentGroups/{deployment_group_id}"]
    pub fn set_mirroring_deployment_groups(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.mirroring_deployment_groups = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkSecuritySecurityProfileCustomMirroringProfileEl {
    type O = BlockAssignable<NetworkSecuritySecurityProfileCustomMirroringProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecuritySecurityProfileCustomMirroringProfileEl {
    #[doc = "The target Mirroring Endpoint Group.\nWhen a mirroring rule with this security profile attached matches a packet,\na replica will be mirrored to the location-local target in this group.\nFormat: projects/{project_id}/locations/global/mirroringEndpointGroups/{endpoint_group_id}"]
    pub mirroring_endpoint_group: PrimField<String>,
}
impl BuildNetworkSecuritySecurityProfileCustomMirroringProfileEl {
    pub fn build(self) -> NetworkSecuritySecurityProfileCustomMirroringProfileEl {
        NetworkSecuritySecurityProfileCustomMirroringProfileEl {
            mirroring_deployment_groups: core::default::Default::default(),
            mirroring_endpoint_group: self.mirroring_endpoint_group,
        }
    }
}
pub struct NetworkSecuritySecurityProfileCustomMirroringProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecuritySecurityProfileCustomMirroringProfileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecuritySecurityProfileCustomMirroringProfileElRef {
        NetworkSecuritySecurityProfileCustomMirroringProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecuritySecurityProfileCustomMirroringProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mirroring_deployment_groups` after provisioning.\nThe target downstream Mirroring Deployment Groups.\nThis field is used for Packet Broker mirroring endpoint groups to specify\nthe deployment groups that the packet should be mirrored to by the broker.\nFormat: projects/{project_id}/locations/global/mirroringDeploymentGroups/{deployment_group_id}"]
    pub fn mirroring_deployment_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mirroring_deployment_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mirroring_endpoint_group` after provisioning.\nThe target Mirroring Endpoint Group.\nWhen a mirroring rule with this security profile attached matches a packet,\na replica will be mirrored to the location-local target in this group.\nFormat: projects/{project_id}/locations/global/mirroringEndpointGroups/{endpoint_group_id}"]
    pub fn mirroring_endpoint_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mirroring_endpoint_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mirroring_endpoint_group_type` after provisioning.\nThe type of the mirroring endpoint group this profile is attached to.\nPossible values:\nDIRECT\nBROKER"]
    pub fn mirroring_endpoint_group_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mirroring_endpoint_group_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesEl {
    action: PrimField<String>,
    protocol: PrimField<String>,
}
impl NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesEl {}
impl ToListMappable
    for NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesEl
{
    type O = BlockAssignable<
        NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesEl {
    #[doc = "Threat action override. For some threat types, only a subset of actions applies. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub action: PrimField<String>,
    #[doc = "Required protocol to match. Possible values: [\"SMTP\", \"SMB\", \"POP3\", \"IMAP\", \"HTTP2\", \"HTTP\", \"FTP\"]"]
    pub protocol: PrimField<String>,
}
impl BuildNetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesEl {
    pub fn build(
        self,
    ) -> NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesEl {
        NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesEl {
            action: self.action,
            protocol: self.protocol,
        }
    }
}
pub struct NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesElRef {
        NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThreat action override. For some threat types, only a subset of actions applies. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nRequired protocol to match. Possible values: [\"SMTP\", \"SMB\", \"POP3\", \"IMAP\", \"HTTP2\", \"HTTP\", \"FTP\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesEl {
    action: PrimField<String>,
    severity: PrimField<String>,
}
impl NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesEl {}
impl ToListMappable for NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesEl {
    type O =
        BlockAssignable<NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesEl {
    #[doc = "Threat action override. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub action: PrimField<String>,
    #[doc = "Severity level to match. Possible values: [\"CRITICAL\", \"HIGH\", \"INFORMATIONAL\", \"LOW\", \"MEDIUM\"]"]
    pub severity: PrimField<String>,
}
impl BuildNetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesEl {
    pub fn build(
        self,
    ) -> NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesEl {
        NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesEl {
            action: self.action,
            severity: self.severity,
        }
    }
}
pub struct NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesElRef {
        NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThreat action override. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\nSeverity level to match. Possible values: [\"CRITICAL\", \"HIGH\", \"INFORMATIONAL\", \"LOW\", \"MEDIUM\"]"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesEl {
    action: PrimField<String>,
    threat_id: PrimField<String>,
}
impl NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesEl {}
impl ToListMappable for NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesEl {
    type O =
        BlockAssignable<NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesEl {
    #[doc = "Threat action. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub action: PrimField<String>,
    #[doc = "Vendor-specific ID of a threat to override."]
    pub threat_id: PrimField<String>,
}
impl BuildNetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesEl {
    pub fn build(self) -> NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesEl {
        NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesEl {
            action: self.action,
            threat_id: self.threat_id,
        }
    }
}
pub struct NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesElRef {
        NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThreat action. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `threat_id` after provisioning.\nVendor-specific ID of a threat to override."]
    pub fn threat_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.threat_id", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of threat."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecuritySecurityProfileThreatPreventionProfileElDynamic {
    antivirus_overrides: Option<
        DynamicBlock<NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesEl>,
    >,
    severity_overrides: Option<
        DynamicBlock<NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesEl>,
    >,
    threat_overrides: Option<
        DynamicBlock<NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesEl>,
    >,
}
#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfileThreatPreventionProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    antivirus_overrides:
        Option<Vec<NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity_overrides:
        Option<Vec<NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threat_overrides:
        Option<Vec<NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesEl>>,
    dynamic: NetworkSecuritySecurityProfileThreatPreventionProfileElDynamic,
}
impl NetworkSecuritySecurityProfileThreatPreventionProfileEl {
    #[doc = "Set the field `antivirus_overrides`.\n"]
    pub fn set_antivirus_overrides(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecuritySecurityProfileThreatPreventionProfileElAntivirusOverridesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.antivirus_overrides = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.antivirus_overrides = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `severity_overrides`.\n"]
    pub fn set_severity_overrides(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecuritySecurityProfileThreatPreventionProfileElSeverityOverridesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.severity_overrides = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.severity_overrides = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `threat_overrides`.\n"]
    pub fn set_threat_overrides(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkSecuritySecurityProfileThreatPreventionProfileElThreatOverridesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.threat_overrides = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_overrides = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecuritySecurityProfileThreatPreventionProfileEl {
    type O = BlockAssignable<NetworkSecuritySecurityProfileThreatPreventionProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecuritySecurityProfileThreatPreventionProfileEl {}
impl BuildNetworkSecuritySecurityProfileThreatPreventionProfileEl {
    pub fn build(self) -> NetworkSecuritySecurityProfileThreatPreventionProfileEl {
        NetworkSecuritySecurityProfileThreatPreventionProfileEl {
            antivirus_overrides: core::default::Default::default(),
            severity_overrides: core::default::Default::default(),
            threat_overrides: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecuritySecurityProfileThreatPreventionProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecuritySecurityProfileThreatPreventionProfileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecuritySecurityProfileThreatPreventionProfileElRef {
        NetworkSecuritySecurityProfileThreatPreventionProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecuritySecurityProfileThreatPreventionProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfileTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkSecuritySecurityProfileTimeoutsEl {
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
impl ToListMappable for NetworkSecuritySecurityProfileTimeoutsEl {
    type O = BlockAssignable<NetworkSecuritySecurityProfileTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecuritySecurityProfileTimeoutsEl {}
impl BuildNetworkSecuritySecurityProfileTimeoutsEl {
    pub fn build(self) -> NetworkSecuritySecurityProfileTimeoutsEl {
        NetworkSecuritySecurityProfileTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecuritySecurityProfileTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecuritySecurityProfileTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkSecuritySecurityProfileTimeoutsElRef {
        NetworkSecuritySecurityProfileTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecuritySecurityProfileTimeoutsElRef {
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
pub struct NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersEl {
    filtering_action: PrimField<String>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    urls: Option<ListField<PrimField<String>>>,
}
impl NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersEl {
    #[doc = "Set the field `urls`.\nA list of domain matcher strings that a domain name gets compared with to determine if the filter is applicable.\nA domain name must match with at least one of the strings in the list for a filter to be applicable."]
    pub fn set_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.urls = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersEl {
    type O = BlockAssignable<NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersEl {
    #[doc = "The action to take when the filter is applied. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub filtering_action: PrimField<String>,
    #[doc = "The priority of the filter within the URL filtering profile.\nMust be an integer from 0 and 2147483647, inclusive. Lower integers indicate higher priorities.\nThe priority of a filter must be unique within a URL filtering profile."]
    pub priority: PrimField<f64>,
}
impl BuildNetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersEl {
    pub fn build(self) -> NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersEl {
        NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersEl {
            filtering_action: self.filtering_action,
            priority: self.priority,
            urls: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersElRef {
        NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filtering_action` after provisioning.\nThe action to take when the filter is applied. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub fn filtering_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filtering_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority of the filter within the URL filtering profile.\nMust be an integer from 0 and 2147483647, inclusive. Lower integers indicate higher priorities.\nThe priority of a filter must be unique within a URL filtering profile."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `urls` after provisioning.\nA list of domain matcher strings that a domain name gets compared with to determine if the filter is applicable.\nA domain name must match with at least one of the strings in the list for a filter to be applicable."]
    pub fn urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.urls", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkSecuritySecurityProfileUrlFilteringProfileElDynamic {
    url_filters:
        Option<DynamicBlock<NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersEl>>,
}
#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfileUrlFilteringProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    url_filters: Option<Vec<NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersEl>>,
    dynamic: NetworkSecuritySecurityProfileUrlFilteringProfileElDynamic,
}
impl NetworkSecuritySecurityProfileUrlFilteringProfileEl {
    #[doc = "Set the field `url_filters`.\n"]
    pub fn set_url_filters(
        mut self,
        v: impl Into<BlockAssignable<NetworkSecuritySecurityProfileUrlFilteringProfileElUrlFiltersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_filters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkSecuritySecurityProfileUrlFilteringProfileEl {
    type O = BlockAssignable<NetworkSecuritySecurityProfileUrlFilteringProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecuritySecurityProfileUrlFilteringProfileEl {}
impl BuildNetworkSecuritySecurityProfileUrlFilteringProfileEl {
    pub fn build(self) -> NetworkSecuritySecurityProfileUrlFilteringProfileEl {
        NetworkSecuritySecurityProfileUrlFilteringProfileEl {
            url_filters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkSecuritySecurityProfileUrlFilteringProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecuritySecurityProfileUrlFilteringProfileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecuritySecurityProfileUrlFilteringProfileElRef {
        NetworkSecuritySecurityProfileUrlFilteringProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecuritySecurityProfileUrlFilteringProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct NetworkSecuritySecurityProfileDynamic {
    custom_intercept_profile:
        Option<DynamicBlock<NetworkSecuritySecurityProfileCustomInterceptProfileEl>>,
    custom_mirroring_profile:
        Option<DynamicBlock<NetworkSecuritySecurityProfileCustomMirroringProfileEl>>,
    threat_prevention_profile:
        Option<DynamicBlock<NetworkSecuritySecurityProfileThreatPreventionProfileEl>>,
    url_filtering_profile:
        Option<DynamicBlock<NetworkSecuritySecurityProfileUrlFilteringProfileEl>>,
}
