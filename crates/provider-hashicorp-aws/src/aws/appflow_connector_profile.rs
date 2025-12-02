use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppflowConnectorProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    connection_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connector_label: Option<PrimField<String>>,
    connector_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_arn: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connector_profile_config: Option<Vec<AppflowConnectorProfileConnectorProfileConfigEl>>,
    dynamic: AppflowConnectorProfileDynamic,
}
struct AppflowConnectorProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppflowConnectorProfileData>,
}
#[derive(Clone)]
pub struct AppflowConnectorProfile(Rc<AppflowConnectorProfile_>);
impl AppflowConnectorProfile {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `connector_label`.\n"]
    pub fn set_connector_label(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().connector_label = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_arn`.\n"]
    pub fn set_kms_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `connector_profile_config`.\n"]
    pub fn set_connector_profile_config(
        self,
        v: impl Into<BlockAssignable<AppflowConnectorProfileConnectorProfileConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connector_profile_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connector_profile_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `connection_mode` after provisioning.\n"]
    pub fn connection_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_label` after provisioning.\n"]
    pub fn connector_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_label", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credentials_arn` after provisioning.\n"]
    pub fn credentials_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_arn` after provisioning.\n"]
    pub fn kms_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_profile_config` after provisioning.\n"]
    pub fn connector_profile_config(
        &self,
    ) -> ListRef<AppflowConnectorProfileConnectorProfileConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connector_profile_config", self.extract_ref()),
        )
    }
}
impl Referable for AppflowConnectorProfile {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppflowConnectorProfile {}
impl ToListMappable for AppflowConnectorProfile {
    type O = ListRef<AppflowConnectorProfileRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppflowConnectorProfile_ {
    fn extract_resource_type(&self) -> String {
        "aws_appflow_connector_profile".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppflowConnectorProfile {
    pub tf_id: String,
    #[doc = ""]
    pub connection_mode: PrimField<String>,
    #[doc = ""]
    pub connector_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildAppflowConnectorProfile {
    pub fn build(self, stack: &mut Stack) -> AppflowConnectorProfile {
        let out = AppflowConnectorProfile(Rc::new(AppflowConnectorProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppflowConnectorProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                connection_mode: self.connection_mode,
                connector_label: core::default::Default::default(),
                connector_type: self.connector_type,
                id: core::default::Default::default(),
                kms_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                connector_profile_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppflowConnectorProfileRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppflowConnectorProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `connection_mode` after provisioning.\n"]
    pub fn connection_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_label` after provisioning.\n"]
    pub fn connector_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_label", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credentials_arn` after provisioning.\n"]
    pub fn credentials_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_arn` after provisioning.\n"]
    pub fn kms_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connector_profile_config` after provisioning.\n"]
    pub fn connector_profile_config(
        &self,
    ) -> ListRef<AppflowConnectorProfileConnectorProfileConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connector_profile_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl {
    api_key: PrimField<String>,
    secret_key: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl
{
    #[doc = ""]
    pub api_key: PrimField<String>,
    #[doc = ""]
    pub secret_key: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl {
            api_key: self.api_key,
            secret_key: self.secret_key,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_key` after provisioning.\n"]
    pub fn secret_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_key", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl
{
    api_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_secret_key: Option<PrimField<String>>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl { # [doc = "Set the field `api_secret_key`.\n"] pub fn set_api_secret_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . api_secret_key = Some (v . into ()) ; self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl
{
    #[doc = ""]
    pub api_key: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl { api_key : self . api_key , api_secret_key : core :: default :: Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `api_key` after provisioning.\n"] pub fn api_key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.api_key" , self . base)) } # [doc = "Get a reference to the value of field `api_secret_key` after provisioning.\n"] pub fn api_secret_key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.api_secret_key" , self . base)) } }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl
{
    password: PrimField<String>,
    username: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl { }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl
{
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl { password : self . password , username : self . username , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `password` after provisioning.\n"] pub fn password (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.password" , self . base)) } # [doc = "Get a reference to the value of field `username` after provisioning.\n"] pub fn username (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.username" , self . base)) } }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials_map: Option<RecField<PrimField<String>>>,
    custom_authentication_type: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl { # [doc = "Set the field `credentials_map`.\n"] pub fn set_credentials_map (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . credentials_map = Some (v . into ()) ; self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl
{
    #[doc = ""]
    pub custom_authentication_type: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl { credentials_map : core :: default :: Default :: default () , custom_authentication_type : self . custom_authentication_type , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `credentials_map` after provisioning.\n"] pub fn credentials_map (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.credentials_map" , self . base)) } # [doc = "Get a reference to the value of field `custom_authentication_type` after provisioning.\n"] pub fn custom_authentication_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.custom_authentication_type" , self . base)) } }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<PrimField<String>>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl { # [doc = "Set the field `auth_code`.\n"] pub fn set_auth_code (mut self , v : impl Into < PrimField < String > >) -> Self { self . auth_code = Some (v . into ()) ; self } # [doc = "Set the field `redirect_uri`.\n"] pub fn set_redirect_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . redirect_uri = Some (v . into ()) ; self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl { auth_code : core :: default :: Default :: default () , redirect_uri : core :: default :: Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auth_code` after provisioning.\n"] pub fn auth_code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auth_code" , self . base)) } # [doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"] pub fn redirect_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.redirect_uri" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElDynamic { oauth_request : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El { # [serde (skip_serializing_if = "Option::is_none")] access_token : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] client_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] client_secret : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] refresh_token : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oauth_request : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El { # [doc = "Set the field `access_token`.\n"] pub fn set_access_token (mut self , v : impl Into < PrimField < String > >) -> Self { self . access_token = Some (v . into ()) ; self } # [doc = "Set the field `client_id`.\n"] pub fn set_client_id (mut self , v : impl Into < PrimField < String > >) -> Self { self . client_id = Some (v . into ()) ; self } # [doc = "Set the field `client_secret`.\n"] pub fn set_client_secret (mut self , v : impl Into < PrimField < String > >) -> Self { self . client_secret = Some (v . into ()) ; self } # [doc = "Set the field `refresh_token`.\n"] pub fn set_refresh_token (mut self , v : impl Into < PrimField < String > >) -> Self { self . refresh_token = Some (v . into ()) ; self } # [doc = "Set the field `oauth_request`.\n"] pub fn set_oauth_request (mut self , v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . oauth_request = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . oauth_request = Some (d) ; } } self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El { access_token : core :: default :: Default :: default () , client_id : core :: default :: Default :: default () , client_secret : core :: default :: Default :: default () , refresh_token : core :: default :: Default :: default () , oauth_request : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `access_token` after provisioning.\n"] pub fn access_token (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.access_token" , self . base)) } # [doc = "Get a reference to the value of field `client_id` after provisioning.\n"] pub fn client_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.client_id" , self . base)) } # [doc = "Get a reference to the value of field `client_secret` after provisioning.\n"] pub fn client_secret (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.client_secret" , self . base)) } # [doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"] pub fn refresh_token (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.refresh_token" , self . base)) } # [doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"] pub fn oauth_request (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.oauth_request" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElDynamic { api_key : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl >> , basic : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl >> , custom : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl >> , oauth2 : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl { authentication_type : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] api_key : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl > > , # [serde (skip_serializing_if = "Option::is_none")] basic : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl > > , # [serde (skip_serializing_if = "Option::is_none")] custom : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl > > , # [serde (skip_serializing_if = "Option::is_none")] oauth2 : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `basic`.\n"]
    pub fn set_basic(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.basic = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.basic = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom`.\n"]
    pub fn set_custom(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth2`.\n"]
    pub fn set_oauth2(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth2 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl
{
    #[doc = ""]
    pub authentication_type: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl { authentication_type : self . authentication_type , api_key : core :: default :: Default :: default () , basic : core :: default :: Default :: default () , custom : core :: default :: Default :: default () , oauth2 : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef { shared : shared , base : base . to_string () , } } }
impl
    AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]    pub fn api_key (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef >{
        ListRef::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]    pub fn basic (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef >{
        ListRef::new(self.shared().clone(), format!("{}.basic", self.base))
    }
    #[doc = "Get a reference to the value of field `custom` after provisioning.\n"]    pub fn custom (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef >{
        ListRef::new(self.shared().clone(), format!("{}.custom", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth2` after provisioning.\n"]    pub fn oauth2 (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef >{
        ListRef::new(self.shared().clone(), format!("{}.oauth2", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl {
    api_key: PrimField<String>,
    application_key: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl
{
    #[doc = ""]
    pub api_key: PrimField<String>,
    #[doc = ""]
    pub application_key: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl {
            api_key: self.api_key,
            application_key: self.application_key,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
    #[doc = "Get a reference to the value of field `application_key` after provisioning.\n"]
    pub fn application_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl {
    api_token: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl
{
    #[doc = ""]
    pub api_token: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl {
            api_token: self.api_token,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_token` after provisioning.\n"]
    pub fn api_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_token", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<PrimField<String>>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl { # [doc = "Set the field `auth_code`.\n"] pub fn set_auth_code (mut self , v : impl Into < PrimField < String > >) -> Self { self . auth_code = Some (v . into ()) ; self } # [doc = "Set the field `redirect_uri`.\n"] pub fn set_redirect_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . redirect_uri = Some (v . into ()) ; self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl { auth_code : core :: default :: Default :: default () , redirect_uri : core :: default :: Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auth_code` after provisioning.\n"] pub fn auth_code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auth_code" , self . base)) } # [doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"] pub fn redirect_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.redirect_uri" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElDynamic { oauth_request : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl { # [serde (skip_serializing_if = "Option::is_none")] access_token : Option < PrimField < String > > , client_id : PrimField < String > , client_secret : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] refresh_token : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oauth_request : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `refresh_token`.\n"]
    pub fn set_refresh_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.refresh_token = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl
{
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub client_secret: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl { access_token : core :: default :: Default :: default () , client_id : self . client_id , client_secret : self . client_secret , refresh_token : core :: default :: Default :: default () , oauth_request : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef { shared : shared , base : base . to_string () , } } }
impl
    AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_token", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]    pub fn oauth_request (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_request", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<PrimField<String>>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl { # [doc = "Set the field `auth_code`.\n"] pub fn set_auth_code (mut self , v : impl Into < PrimField < String > >) -> Self { self . auth_code = Some (v . into ()) ; self } # [doc = "Set the field `redirect_uri`.\n"] pub fn set_redirect_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . redirect_uri = Some (v . into ()) ; self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl { auth_code : core :: default :: Default :: default () , redirect_uri : core :: default :: Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auth_code` after provisioning.\n"] pub fn auth_code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auth_code" , self . base)) } # [doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"] pub fn redirect_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.redirect_uri" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElDynamic { oauth_request : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl { # [serde (skip_serializing_if = "Option::is_none")] access_token : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] refresh_token : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oauth_request : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `refresh_token`.\n"]
    pub fn set_refresh_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.refresh_token = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl {
            access_token: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }
    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_token", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]    pub fn oauth_request (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_request", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl
{
    access_key_id: PrimField<String>,
    datakey: PrimField<String>,
    secret_access_key: PrimField<String>,
    user_id: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl
{
    #[doc = ""]
    pub access_key_id: PrimField<String>,
    #[doc = ""]
    pub datakey: PrimField<String>,
    #[doc = ""]
    pub secret_access_key: PrimField<String>,
    #[doc = ""]
    pub user_id: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl {
            access_key_id: self.access_key_id,
            datakey: self.datakey,
            secret_access_key: self.secret_access_key,
            user_id: self.user_id,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef { shared : shared , base : base . to_string () , }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\n"]
    pub fn access_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_key_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `datakey` after provisioning.\n"]
    pub fn datakey(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datakey", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_access_key` after provisioning.\n"]
    pub fn secret_access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_access_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<PrimField<String>>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl { # [doc = "Set the field `auth_code`.\n"] pub fn set_auth_code (mut self , v : impl Into < PrimField < String > >) -> Self { self . auth_code = Some (v . into ()) ; self } # [doc = "Set the field `redirect_uri`.\n"] pub fn set_redirect_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . redirect_uri = Some (v . into ()) ; self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl { auth_code : core :: default :: Default :: default () , redirect_uri : core :: default :: Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auth_code` after provisioning.\n"] pub fn auth_code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auth_code" , self . base)) } # [doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"] pub fn redirect_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.redirect_uri" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElDynamic { oauth_request : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl { # [serde (skip_serializing_if = "Option::is_none")] access_token : Option < PrimField < String > > , client_id : PrimField < String > , client_secret : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] oauth_request : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl
{
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub client_secret: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            oauth_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]    pub fn oauth_request (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_request", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl {
    password: PrimField<String>,
    username: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl
{
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl {
            password: self.password,
            username: self.username,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<PrimField<String>>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl { # [doc = "Set the field `auth_code`.\n"] pub fn set_auth_code (mut self , v : impl Into < PrimField < String > >) -> Self { self . auth_code = Some (v . into ()) ; self } # [doc = "Set the field `redirect_uri`.\n"] pub fn set_redirect_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . redirect_uri = Some (v . into ()) ; self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl { auth_code : core :: default :: Default :: default () , redirect_uri : core :: default :: Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auth_code` after provisioning.\n"] pub fn auth_code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auth_code" , self . base)) } # [doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"] pub fn redirect_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.redirect_uri" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElDynamic { oauth_request : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl { # [serde (skip_serializing_if = "Option::is_none")] access_token : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] client_credentials_arn : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] jwt_token : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oauth2_grant_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] refresh_token : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oauth_request : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `client_credentials_arn`.\n"]
    pub fn set_client_credentials_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_credentials_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `jwt_token`.\n"]
    pub fn set_jwt_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.jwt_token = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth2_grant_type`.\n"]
    pub fn set_oauth2_grant_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oauth2_grant_type = Some(v.into());
        self
    }
    #[doc = "Set the field `refresh_token`.\n"]
    pub fn set_refresh_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.refresh_token = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl {
            access_token: core::default::Default::default(),
            client_credentials_arn: core::default::Default::default(),
            jwt_token: core::default::Default::default(),
            oauth2_grant_type: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef { shared : shared , base : base . to_string () , }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }
    #[doc = "Get a reference to the value of field `client_credentials_arn` after provisioning.\n"]
    pub fn client_credentials_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_credentials_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `jwt_token` after provisioning.\n"]
    pub fn jwt_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwt_token", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth2_grant_type` after provisioning.\n"]
    pub fn oauth2_grant_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_grant_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_token", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]    pub fn oauth_request (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_request", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl
{
    password: PrimField<String>,
    username: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl { }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl
{
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl { password : self . password , username : self . username , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `password` after provisioning.\n"] pub fn password (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.password" , self . base)) } # [doc = "Get a reference to the value of field `username` after provisioning.\n"] pub fn username (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.username" , self . base)) } }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<PrimField<String>>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl { # [doc = "Set the field `auth_code`.\n"] pub fn set_auth_code (mut self , v : impl Into < PrimField < String > >) -> Self { self . auth_code = Some (v . into ()) ; self } # [doc = "Set the field `redirect_uri`.\n"] pub fn set_redirect_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . redirect_uri = Some (v . into ()) ; self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl { auth_code : core :: default :: Default :: default () , redirect_uri : core :: default :: Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auth_code` after provisioning.\n"] pub fn auth_code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auth_code" , self . base)) } # [doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"] pub fn redirect_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.redirect_uri" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElDynamic { oauth_request : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl { # [serde (skip_serializing_if = "Option::is_none")] access_token : Option < PrimField < String > > , client_id : PrimField < String > , client_secret : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] refresh_token : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oauth_request : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl { # [doc = "Set the field `access_token`.\n"] pub fn set_access_token (mut self , v : impl Into < PrimField < String > >) -> Self { self . access_token = Some (v . into ()) ; self } # [doc = "Set the field `refresh_token`.\n"] pub fn set_refresh_token (mut self , v : impl Into < PrimField < String > >) -> Self { self . refresh_token = Some (v . into ()) ; self } # [doc = "Set the field `oauth_request`.\n"] pub fn set_oauth_request (mut self , v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . oauth_request = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . oauth_request = Some (d) ; } } self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl
{
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub client_secret: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl { access_token : core :: default :: Default :: default () , client_id : self . client_id , client_secret : self . client_secret , refresh_token : core :: default :: Default :: default () , oauth_request : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `access_token` after provisioning.\n"] pub fn access_token (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.access_token" , self . base)) } # [doc = "Get a reference to the value of field `client_id` after provisioning.\n"] pub fn client_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.client_id" , self . base)) } # [doc = "Get a reference to the value of field `client_secret` after provisioning.\n"] pub fn client_secret (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.client_secret" , self . base)) } # [doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"] pub fn refresh_token (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.refresh_token" , self . base)) } # [doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"] pub fn oauth_request (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.oauth_request" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElDynamic { basic_auth_credentials : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl >> , oauth_credentials : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl { # [serde (skip_serializing_if = "Option::is_none")] basic_auth_credentials : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl > > , # [serde (skip_serializing_if = "Option::is_none")] oauth_credentials : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl {
    #[doc = "Set the field `basic_auth_credentials`.\n"]
    pub fn set_basic_auth_credentials(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.basic_auth_credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_auth_credentials = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth_credentials`.\n"]
    pub fn set_oauth_credentials(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_credentials = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl {
            basic_auth_credentials: core::default::Default::default(),
            oauth_credentials: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `basic_auth_credentials` after provisioning.\n"]    pub fn basic_auth_credentials (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.basic_auth_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_credentials` after provisioning.\n"]    pub fn oauth_credentials (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_credentials", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl
{
    password: PrimField<String>,
    username: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl
{
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl {
            password: self.password,
            username: self.username,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef { shared : shared , base : base . to_string () , }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl {
    api_key: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl
{
    #[doc = ""]
    pub api_key: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl {
            api_key: self.api_key,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<PrimField<String>>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl { # [doc = "Set the field `auth_code`.\n"] pub fn set_auth_code (mut self , v : impl Into < PrimField < String > >) -> Self { self . auth_code = Some (v . into ()) ; self } # [doc = "Set the field `redirect_uri`.\n"] pub fn set_redirect_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . redirect_uri = Some (v . into ()) ; self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl { auth_code : core :: default :: Default :: default () , redirect_uri : core :: default :: Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auth_code` after provisioning.\n"] pub fn auth_code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auth_code" , self . base)) } # [doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"] pub fn redirect_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.redirect_uri" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElDynamic { oauth_request : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl { # [serde (skip_serializing_if = "Option::is_none")] access_token : Option < PrimField < String > > , client_id : PrimField < String > , client_secret : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] oauth_request : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl
{
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub client_secret: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            oauth_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]    pub fn oauth_request (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_request", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl {
    password: PrimField<String>,
    username: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl
{
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl {
            password: self.password,
            username: self.username,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl
{
    api_secret_key: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl
{
    #[doc = ""]
    pub api_secret_key: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl {
            api_secret_key: self.api_secret_key,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef { shared : shared , base : base . to_string () , }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_secret_key` after provisioning.\n"]
    pub fn api_secret_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_secret_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl {
    password: PrimField<String>,
    username: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl
{
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl {
            password: self.password,
            username: self.username,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<PrimField<String>>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl { # [doc = "Set the field `auth_code`.\n"] pub fn set_auth_code (mut self , v : impl Into < PrimField < String > >) -> Self { self . auth_code = Some (v . into ()) ; self } # [doc = "Set the field `redirect_uri`.\n"] pub fn set_redirect_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . redirect_uri = Some (v . into ()) ; self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl { auth_code : core :: default :: Default :: default () , redirect_uri : core :: default :: Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auth_code` after provisioning.\n"] pub fn auth_code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auth_code" , self . base)) } # [doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"] pub fn redirect_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.redirect_uri" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElDynamic { oauth_request : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl { # [serde (skip_serializing_if = "Option::is_none")] access_token : Option < PrimField < String > > , client_id : PrimField < String > , client_secret : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] oauth_request : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl
{
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub client_secret: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            oauth_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]    pub fn oauth_request (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_request", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynamic { amplitude : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl >> , custom_connector : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl >> , datadog : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl >> , dynatrace : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl >> , google_analytics : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl >> , honeycode : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl >> , infor_nexus : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl >> , marketo : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl >> , redshift : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl >> , salesforce : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl >> , sapo_data : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl >> , service_now : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl >> , singular : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl >> , slack : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl >> , snowflake : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl >> , trendmicro : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl >> , veeva : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl >> , zendesk : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl { # [serde (skip_serializing_if = "Option::is_none")] amplitude : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl > > , # [serde (skip_serializing_if = "Option::is_none")] custom_connector : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl > > , # [serde (skip_serializing_if = "Option::is_none")] datadog : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl > > , # [serde (skip_serializing_if = "Option::is_none")] dynatrace : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl > > , # [serde (skip_serializing_if = "Option::is_none")] google_analytics : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl > > , # [serde (skip_serializing_if = "Option::is_none")] honeycode : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl > > , # [serde (skip_serializing_if = "Option::is_none")] infor_nexus : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl > > , # [serde (skip_serializing_if = "Option::is_none")] marketo : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl > > , # [serde (skip_serializing_if = "Option::is_none")] redshift : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl > > , # [serde (skip_serializing_if = "Option::is_none")] salesforce : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl > > , # [serde (skip_serializing_if = "Option::is_none")] sapo_data : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl > > , # [serde (skip_serializing_if = "Option::is_none")] service_now : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl > > , # [serde (skip_serializing_if = "Option::is_none")] singular : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl > > , # [serde (skip_serializing_if = "Option::is_none")] slack : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl > > , # [serde (skip_serializing_if = "Option::is_none")] snowflake : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl > > , # [serde (skip_serializing_if = "Option::is_none")] trendmicro : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl > > , # [serde (skip_serializing_if = "Option::is_none")] veeva : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl > > , # [serde (skip_serializing_if = "Option::is_none")] zendesk : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {
    #[doc = "Set the field `amplitude`.\n"]
    pub fn set_amplitude(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amplitude = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amplitude = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_connector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_connector = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.datadog = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dynatrace`.\n"]
    pub fn set_dynatrace(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dynatrace = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dynatrace = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `google_analytics`.\n"]
    pub fn set_google_analytics(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_analytics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_analytics = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `honeycode`.\n"]
    pub fn set_honeycode(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.honeycode = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.honeycode = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `infor_nexus`.\n"]
    pub fn set_infor_nexus(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.infor_nexus = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.infor_nexus = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.marketo = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.marketo = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redshift`.\n"]
    pub fn set_redshift(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redshift = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.salesforce = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sapo_data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sapo_data = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_now = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_now = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `singular`.\n"]
    pub fn set_singular(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.singular = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.singular = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.slack = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.slack = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `snowflake`.\n"]
    pub fn set_snowflake(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.snowflake = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `trendmicro`.\n"]
    pub fn set_trendmicro(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trendmicro = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trendmicro = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `veeva`.\n"]
    pub fn set_veeva(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.veeva = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.veeva = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.zendesk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.zendesk = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {
            amplitude: core::default::Default::default(),
            custom_connector: core::default::Default::default(),
            datadog: core::default::Default::default(),
            dynatrace: core::default::Default::default(),
            google_analytics: core::default::Default::default(),
            honeycode: core::default::Default::default(),
            infor_nexus: core::default::Default::default(),
            marketo: core::default::Default::default(),
            redshift: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            service_now: core::default::Default::default(),
            singular: core::default::Default::default(),
            slack: core::default::Default::default(),
            snowflake: core::default::Default::default(),
            trendmicro: core::default::Default::default(),
            veeva: core::default::Default::default(),
            zendesk: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]    pub fn custom_connector (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_connector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }
    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }
    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]    pub fn google_analytics (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_analytics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `honeycode` after provisioning.\n"]
    pub fn honeycode(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.honeycode", self.base))
    }
    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }
    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }
    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }
    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }
    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }
    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }
    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.singular", self.base))
    }
    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }
    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }
    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }
    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.veeva", self.base))
    }
    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl {
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl {}
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl
{
    oauth2_grant_type: PrimField<String>,
    token_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_url_custom_properties: Option<RecField<PrimField<String>>>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl { # [doc = "Set the field `token_url_custom_properties`.\n"] pub fn set_token_url_custom_properties (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . token_url_custom_properties = Some (v . into ()) ; self } }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl
{
    #[doc = ""]
    pub oauth2_grant_type: PrimField<String>,
    #[doc = ""]
    pub token_url: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl { oauth2_grant_type : self . oauth2_grant_type , token_url : self . token_url , token_url_custom_properties : core :: default :: Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `oauth2_grant_type` after provisioning.\n"] pub fn oauth2_grant_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.oauth2_grant_type" , self . base)) } # [doc = "Get a reference to the value of field `token_url` after provisioning.\n"] pub fn token_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.token_url" , self . base)) } # [doc = "Get a reference to the value of field `token_url_custom_properties` after provisioning.\n"] pub fn token_url_custom_properties (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.token_url_custom_properties" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElDynamic { oauth2_properties : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl { # [serde (skip_serializing_if = "Option::is_none")] profile_properties : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] oauth2_properties : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl {
    #[doc = "Set the field `profile_properties`.\n"]
    pub fn set_profile_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.profile_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth2_properties`.\n"]
    pub fn set_oauth2_properties(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth2_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_properties = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl
{
    type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl { profile_properties : core :: default :: Default :: default () , oauth2_properties : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef { shared : shared , base : base . to_string () , } } }
impl
    AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `profile_properties` after provisioning.\n"]
    pub fn profile_properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.profile_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2_properties` after provisioning.\n"]    pub fn oauth2_properties (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth2_properties", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl {
    instance_url: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl
{
    #[doc = ""]
    pub instance_url: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl {
            instance_url: self.instance_url,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl {
    instance_url: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl
{
    #[doc = ""]
    pub instance_url: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl {
            instance_url: self.instance_url,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl
{}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl
{
    type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl { } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef { shared : shared , base : base . to_string () , } } }
impl
    AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl {
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl {}
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl {
    instance_url: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl
{
    #[doc = ""]
    pub instance_url: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl {
            instance_url: self.instance_url,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl {
    instance_url: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl
{
    #[doc = ""]
    pub instance_url: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl {
            instance_url: self.instance_url,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_api_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_url: Option<PrimField<String>>,
    role_arn: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_identifier`.\n"]
    pub fn set_cluster_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `data_api_role_arn`.\n"]
    pub fn set_data_api_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_api_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_name = Some(v.into());
        self
    }
    #[doc = "Set the field `database_url`.\n"]
    pub fn set_database_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_url = Some(v.into());
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl
{
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            cluster_identifier: core::default::Default::default(),
            data_api_role_arn: core::default::Default::default(),
            database_name: core::default::Default::default(),
            database_url: core::default::Default::default(),
            role_arn: self.role_arn,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_api_role_arn` after provisioning.\n"]
    pub fn data_api_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_api_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database_url` after provisioning.\n"]
    pub fn database_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_url", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_sandbox_environment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_privatelink_for_metadata_and_authorization: Option<PrimField<bool>>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl {
    #[doc = "Set the field `instance_url`.\n"]
    pub fn set_instance_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_url = Some(v.into());
        self
    }
    #[doc = "Set the field `is_sandbox_environment`.\n"]
    pub fn set_is_sandbox_environment(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_sandbox_environment = Some(v.into());
        self
    }
    #[doc = "Set the field `use_privatelink_for_metadata_and_authorization`.\n"]
    pub fn set_use_privatelink_for_metadata_and_authorization(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.use_privatelink_for_metadata_and_authorization = Some(v.into());
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl {
            instance_url: core::default::Default::default(),
            is_sandbox_environment: core::default::Default::default(),
            use_privatelink_for_metadata_and_authorization: core::default::Default::default(),
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
    #[doc = "Get a reference to the value of field `is_sandbox_environment` after provisioning.\n"]
    pub fn is_sandbox_environment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_sandbox_environment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_privatelink_for_metadata_and_authorization` after provisioning.\n"]
    pub fn use_privatelink_for_metadata_and_authorization(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.use_privatelink_for_metadata_and_authorization",
                self.base
            ),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl
{
    auth_code_url: PrimField<String>,
    oauth_scopes: ListField<PrimField<String>>,
    token_url: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl { }
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl { type O = BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl
{
    #[doc = ""]
    pub auth_code_url: PrimField<String>,
    #[doc = ""]
    pub oauth_scopes: ListField<PrimField<String>>,
    #[doc = ""]
    pub token_url: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl { pub fn build (self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl { AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl { auth_code_url : self . auth_code_url , oauth_scopes : self . oauth_scopes , token_url : self . token_url , } } }
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef { fn new (shared : StackShared , base : String) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef { AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef { shared : shared , base : base . to_string () , } } }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auth_code_url` after provisioning.\n"] pub fn auth_code_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auth_code_url" , self . base)) } # [doc = "Get a reference to the value of field `oauth_scopes` after provisioning.\n"] pub fn oauth_scopes (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.oauth_scopes" , self . base)) } # [doc = "Get a reference to the value of field `token_url` after provisioning.\n"] pub fn token_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.token_url" , self . base)) } }
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElDynamic { oauth_properties : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl { application_host_url : PrimField < String > , application_service_path : PrimField < String > , client_number : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] logon_language : Option < PrimField < String > > , port_number : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] private_link_service_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oauth_properties : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl {
    #[doc = "Set the field `logon_language`.\n"]
    pub fn set_logon_language(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logon_language = Some(v.into());
        self
    }
    #[doc = "Set the field `private_link_service_name`.\n"]
    pub fn set_private_link_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_link_service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_properties`.\n"]
    pub fn set_oauth_properties(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_properties = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl
{
    #[doc = ""]
    pub application_host_url: PrimField<String>,
    #[doc = ""]
    pub application_service_path: PrimField<String>,
    #[doc = ""]
    pub client_number: PrimField<String>,
    #[doc = ""]
    pub port_number: PrimField<f64>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl {
            application_host_url: self.application_host_url,
            application_service_path: self.application_service_path,
            client_number: self.client_number,
            logon_language: core::default::Default::default(),
            port_number: self.port_number,
            private_link_service_name: core::default::Default::default(),
            oauth_properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_host_url` after provisioning.\n"]
    pub fn application_host_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_host_url", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `application_service_path` after provisioning.\n"]
    pub fn application_service_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_service_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_number` after provisioning.\n"]
    pub fn client_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logon_language` after provisioning.\n"]
    pub fn logon_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logon_language", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port_number` after provisioning.\n"]
    pub fn port_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_number", self.base))
    }
    #[doc = "Get a reference to the value of field `private_link_service_name` after provisioning.\n"]
    pub fn private_link_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_link_service_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_properties` after provisioning.\n"]    pub fn oauth_properties (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_properties", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl {
    instance_url: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl
{
    #[doc = ""]
    pub instance_url: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl {
            instance_url: self.instance_url,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl {
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl {}
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {
    instance_url: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {
    #[doc = ""]
    pub instance_url: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {
            instance_url: self.instance_url,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_name: Option<PrimField<String>>,
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_link_service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    stage: PrimField<String>,
    warehouse: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl {
    #[doc = "Set the field `account_name`.\n"]
    pub fn set_account_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_name = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `private_link_service_name`.\n"]
    pub fn set_private_link_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_link_service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl
{
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub stage: PrimField<String>,
    #[doc = ""]
    pub warehouse: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl {
            account_name: core::default::Default::default(),
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            private_link_service_name: core::default::Default::default(),
            region: core::default::Default::default(),
            stage: self.stage,
            warehouse: self.warehouse,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_name` after provisioning.\n"]
    pub fn account_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_link_service_name` after provisioning.\n"]
    pub fn private_link_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_link_service_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }
    #[doc = "Get a reference to the value of field `warehouse` after provisioning.\n"]
    pub fn warehouse(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.warehouse", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl
{}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl
{}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl {}
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {
    instance_url: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {
    #[doc = ""]
    pub instance_url: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {
            instance_url: self.instance_url,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl {
    instance_url: PrimField<String>,
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl {}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl
{
    #[doc = ""]
    pub instance_url: PrimField<String>,
}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl {
            instance_url: self.instance_url,
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef
    {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynamic { amplitude : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl >> , custom_connector : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl >> , datadog : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl >> , dynatrace : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl >> , google_analytics : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl >> , honeycode : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl >> , infor_nexus : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl >> , marketo : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl >> , redshift : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl >> , salesforce : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl >> , sapo_data : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl >> , service_now : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl >> , singular : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl >> , slack : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl >> , snowflake : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl >> , trendmicro : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl >> , veeva : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl >> , zendesk : Option < DynamicBlock < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl >> , }
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl { # [serde (skip_serializing_if = "Option::is_none")] amplitude : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl > > , # [serde (skip_serializing_if = "Option::is_none")] custom_connector : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl > > , # [serde (skip_serializing_if = "Option::is_none")] datadog : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl > > , # [serde (skip_serializing_if = "Option::is_none")] dynatrace : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl > > , # [serde (skip_serializing_if = "Option::is_none")] google_analytics : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl > > , # [serde (skip_serializing_if = "Option::is_none")] honeycode : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl > > , # [serde (skip_serializing_if = "Option::is_none")] infor_nexus : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl > > , # [serde (skip_serializing_if = "Option::is_none")] marketo : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl > > , # [serde (skip_serializing_if = "Option::is_none")] redshift : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl > > , # [serde (skip_serializing_if = "Option::is_none")] salesforce : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl > > , # [serde (skip_serializing_if = "Option::is_none")] sapo_data : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl > > , # [serde (skip_serializing_if = "Option::is_none")] service_now : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl > > , # [serde (skip_serializing_if = "Option::is_none")] singular : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl > > , # [serde (skip_serializing_if = "Option::is_none")] slack : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl > > , # [serde (skip_serializing_if = "Option::is_none")] snowflake : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl > > , # [serde (skip_serializing_if = "Option::is_none")] trendmicro : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl > > , # [serde (skip_serializing_if = "Option::is_none")] veeva : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl > > , # [serde (skip_serializing_if = "Option::is_none")] zendesk : Option < Vec < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl > > , dynamic : AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynamic , }
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {
    #[doc = "Set the field `amplitude`.\n"]
    pub fn set_amplitude(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amplitude = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amplitude = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_connector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_connector = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.datadog = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dynatrace`.\n"]
    pub fn set_dynatrace(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dynatrace = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dynatrace = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `google_analytics`.\n"]
    pub fn set_google_analytics(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_analytics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_analytics = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `honeycode`.\n"]
    pub fn set_honeycode(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.honeycode = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.honeycode = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `infor_nexus`.\n"]
    pub fn set_infor_nexus(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.infor_nexus = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.infor_nexus = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.marketo = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.marketo = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redshift`.\n"]
    pub fn set_redshift(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redshift = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.salesforce = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sapo_data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sapo_data = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_now = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_now = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `singular`.\n"]
    pub fn set_singular(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.singular = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.singular = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.slack = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.slack = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `snowflake`.\n"]
    pub fn set_snowflake(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.snowflake = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `trendmicro`.\n"]
    pub fn set_trendmicro(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trendmicro = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trendmicro = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `veeva`.\n"]
    pub fn set_veeva(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.veeva = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.veeva = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(
        mut self,
        v : impl Into < BlockAssignable < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.zendesk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.zendesk = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl
{
    type O = BlockAssignable<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {}
impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {
            amplitude: core::default::Default::default(),
            custom_connector: core::default::Default::default(),
            datadog: core::default::Default::default(),
            dynatrace: core::default::Default::default(),
            google_analytics: core::default::Default::default(),
            honeycode: core::default::Default::default(),
            infor_nexus: core::default::Default::default(),
            marketo: core::default::Default::default(),
            redshift: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            service_now: core::default::Default::default(),
            singular: core::default::Default::default(),
            slack: core::default::Default::default(),
            snowflake: core::default::Default::default(),
            trendmicro: core::default::Default::default(),
            veeva: core::default::Default::default(),
            zendesk: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]    pub fn custom_connector (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_connector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }
    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }
    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]    pub fn google_analytics (& self) -> ListRef < AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_analytics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `honeycode` after provisioning.\n"]
    pub fn honeycode(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.honeycode", self.base))
    }
    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }
    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }
    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }
    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }
    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }
    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }
    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.singular", self.base))
    }
    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }
    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }
    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }
    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.veeva", self.base))
    }
    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(
        &self,
    ) -> ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElDynamic {
    connector_profile_credentials: Option<
        DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl>,
    >,
    connector_profile_properties: Option<
        DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl>,
    >,
}
#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connector_profile_credentials:
        Option<Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connector_profile_properties:
        Option<Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl>>,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElDynamic,
}
impl AppflowConnectorProfileConnectorProfileConfigEl {
    #[doc = "Set the field `connector_profile_credentials`.\n"]
    pub fn set_connector_profile_credentials(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connector_profile_credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connector_profile_credentials = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `connector_profile_properties`.\n"]
    pub fn set_connector_profile_properties(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connector_profile_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connector_profile_properties = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppflowConnectorProfileConnectorProfileConfigEl {
    type O = BlockAssignable<AppflowConnectorProfileConnectorProfileConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppflowConnectorProfileConnectorProfileConfigEl {}
impl BuildAppflowConnectorProfileConnectorProfileConfigEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigEl {
        AppflowConnectorProfileConnectorProfileConfigEl {
            connector_profile_credentials: core::default::Default::default(),
            connector_profile_properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppflowConnectorProfileConnectorProfileConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppflowConnectorProfileConnectorProfileConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElRef {
        AppflowConnectorProfileConnectorProfileConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppflowConnectorProfileConnectorProfileConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connector_profile_credentials` after provisioning.\n"]
    pub fn connector_profile_credentials(
        &self,
    ) -> ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connector_profile_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connector_profile_properties` after provisioning.\n"]
    pub fn connector_profile_properties(
        &self,
    ) -> ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connector_profile_properties", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AppflowConnectorProfileDynamic {
    connector_profile_config: Option<DynamicBlock<AppflowConnectorProfileConnectorProfileConfigEl>>,
}
