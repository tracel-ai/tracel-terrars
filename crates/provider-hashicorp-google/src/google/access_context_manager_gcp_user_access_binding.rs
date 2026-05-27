use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AccessContextManagerGcpUserAccessBindingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_levels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    group_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    organization_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scoped_access_settings:
        Option<Vec<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_settings: Option<Vec<AccessContextManagerGcpUserAccessBindingSessionSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AccessContextManagerGcpUserAccessBindingTimeoutsEl>,
    dynamic: AccessContextManagerGcpUserAccessBindingDynamic,
}
struct AccessContextManagerGcpUserAccessBinding_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessContextManagerGcpUserAccessBindingData>,
}
#[derive(Clone)]
pub struct AccessContextManagerGcpUserAccessBinding(Rc<AccessContextManagerGcpUserAccessBinding_>);
impl AccessContextManagerGcpUserAccessBinding {
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
    #[doc = "Set the field `access_levels`.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn set_access_levels(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().access_levels = Some(v.into());
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
    #[doc = "Set the field `scoped_access_settings`.\n"]
    pub fn set_scoped_access_settings(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scoped_access_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scoped_access_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `session_settings`.\n"]
    pub fn set_session_settings(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerGcpUserAccessBindingSessionSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().session_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.session_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<AccessContextManagerGcpUserAccessBindingTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `access_levels` after provisioning.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn access_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_levels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_key` after provisioning.\nRequired. Immutable. Google Group id whose members are subject to this binding's restrictions. See \"id\" in the G Suite Directory API's Groups resource. If a group's email address/alias is changed, this resource will continue to point at the changed group. This field does not accept group email addresses or aliases. Example: \"01d520gv4vjcrht\""]
    pub fn group_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by RFC 3986 Section 2.3). Should not be specified by the client during creation. Example: \"organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nRequired. ID of the parent organization."]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scoped_access_settings` after provisioning.\n"]
    pub fn scoped_access_settings(
        &self,
    ) -> ListRef<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scoped_access_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_settings` after provisioning.\n"]
    pub fn session_settings(
        &self,
    ) -> ListRef<AccessContextManagerGcpUserAccessBindingSessionSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.session_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
        AccessContextManagerGcpUserAccessBindingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AccessContextManagerGcpUserAccessBinding {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AccessContextManagerGcpUserAccessBinding {}
impl ToListMappable for AccessContextManagerGcpUserAccessBinding {
    type O = ListRef<AccessContextManagerGcpUserAccessBindingRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AccessContextManagerGcpUserAccessBinding_ {
    fn extract_resource_type(&self) -> String {
        "google_access_context_manager_gcp_user_access_binding".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAccessContextManagerGcpUserAccessBinding {
    pub tf_id: String,
    #[doc = "Required. Immutable. Google Group id whose members are subject to this binding's restrictions. See \"id\" in the G Suite Directory API's Groups resource. If a group's email address/alias is changed, this resource will continue to point at the changed group. This field does not accept group email addresses or aliases. Example: \"01d520gv4vjcrht\""]
    pub group_key: PrimField<String>,
    #[doc = "Required. ID of the parent organization."]
    pub organization_id: PrimField<String>,
}
impl BuildAccessContextManagerGcpUserAccessBinding {
    pub fn build(self, stack: &mut Stack) -> AccessContextManagerGcpUserAccessBinding {
        let out = AccessContextManagerGcpUserAccessBinding(Rc::new(
            AccessContextManagerGcpUserAccessBinding_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(AccessContextManagerGcpUserAccessBindingData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    access_levels: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    group_key: self.group_key,
                    id: core::default::Default::default(),
                    organization_id: self.organization_id,
                    scoped_access_settings: core::default::Default::default(),
                    session_settings: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AccessContextManagerGcpUserAccessBindingRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerGcpUserAccessBindingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AccessContextManagerGcpUserAccessBindingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_levels` after provisioning.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn access_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_levels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_key` after provisioning.\nRequired. Immutable. Google Group id whose members are subject to this binding's restrictions. See \"id\" in the G Suite Directory API's Groups resource. If a group's email address/alias is changed, this resource will continue to point at the changed group. This field does not accept group email addresses or aliases. Example: \"01d520gv4vjcrht\""]
    pub fn group_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by RFC 3986 Section 2.3). Should not be specified by the client during creation. Example: \"organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nRequired. ID of the parent organization."]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scoped_access_settings` after provisioning.\n"]
    pub fn scoped_access_settings(
        &self,
    ) -> ListRef<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scoped_access_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_settings` after provisioning.\n"]
    pub fn session_settings(
        &self,
    ) -> ListRef<AccessContextManagerGcpUserAccessBindingSessionSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.session_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
        AccessContextManagerGcpUserAccessBindingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max_inactivity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_length: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_length_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_reauth_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_oidc_max_age: Option<PrimField<bool>>,
}
impl
    AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsEl
{
    #[doc = "Set the field `max_inactivity`.\nOptional. How long a user is allowed to take between actions before a new access token must be issued. Only set for Google Cloud apps."]
    pub fn set_max_inactivity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_inactivity = Some(v.into());
        self
    }
    #[doc = "Set the field `session_length`.\nOptional. The session length. Setting this field to zero is equal to disabling session. Also can set infinite session by flipping the enabled bit to false below. If useOidcMaxAge is true, for OIDC apps, the session length will be the minimum of this field and OIDC max_age param."]
    pub fn set_session_length(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_length = Some(v.into());
        self
    }
    #[doc = "Set the field `session_length_enabled`.\nOptional. This field enables or disables Google Cloud session length. When false, all fields set above will be disregarded and the session length is basically infinite."]
    pub fn set_session_length_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.session_length_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `session_reauth_method`.\nOptional. The session challenges proposed to users when the Google Cloud session length is up. Possible values: [\"LOGIN\", \"SECURITY_KEY\", \"PASSWORD\"]"]
    pub fn set_session_reauth_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_reauth_method = Some(v.into());
        self
    }
    #[doc = "Set the field `use_oidc_max_age`.\nOptional. Only useful for OIDC apps. When false, the OIDC max_age param, if passed in the authentication request will be ignored. When true, the re-auth period will be the minimum of the sessionLength field and the max_age OIDC param."]
    pub fn set_use_oidc_max_age(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_oidc_max_age = Some(v.into());
        self
    }
}
impl ToListMappable for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsEl { type O = BlockAssignable < AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsEl
{}
impl BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsEl { pub fn build (self) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsEl { AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsEl { max_inactivity : core :: default :: Default :: default () , session_length : core :: default :: Default :: default () , session_length_enabled : core :: default :: Default :: default () , session_reauth_method : core :: default :: Default :: default () , use_oidc_max_age : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsElRef { AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max_inactivity` after provisioning.\nOptional. How long a user is allowed to take between actions before a new access token must be issued. Only set for Google Cloud apps."] pub fn max_inactivity (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_inactivity" , self . base)) } # [doc = "Get a reference to the value of field `session_length` after provisioning.\nOptional. The session length. Setting this field to zero is equal to disabling session. Also can set infinite session by flipping the enabled bit to false below. If useOidcMaxAge is true, for OIDC apps, the session length will be the minimum of this field and OIDC max_age param."] pub fn session_length (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.session_length" , self . base)) } # [doc = "Get a reference to the value of field `session_length_enabled` after provisioning.\nOptional. This field enables or disables Google Cloud session length. When false, all fields set above will be disregarded and the session length is basically infinite."] pub fn session_length_enabled (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.session_length_enabled" , self . base)) } # [doc = "Get a reference to the value of field `session_reauth_method` after provisioning.\nOptional. The session challenges proposed to users when the Google Cloud session length is up. Possible values: [\"LOGIN\", \"SECURITY_KEY\", \"PASSWORD\"]"] pub fn session_reauth_method (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.session_reauth_method" , self . base)) } # [doc = "Get a reference to the value of field `use_oidc_max_age` after provisioning.\nOptional. Only useful for OIDC apps. When false, the OIDC max_age param, if passed in the authentication request will be ignored. When true, the re-auth period will be the minimum of the sessionLength field and the max_age OIDC param."] pub fn use_oidc_max_age (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.use_oidc_max_age" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElDynamic { session_settings : Option < DynamicBlock < AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] access_levels : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] session_settings : Option < Vec < AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsEl > > , dynamic : AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElDynamic , }
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsEl {
    #[doc = "Set the field `access_levels`.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn set_access_levels(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.access_levels = Some(v.into());
        self
    }
    #[doc = "Set the field `session_settings`.\n"]
    pub fn set_session_settings(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.session_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.session_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsEl
{
    type O = BlockAssignable<
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsEl {}
impl BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsEl {
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsEl {
            access_levels: core::default::Default::default(),
            session_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElRef {
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_levels` after provisioning.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn access_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_levels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_settings` after provisioning.\n"]    pub fn session_settings (& self) -> ListRef < AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElSessionSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.session_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_levels: Option<ListField<PrimField<String>>>,
}
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsEl {
    #[doc = "Set the field `access_levels`.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn set_access_levels(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.access_levels = Some(v.into());
        self
    }
}
impl ToListMappable
    for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsEl
{
    type O = BlockAssignable<
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsEl {}
impl BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsEl {
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsEl {
            access_levels: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsElRef {
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_levels` after provisioning.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn access_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_levels", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationEl { # [doc = "Set the field `client_id`.\nThe OAuth client ID of the application."] pub fn set_client_id (mut self , v : impl Into < PrimField < String > >) -> Self { self . client_id = Some (v . into ()) ; self } # [doc = "Set the field `name`.\nThe name of the application. Example: \"Cloud Console\""] pub fn set_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . name = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationEl { type O = BlockAssignable < AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationEl
{}
impl BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationEl { pub fn build (self) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationEl { AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationEl { client_id : core :: default :: Default :: default () , name : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationElRef { AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `client_id` after provisioning.\nThe OAuth client ID of the application."] pub fn client_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.client_id" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the application. Example: \"Cloud Console\""] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElDynamic { restricted_client_application : Option < DynamicBlock < AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeEl { # [serde (skip_serializing_if = "Option::is_none")] restricted_client_application : Option < Vec < AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationEl > > , dynamic : AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElDynamic , }
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeEl {
    #[doc = "Set the field `restricted_client_application`.\n"]
    pub fn set_restricted_client_application(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.restricted_client_application = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.restricted_client_application = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeEl
{
    type O = BlockAssignable<
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeEl
{}
impl BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeEl {
    pub fn build(
        self,
    ) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeEl {
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeEl {
            restricted_client_application: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRef {
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `restricted_client_application` after provisioning.\n"]    pub fn restricted_client_application (& self) -> ListRef < AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRestrictedClientApplicationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.restricted_client_application", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElDynamic {
    client_scope: Option<
        DynamicBlock<
            AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_scope: Option<
        Vec<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeEl>,
    >,
    dynamic: AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElDynamic,
}
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeEl {
    #[doc = "Set the field `client_scope`.\n"]
    pub fn set_client_scope(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_scope = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_scope = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeEl {
    type O = BlockAssignable<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeEl {}
impl BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeEl {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeEl {
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeEl {
            client_scope: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElRef {
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_scope` after provisioning.\n"]
    pub fn client_scope(
        &self,
    ) -> ListRef<
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElClientScopeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.client_scope", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDynamic {
    active_settings: Option<
        DynamicBlock<
            AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsEl,
        >,
    >,
    dry_run_settings: Option<
        DynamicBlock<
            AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsEl,
        >,
    >,
    scope:
        Option<DynamicBlock<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeEl>>,
}
#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_settings:
        Option<Vec<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dry_run_settings:
        Option<Vec<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Vec<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeEl>>,
    dynamic: AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDynamic,
}
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsEl {
    #[doc = "Set the field `active_settings`.\n"]
    pub fn set_active_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.active_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.active_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dry_run_settings`.\n"]
    pub fn set_dry_run_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dry_run_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dry_run_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(
        mut self,
        v: impl Into<
            BlockAssignable<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scope = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scope = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsEl {
    type O = BlockAssignable<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsEl {}
impl BuildAccessContextManagerGcpUserAccessBindingScopedAccessSettingsEl {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsEl {
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsEl {
            active_settings: core::default::Default::default(),
            dry_run_settings: core::default::Default::default(),
            scope: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElRef {
        AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active_settings` after provisioning.\n"]
    pub fn active_settings(
        &self,
    ) -> ListRef<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElActiveSettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.active_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dry_run_settings` after provisioning.\n"]
    pub fn dry_run_settings(
        &self,
    ) -> ListRef<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElDryRunSettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dry_run_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(
        &self,
    ) -> ListRef<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsElScopeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBindingSessionSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_inactivity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_length: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_length_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_reauth_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_oidc_max_age: Option<PrimField<bool>>,
}
impl AccessContextManagerGcpUserAccessBindingSessionSettingsEl {
    #[doc = "Set the field `max_inactivity`.\nOptional. How long a user is allowed to take between actions before a new access token must be issued. Only set for Google Cloud apps."]
    pub fn set_max_inactivity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_inactivity = Some(v.into());
        self
    }
    #[doc = "Set the field `session_length`.\nOptional. The session length. Setting this field to zero is equal to disabling session. Also can set infinite session by flipping the enabled bit to false below. If useOidcMaxAge is true, for OIDC apps, the session length will be the minimum of this field and OIDC max_age param."]
    pub fn set_session_length(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_length = Some(v.into());
        self
    }
    #[doc = "Set the field `session_length_enabled`.\nOptional. This field enables or disables Google Cloud session length. When false, all fields set above will be disregarded and the session length is basically infinite."]
    pub fn set_session_length_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.session_length_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `session_reauth_method`.\nOptional. The session challenges proposed to users when the Google Cloud session length is up. Possible values: [\"LOGIN\", \"SECURITY_KEY\", \"PASSWORD\"]"]
    pub fn set_session_reauth_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_reauth_method = Some(v.into());
        self
    }
    #[doc = "Set the field `use_oidc_max_age`.\nOptional. Only useful for OIDC apps. When false, the OIDC max_age param, if passed in the authentication request will be ignored. When true, the re-auth period will be the minimum of the sessionLength field and the max_age OIDC param."]
    pub fn set_use_oidc_max_age(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_oidc_max_age = Some(v.into());
        self
    }
}
impl ToListMappable for AccessContextManagerGcpUserAccessBindingSessionSettingsEl {
    type O = BlockAssignable<AccessContextManagerGcpUserAccessBindingSessionSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerGcpUserAccessBindingSessionSettingsEl {}
impl BuildAccessContextManagerGcpUserAccessBindingSessionSettingsEl {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBindingSessionSettingsEl {
        AccessContextManagerGcpUserAccessBindingSessionSettingsEl {
            max_inactivity: core::default::Default::default(),
            session_length: core::default::Default::default(),
            session_length_enabled: core::default::Default::default(),
            session_reauth_method: core::default::Default::default(),
            use_oidc_max_age: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerGcpUserAccessBindingSessionSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerGcpUserAccessBindingSessionSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerGcpUserAccessBindingSessionSettingsElRef {
        AccessContextManagerGcpUserAccessBindingSessionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerGcpUserAccessBindingSessionSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_inactivity` after provisioning.\nOptional. How long a user is allowed to take between actions before a new access token must be issued. Only set for Google Cloud apps."]
    pub fn max_inactivity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_inactivity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_length` after provisioning.\nOptional. The session length. Setting this field to zero is equal to disabling session. Also can set infinite session by flipping the enabled bit to false below. If useOidcMaxAge is true, for OIDC apps, the session length will be the minimum of this field and OIDC max_age param."]
    pub fn session_length(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_length_enabled` after provisioning.\nOptional. This field enables or disables Google Cloud session length. When false, all fields set above will be disregarded and the session length is basically infinite."]
    pub fn session_length_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_length_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `session_reauth_method` after provisioning.\nOptional. The session challenges proposed to users when the Google Cloud session length is up. Possible values: [\"LOGIN\", \"SECURITY_KEY\", \"PASSWORD\"]"]
    pub fn session_reauth_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_reauth_method", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_oidc_max_age` after provisioning.\nOptional. Only useful for OIDC apps. When false, the OIDC max_age param, if passed in the authentication request will be ignored. When true, the re-auth period will be the minimum of the sessionLength field and the max_age OIDC param."]
    pub fn use_oidc_max_age(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_oidc_max_age", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBindingTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AccessContextManagerGcpUserAccessBindingTimeoutsEl {
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
impl ToListMappable for AccessContextManagerGcpUserAccessBindingTimeoutsEl {
    type O = BlockAssignable<AccessContextManagerGcpUserAccessBindingTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerGcpUserAccessBindingTimeoutsEl {}
impl BuildAccessContextManagerGcpUserAccessBindingTimeoutsEl {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBindingTimeoutsEl {
        AccessContextManagerGcpUserAccessBindingTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
        AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
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
struct AccessContextManagerGcpUserAccessBindingDynamic {
    scoped_access_settings:
        Option<DynamicBlock<AccessContextManagerGcpUserAccessBindingScopedAccessSettingsEl>>,
    session_settings:
        Option<DynamicBlock<AccessContextManagerGcpUserAccessBindingSessionSettingsEl>>,
}
