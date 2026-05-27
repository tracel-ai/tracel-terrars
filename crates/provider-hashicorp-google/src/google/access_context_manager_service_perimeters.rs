use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AccessContextManagerServicePerimetersData {
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
    id: Option<PrimField<String>>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_perimeters: Option<Vec<AccessContextManagerServicePerimetersServicePerimetersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AccessContextManagerServicePerimetersTimeoutsEl>,
    dynamic: AccessContextManagerServicePerimetersDynamic,
}
struct AccessContextManagerServicePerimeters_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessContextManagerServicePerimetersData>,
}
#[derive(Clone)]
pub struct AccessContextManagerServicePerimeters(Rc<AccessContextManagerServicePerimeters_>);
impl AccessContextManagerServicePerimeters {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `service_perimeters`.\n"]
    pub fn set_service_perimeters(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_perimeters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_perimeters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<AccessContextManagerServicePerimetersTimeoutsEl>,
    ) -> Self {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_perimeters` after provisioning.\n"]
    pub fn service_perimeters(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimetersServicePerimetersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_perimeters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimetersTimeoutsElRef {
        AccessContextManagerServicePerimetersTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AccessContextManagerServicePerimeters {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AccessContextManagerServicePerimeters {}
impl ToListMappable for AccessContextManagerServicePerimeters {
    type O = ListRef<AccessContextManagerServicePerimetersRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AccessContextManagerServicePerimeters_ {
    fn extract_resource_type(&self) -> String {
        "google_access_context_manager_service_perimeters".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAccessContextManagerServicePerimeters {
    pub tf_id: String,
    #[doc = "The AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub parent: PrimField<String>,
}
impl BuildAccessContextManagerServicePerimeters {
    pub fn build(self, stack: &mut Stack) -> AccessContextManagerServicePerimeters {
        let out = AccessContextManagerServicePerimeters(Rc::new(
            AccessContextManagerServicePerimeters_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(AccessContextManagerServicePerimetersData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    id: core::default::Default::default(),
                    parent: self.parent,
                    service_perimeters: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AccessContextManagerServicePerimetersRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AccessContextManagerServicePerimetersRef {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_perimeters` after provisioning.\n"]
    pub fn service_perimeters(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimetersServicePerimetersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_perimeters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimetersTimeoutsElRef {
        AccessContextManagerServicePerimetersTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl { # [doc = "Set the field `access_level`.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."] pub fn set_access_level (mut self , v : impl Into < PrimField < String > >) -> Self { self . access_level = Some (v . into ()) ; self } # [doc = "Set the field `resource`.\nA Google Cloud resource that is allowed to egress the perimeter.\nRequests from these resources are allowed to access data outside the perimeter.\nCurrently only projects are allowed. Project format: 'projects/{project_number}'.\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the\ncase of allowing all Google Cloud resources only is not supported."] pub fn set_resource (mut self , v : impl Into < PrimField < String > >) -> Self { self . resource = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl { AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl { access_level : core :: default :: Default :: default () , resource : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef { AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `access_level` after provisioning.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."] pub fn access_level (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.access_level" , self . base)) } # [doc = "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to egress the perimeter.\nRequests from these resources are allowed to access data outside the perimeter.\nCurrently only projects are allowed. Project format: 'projects/{project_number}'.\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the\ncase of allowing all Google Cloud resources only is not supported."] pub fn resource (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.resource" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElDynamic { sources : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl { # [serde (skip_serializing_if = "Option::is_none")] identities : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] identity_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] source_restriction : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] sources : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl {
    #[doc = "Set the field `identities`.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn set_identities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.identities = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `source_restriction`.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn set_source_restriction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_restriction = Some(v.into());
        self
    }
    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl
{}
impl
    BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl
{
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl
    {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            source_restriction: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef { AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identities` after provisioning.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn identities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }
    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_restriction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]    pub fn sources (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { # [doc = "Set the field `method`.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."] pub fn set_method (mut self , v : impl Into < PrimField < String > >) -> Self { self . method = Some (v . into ()) ; self } # [doc = "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn set_permission (mut self , v : impl Into < PrimField < String > >) -> Self { self . permission = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { method : core :: default :: Default :: default () , permission : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `method` after provisioning.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."] pub fn method (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.method" , self . base)) } # [doc = "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn permission (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.permission" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElDynamic { method_selectors : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl { # [serde (skip_serializing_if = "Option::is_none")] service_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] method_selectors : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl { # [doc = "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."] pub fn set_service_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . service_name = Some (v . into ()) ; self } # [doc = "Set the field `method_selectors`.\n"] pub fn set_method_selectors (mut self , v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . method_selectors = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . method_selectors = Some (d) ; } } self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl { AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl { service_name : core :: default :: Default :: default () , method_selectors : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef { AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."] pub fn service_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service_name" , self . base)) } # [doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"] pub fn method_selectors (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.method_selectors" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElDynamic { operations : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl { # [serde (skip_serializing_if = "Option::is_none")] external_resources : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] resources : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] roles : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] operations : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl {
    #[doc = "Set the field `external_resources`.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn set_external_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.external_resources = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'EgressFrom'\nare allowed to perform."]
    pub fn set_roles(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.roles = Some(v.into());
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl
    {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl {
            external_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            roles: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef
    {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef { shared : shared , base : base . to_string () , }
    }
}
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_resources` after provisioning.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn external_resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.external_resources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'EgressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]    pub fn operations (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElDynamic { egress_from : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl >> , egress_to : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl { # [serde (skip_serializing_if = "Option::is_none")] title : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] egress_from : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl > > , # [serde (skip_serializing_if = "Option::is_none")] egress_to : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {
    #[doc = "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_from = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_from = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_to = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {
            title: core::default::Default::default(),
            egress_from: core::default::Default::default(),
            egress_to: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]    pub fn egress_from (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef >{
        ListRef::new(self.shared().clone(), format!("{}.egress_from", self.base))
    }
    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(
        &self,
    ) -> ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.egress_to", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl { # [doc = "Set the field `access_level`.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."] pub fn set_access_level (mut self , v : impl Into < PrimField < String > >) -> Self { self . access_level = Some (v . into ()) ; self } # [doc = "Set the field `resource`.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."] pub fn set_resource (mut self , v : impl Into < PrimField < String > >) -> Self { self . resource = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl { AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl { access_level : core :: default :: Default :: default () , resource : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef { AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `access_level` after provisioning.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."] pub fn access_level (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.access_level" , self . base)) } # [doc = "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."] pub fn resource (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.resource" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElDynamic { sources : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl { # [serde (skip_serializing_if = "Option::is_none")] identities : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] identity_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] sources : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl {
    #[doc = "Set the field `identities`.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn set_identities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.identities = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl
{
    type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl { AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl { identities : core :: default :: Default :: default () , identity_type : core :: default :: Default :: default () , sources : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef { AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef { shared : shared , base : base . to_string () , } } }
impl
    AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn identities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }
    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]    pub fn sources (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { # [doc = "Set the field `method`.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."] pub fn set_method (mut self , v : impl Into < PrimField < String > >) -> Self { self . method = Some (v . into ()) ; self } # [doc = "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn set_permission (mut self , v : impl Into < PrimField < String > >) -> Self { self . permission = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { method : core :: default :: Default :: default () , permission : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `method` after provisioning.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."] pub fn method (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.method" , self . base)) } # [doc = "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn permission (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.permission" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElDynamic { method_selectors : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl { # [serde (skip_serializing_if = "Option::is_none")] service_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] method_selectors : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl { # [doc = "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."] pub fn set_service_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . service_name = Some (v . into ()) ; self } # [doc = "Set the field `method_selectors`.\n"] pub fn set_method_selectors (mut self , v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . method_selectors = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . method_selectors = Some (d) ; } } self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl { AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl { service_name : core :: default :: Default :: default () , method_selectors : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef { AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."] pub fn service_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service_name" , self . base)) } # [doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"] pub fn method_selectors (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.method_selectors" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElDynamic { operations : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl { # [serde (skip_serializing_if = "Option::is_none")] resources : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] roles : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] operations : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl {
    #[doc = "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn set_roles(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.roles = Some(v.into());
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl
{}
impl
    BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl
{
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl
    {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl {
            resources: core::default::Default::default(),
            roles: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef { AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]    pub fn operations (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElDynamic { ingress_from : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl >> , ingress_to : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl { # [serde (skip_serializing_if = "Option::is_none")] title : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ingress_from : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl > > , # [serde (skip_serializing_if = "Option::is_none")] ingress_to : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {
    #[doc = "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_from = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_from = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_to = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {
            title: core::default::Default::default(),
            ingress_from: core::default::Default::default(),
            ingress_to: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]    pub fn ingress_from (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef >{
        ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]    pub fn ingress_to (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef >{
        ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_services: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_restriction: Option<PrimField<bool>>,
}
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {
    #[doc = "Set the field `allowed_services`.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn set_allowed_services(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_services = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_restriction`.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn set_enable_restriction(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_restriction = Some(v.into());
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {
            allowed_services: core::default::Default::default(),
            enable_restriction: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef
    {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_services` after provisioning.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn allowed_services(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_services", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_restriction` after provisioning.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn enable_restriction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_restriction", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElDynamic {
    egress_policies: Option<
        DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl,
        >,
    >,
    ingress_policies: Option<
        DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl,
        >,
    >,
    vpc_accessible_services: Option<
        DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_levels: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_services: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_policies:
        Option<Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_policies: Option<
        Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_accessible_services: Option<
        Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl>,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElSpecElDynamic,
}
impl AccessContextManagerServicePerimetersServicePerimetersElSpecEl {
    #[doc = "Set the field `access_levels`.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn set_access_levels(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.access_levels = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `restricted_services`.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn set_restricted_services(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.restricted_services = Some(v.into());
        self
    }
    #[doc = "Set the field `egress_policies`.\n"]
    pub fn set_egress_policies(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ingress_policies`.\n"]
    pub fn set_ingress_policies(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_accessible_services`.\n"]
    pub fn set_vpc_accessible_services(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_accessible_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_accessible_services = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecEl {
    type O = BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecEl {}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecEl {
            access_levels: core::default::Default::default(),
            resources: core::default::Default::default(),
            restricted_services: core::default::Default::default(),
            egress_policies: core::default::Default::default(),
            ingress_policies: core::default::Default::default(),
            vpc_accessible_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimetersServicePerimetersElSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_levels` after provisioning.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn access_levels(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.access_levels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `restricted_services` after provisioning.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn restricted_services(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.restricted_services", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `egress_policies` after provisioning.\n"]
    pub fn egress_policies(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.egress_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_policies` after provisioning.\n"]
    pub fn ingress_policies(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingress_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_accessible_services` after provisioning.\n"]
    pub fn vpc_accessible_services(
        &self,
    ) -> ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_accessible_services", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl { # [doc = "Set the field `access_level`.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."] pub fn set_access_level (mut self , v : impl Into < PrimField < String > >) -> Self { self . access_level = Some (v . into ()) ; self } # [doc = "Set the field `resource`.\nA Google Cloud resource that is allowed to egress the perimeter.\nRequests from these resources are allowed to access data outside the perimeter.\nCurrently only projects are allowed. Project format: 'projects/{project_number}'.\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the\ncase of allowing all Google Cloud resources only is not supported."] pub fn set_resource (mut self , v : impl Into < PrimField < String > >) -> Self { self . resource = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl { AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl { access_level : core :: default :: Default :: default () , resource : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef { AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `access_level` after provisioning.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."] pub fn access_level (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.access_level" , self . base)) } # [doc = "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to egress the perimeter.\nRequests from these resources are allowed to access data outside the perimeter.\nCurrently only projects are allowed. Project format: 'projects/{project_number}'.\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the\ncase of allowing all Google Cloud resources only is not supported."] pub fn resource (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.resource" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElDynamic { sources : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl { # [serde (skip_serializing_if = "Option::is_none")] identities : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] identity_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] source_restriction : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] sources : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl {
    #[doc = "Set the field `identities`.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn set_identities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.identities = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `source_restriction`.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn set_source_restriction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_restriction = Some(v.into());
        self
    }
    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl
{
    type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl { AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl { identities : core :: default :: Default :: default () , identity_type : core :: default :: Default :: default () , source_restriction : core :: default :: Default :: default () , sources : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef { AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef { shared : shared , base : base . to_string () , } } }
impl
    AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn identities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }
    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_restriction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]    pub fn sources (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { # [doc = "Set the field `method`.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."] pub fn set_method (mut self , v : impl Into < PrimField < String > >) -> Self { self . method = Some (v . into ()) ; self } # [doc = "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn set_permission (mut self , v : impl Into < PrimField < String > >) -> Self { self . permission = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { method : core :: default :: Default :: default () , permission : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `method` after provisioning.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."] pub fn method (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.method" , self . base)) } # [doc = "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn permission (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.permission" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElDynamic { method_selectors : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl { # [serde (skip_serializing_if = "Option::is_none")] service_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] method_selectors : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl { # [doc = "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."] pub fn set_service_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . service_name = Some (v . into ()) ; self } # [doc = "Set the field `method_selectors`.\n"] pub fn set_method_selectors (mut self , v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . method_selectors = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . method_selectors = Some (d) ; } } self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl { AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl { service_name : core :: default :: Default :: default () , method_selectors : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef { AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."] pub fn service_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service_name" , self . base)) } # [doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"] pub fn method_selectors (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.method_selectors" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElDynamic { operations : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl { # [serde (skip_serializing_if = "Option::is_none")] external_resources : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] resources : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] roles : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] operations : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl {
    #[doc = "Set the field `external_resources`.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn set_external_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.external_resources = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'EgressFrom'\nare allowed to perform."]
    pub fn set_roles(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.roles = Some(v.into());
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl
{}
impl
    BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl
{
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl
    {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl {
            external_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            roles: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef { AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_resources` after provisioning.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn external_resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.external_resources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'EgressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]    pub fn operations (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElDynamic { egress_from : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl >> , egress_to : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl { # [serde (skip_serializing_if = "Option::is_none")] title : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] egress_from : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl > > , # [serde (skip_serializing_if = "Option::is_none")] egress_to : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {
    #[doc = "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_from = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_from = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_to = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {
}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {
            title: core::default::Default::default(),
            egress_from: core::default::Default::default(),
            egress_to: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]    pub fn egress_from (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef >{
        ListRef::new(self.shared().clone(), format!("{}.egress_from", self.base))
    }
    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]    pub fn egress_to (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef >{
        ListRef::new(self.shared().clone(), format!("{}.egress_to", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl { # [doc = "Set the field `access_level`.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."] pub fn set_access_level (mut self , v : impl Into < PrimField < String > >) -> Self { self . access_level = Some (v . into ()) ; self } # [doc = "Set the field `resource`.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."] pub fn set_resource (mut self , v : impl Into < PrimField < String > >) -> Self { self . resource = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl { AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl { access_level : core :: default :: Default :: default () , resource : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef { AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `access_level` after provisioning.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."] pub fn access_level (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.access_level" , self . base)) } # [doc = "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."] pub fn resource (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.resource" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElDynamic { sources : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl { # [serde (skip_serializing_if = "Option::is_none")] identities : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] identity_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] sources : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElDynamic , }
impl
    AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl
{
    #[doc = "Set the field `identities`.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn set_identities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.identities = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl { AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl { identities : core :: default :: Default :: default () , identity_type : core :: default :: Default :: default () , sources : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef { AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."] pub fn identities (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.identities" , self . base)) } # [doc = "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"] pub fn identity_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.identity_type" , self . base)) } # [doc = "Get a reference to the value of field `sources` after provisioning.\n"] pub fn sources (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.sources" , self . base)) } }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { # [doc = "Set the field `method`.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."] pub fn set_method (mut self , v : impl Into < PrimField < String > >) -> Self { self . method = Some (v . into ()) ; self } # [doc = "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn set_permission (mut self , v : impl Into < PrimField < String > >) -> Self { self . permission = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { method : core :: default :: Default :: default () , permission : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `method` after provisioning.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."] pub fn method (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.method" , self . base)) } # [doc = "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn permission (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.permission" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElDynamic { method_selectors : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl { # [serde (skip_serializing_if = "Option::is_none")] service_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] method_selectors : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl { # [doc = "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."] pub fn set_service_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . service_name = Some (v . into ()) ; self } # [doc = "Set the field `method_selectors`.\n"] pub fn set_method_selectors (mut self , v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . method_selectors = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . method_selectors = Some (d) ; } } self } }
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl { type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl { AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl { service_name : core :: default :: Default :: default () , method_selectors : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef { AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."] pub fn service_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service_name" , self . base)) } # [doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"] pub fn method_selectors (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.method_selectors" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElDynamic { operations : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl { # [serde (skip_serializing_if = "Option::is_none")] resources : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] roles : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] operations : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl {
    #[doc = "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn set_roles(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.roles = Some(v.into());
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl
{
    type O = BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl { pub fn build (self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl { AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl { resources : core :: default :: Default :: default () , roles : core :: default :: Default :: default () , operations : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef { AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef { shared : shared , base : base . to_string () , } } }
impl
    AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]    pub fn operations (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElDynamic { ingress_from : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl >> , ingress_to : Option < DynamicBlock < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl { # [serde (skip_serializing_if = "Option::is_none")] title : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ingress_from : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl > > , # [serde (skip_serializing_if = "Option::is_none")] ingress_to : Option < Vec < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl > > , dynamic : AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElDynamic , }
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {
    #[doc = "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_from = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_from = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_to = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {
}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {
            title: core::default::Default::default(),
            ingress_from: core::default::Default::default(),
            ingress_to: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]    pub fn ingress_from (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef >{
        ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]    pub fn ingress_to (& self) -> ListRef < AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef >{
        ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_services: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_restriction: Option<PrimField<bool>>,
}
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl {
    #[doc = "Set the field `allowed_services`.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn set_allowed_services(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_services = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_restriction`.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn set_enable_restriction(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_restriction = Some(v.into());
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl
{}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl
    {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl {
            allowed_services: core::default::Default::default(),
            enable_restriction: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef
    {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_services` after provisioning.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn allowed_services(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_services", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_restriction` after provisioning.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn enable_restriction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_restriction", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElDynamic {
    egress_policies: Option<
        DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl,
        >,
    >,
    ingress_policies: Option<
        DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl,
        >,
    >,
    vpc_accessible_services: Option<
        DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_levels: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_services: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_policies: Option<
        Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_policies: Option<
        Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_accessible_services: Option<
        Vec<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl,
        >,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElStatusElDynamic,
}
impl AccessContextManagerServicePerimetersServicePerimetersElStatusEl {
    #[doc = "Set the field `access_levels`.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn set_access_levels(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.access_levels = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `restricted_services`.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn set_restricted_services(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.restricted_services = Some(v.into());
        self
    }
    #[doc = "Set the field `egress_policies`.\n"]
    pub fn set_egress_policies(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ingress_policies`.\n"]
    pub fn set_ingress_policies(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_accessible_services`.\n"]
    pub fn set_vpc_accessible_services(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_accessible_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_accessible_services = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusEl {
    type O = BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusEl {}
impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusEl {
            access_levels: core::default::Default::default(),
            resources: core::default::Default::default(),
            restricted_services: core::default::Default::default(),
            egress_policies: core::default::Default::default(),
            ingress_policies: core::default::Default::default(),
            vpc_accessible_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimetersServicePerimetersElStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_levels` after provisioning.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn access_levels(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.access_levels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `restricted_services` after provisioning.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn restricted_services(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.restricted_services", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `egress_policies` after provisioning.\n"]
    pub fn egress_policies(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.egress_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_accessible_services` after provisioning.\n"]
    pub fn vpc_accessible_services(
        &self,
    ) -> ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_accessible_services", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElDynamic {
    spec: Option<DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElSpecEl>>,
    status: Option<DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElStatusEl>>,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    perimeter_type: Option<PrimField<String>>,
    title: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_explicit_dry_run_spec: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusEl>>,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElDynamic,
}
impl AccessContextManagerServicePerimetersServicePerimetersEl {
    #[doc = "Set the field `description`.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `perimeter_type`.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn set_perimeter_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.perimeter_type = Some(v.into());
        self
    }
    #[doc = "Set the field `use_explicit_dry_run_spec`.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn set_use_explicit_dry_run_spec(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_explicit_dry_run_spec = Some(v.into());
        self
    }
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElStatusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.status = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.status = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimetersServicePerimetersEl {
    type O = BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersServicePerimetersEl {
    #[doc = "Resource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub name: PrimField<String>,
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: PrimField<String>,
}
impl BuildAccessContextManagerServicePerimetersServicePerimetersEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersEl {
        AccessContextManagerServicePerimetersServicePerimetersEl {
            description: core::default::Default::default(),
            name: self.name,
            perimeter_type: core::default::Default::default(),
            title: self.title,
            use_explicit_dry_run_spec: core::default::Default::default(),
            spec: core::default::Default::default(),
            status: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersServicePerimetersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersServicePerimetersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElRef {
        AccessContextManagerServicePerimetersServicePerimetersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimetersServicePerimetersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the AccessPolicy was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `perimeter_type` after provisioning.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn perimeter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.perimeter_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the Policy."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the AccessPolicy was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `use_explicit_dry_run_spec` after provisioning.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn use_explicit_dry_run_spec(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_explicit_dry_run_spec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimetersServicePerimetersElStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimetersTimeoutsEl {
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
impl ToListMappable for AccessContextManagerServicePerimetersTimeoutsEl {
    type O = BlockAssignable<AccessContextManagerServicePerimetersTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimetersTimeoutsEl {}
impl BuildAccessContextManagerServicePerimetersTimeoutsEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersTimeoutsEl {
        AccessContextManagerServicePerimetersTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimetersTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimetersTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimetersTimeoutsElRef {
        AccessContextManagerServicePerimetersTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimetersTimeoutsElRef {
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
struct AccessContextManagerServicePerimetersDynamic {
    service_perimeters:
        Option<DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersEl>>,
}
