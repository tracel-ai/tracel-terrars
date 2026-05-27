use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApikeysKeyData {
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
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<Vec<ApikeysKeyRestrictionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApikeysKeyTimeoutsEl>,
    dynamic: ApikeysKeyDynamic,
}
struct ApikeysKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApikeysKeyData>,
}
#[derive(Clone)]
pub struct ApikeysKey(Rc<ApikeysKey_>);
impl ApikeysKey {
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
    #[doc = "Set the field `display_name`.\nHuman-readable display name of this API key. Modifiable by user."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\nThe email of the service account the key is bound to. If this field is specified, the key is a service account bound key and auth enabled. See [Documentation](https://cloud.google.com/docs/authentication/api-keys?#api-keys-bound-sa) for more details."]
    pub fn set_service_account_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `restrictions`.\n"]
    pub fn set_restrictions(self, v: impl Into<BlockAssignable<ApikeysKeyRestrictionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restrictions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restrictions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApikeysKeyTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of this API key. Modifiable by user."]
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
    #[doc = "Get a reference to the value of field `key_string` after provisioning.\nOutput only. An encrypted and signed value held by this key. This field can be accessed only through the `GetKeyString` method."]
    pub fn key_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_string", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the key. The name must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. In another word, the name must match the regular expression: `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nThe email of the service account the key is bound to. If this field is specified, the key is a service account bound key and auth enabled. See [Documentation](https://cloud.google.com/docs/authentication/api-keys?#api-keys-bound-sa) for more details."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique id in UUID4 format."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> ListRef<ApikeysKeyRestrictionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restrictions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApikeysKeyTimeoutsElRef {
        ApikeysKeyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApikeysKey {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApikeysKey {}
impl ToListMappable for ApikeysKey {
    type O = ListRef<ApikeysKeyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApikeysKey_ {
    fn extract_resource_type(&self) -> String {
        "google_apikeys_key".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApikeysKey {
    pub tf_id: String,
    #[doc = "The resource name of the key. The name must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. In another word, the name must match the regular expression: `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub name: PrimField<String>,
}
impl BuildApikeysKey {
    pub fn build(self, stack: &mut Stack) -> ApikeysKey {
        let out = ApikeysKey(Rc::new(ApikeysKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApikeysKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                service_account_email: core::default::Default::default(),
                restrictions: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApikeysKeyRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApikeysKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApikeysKeyRef {
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of this API key. Modifiable by user."]
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
    #[doc = "Get a reference to the value of field `key_string` after provisioning.\nOutput only. An encrypted and signed value held by this key. This field can be accessed only through the `GetKeyString` method."]
    pub fn key_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_string", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the key. The name must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. In another word, the name must match the regular expression: `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nThe email of the service account the key is bound to. If this field is specified, the key is a service account bound key and auth enabled. See [Documentation](https://cloud.google.com/docs/authentication/api-keys?#api-keys-bound-sa) for more details."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique id in UUID4 format."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> ListRef<ApikeysKeyRestrictionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restrictions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApikeysKeyTimeoutsElRef {
        ApikeysKeyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
    package_name: PrimField<String>,
    sha1_fingerprint: PrimField<String>,
}
impl ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {}
impl ToListMappable for ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
    type O = BlockAssignable<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
    #[doc = "The package name of the application."]
    pub package_name: PrimField<String>,
    #[doc = "The SHA1 fingerprint of the application. For example, both sha1 formats are acceptable : DA:39:A3:EE:5E:6B:4B:0D:32:55:BF:EF:95:60:18:90:AF:D8:07:09 or DA39A3EE5E6B4B0D3255BFEF95601890AFD80709. Output format is the latter."]
    pub sha1_fingerprint: PrimField<String>,
}
impl BuildApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
        ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
            package_name: self.package_name,
            sha1_fingerprint: self.sha1_fingerprint,
        }
    }
}
pub struct ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef {
        ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `package_name` after provisioning.\nThe package name of the application."]
    pub fn package_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.package_name", self.base))
    }
    #[doc = "Get a reference to the value of field `sha1_fingerprint` after provisioning.\nThe SHA1 fingerprint of the application. For example, both sha1 formats are acceptable : DA:39:A3:EE:5E:6B:4B:0D:32:55:BF:EF:95:60:18:90:AF:D8:07:09 or DA39A3EE5E6B4B0D3255BFEF95601890AFD80709. Output format is the latter."]
    pub fn sha1_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sha1_fingerprint", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApikeysKeyRestrictionsElAndroidKeyRestrictionsElDynamic {
    allowed_applications:
        Option<DynamicBlock<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl>>,
}
#[derive(Serialize)]
pub struct ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_applications:
        Option<Vec<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl>>,
    dynamic: ApikeysKeyRestrictionsElAndroidKeyRestrictionsElDynamic,
}
impl ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
    #[doc = "Set the field `allowed_applications`.\n"]
    pub fn set_allowed_applications(
        mut self,
        v: impl Into<
            BlockAssignable<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowed_applications = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_applications = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
    type O = BlockAssignable<ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {}
impl BuildApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
        ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
            allowed_applications: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef {
        ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_applications` after provisioning.\n"]
    pub fn allowed_applications(
        &self,
    ) -> ListRef<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_applications", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApikeysKeyRestrictionsElApiTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    methods: Option<ListField<PrimField<String>>>,
    service: PrimField<String>,
}
impl ApikeysKeyRestrictionsElApiTargetsEl {
    #[doc = "Set the field `methods`.\nOptional. List of one or more methods that can be called. If empty, all methods for the service are allowed. A wildcard (*) can be used as the last symbol. Valid examples: `google.cloud.translate.v2.TranslateService.GetSupportedLanguage` `TranslateText` `Get*` `translate.googleapis.com.Get*`"]
    pub fn set_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.methods = Some(v.into());
        self
    }
}
impl ToListMappable for ApikeysKeyRestrictionsElApiTargetsEl {
    type O = BlockAssignable<ApikeysKeyRestrictionsElApiTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApikeysKeyRestrictionsElApiTargetsEl {
    #[doc = "The service for this restriction. It should be the canonical service name, for example: `translate.googleapis.com`. You can use `gcloud services list` to get a list of services that are enabled in the project."]
    pub service: PrimField<String>,
}
impl BuildApikeysKeyRestrictionsElApiTargetsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElApiTargetsEl {
        ApikeysKeyRestrictionsElApiTargetsEl {
            methods: core::default::Default::default(),
            service: self.service,
        }
    }
}
pub struct ApikeysKeyRestrictionsElApiTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApikeysKeyRestrictionsElApiTargetsElRef {
    fn new(shared: StackShared, base: String) -> ApikeysKeyRestrictionsElApiTargetsElRef {
        ApikeysKeyRestrictionsElApiTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApikeysKeyRestrictionsElApiTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `methods` after provisioning.\nOptional. List of one or more methods that can be called. If empty, all methods for the service are allowed. A wildcard (*) can be used as the last symbol. Valid examples: `google.cloud.translate.v2.TranslateService.GetSupportedLanguage` `TranslateText` `Get*` `translate.googleapis.com.Get*`"]
    pub fn methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe service for this restriction. It should be the canonical service name, for example: `translate.googleapis.com`. You can use `gcloud services list` to get a list of services that are enabled in the project."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {
    allowed_referrers: ListField<PrimField<String>>,
}
impl ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {}
impl ToListMappable for ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {
    type O = BlockAssignable<ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {
    #[doc = "A list of regular expressions for the referrer URLs that are allowed to make API calls with this key."]
    pub allowed_referrers: ListField<PrimField<String>>,
}
impl BuildApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {
        ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {
            allowed_referrers: self.allowed_referrers,
        }
    }
}
pub struct ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef {
        ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_referrers` after provisioning.\nA list of regular expressions for the referrer URLs that are allowed to make API calls with this key."]
    pub fn allowed_referrers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_referrers", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApikeysKeyRestrictionsElIosKeyRestrictionsEl {
    allowed_bundle_ids: ListField<PrimField<String>>,
}
impl ApikeysKeyRestrictionsElIosKeyRestrictionsEl {}
impl ToListMappable for ApikeysKeyRestrictionsElIosKeyRestrictionsEl {
    type O = BlockAssignable<ApikeysKeyRestrictionsElIosKeyRestrictionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApikeysKeyRestrictionsElIosKeyRestrictionsEl {
    #[doc = "A list of bundle IDs that are allowed when making API calls with this key."]
    pub allowed_bundle_ids: ListField<PrimField<String>>,
}
impl BuildApikeysKeyRestrictionsElIosKeyRestrictionsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElIosKeyRestrictionsEl {
        ApikeysKeyRestrictionsElIosKeyRestrictionsEl {
            allowed_bundle_ids: self.allowed_bundle_ids,
        }
    }
}
pub struct ApikeysKeyRestrictionsElIosKeyRestrictionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApikeysKeyRestrictionsElIosKeyRestrictionsElRef {
    fn new(shared: StackShared, base: String) -> ApikeysKeyRestrictionsElIosKeyRestrictionsElRef {
        ApikeysKeyRestrictionsElIosKeyRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApikeysKeyRestrictionsElIosKeyRestrictionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_bundle_ids` after provisioning.\nA list of bundle IDs that are allowed when making API calls with this key."]
    pub fn allowed_bundle_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_bundle_ids", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApikeysKeyRestrictionsElServerKeyRestrictionsEl {
    allowed_ips: ListField<PrimField<String>>,
}
impl ApikeysKeyRestrictionsElServerKeyRestrictionsEl {}
impl ToListMappable for ApikeysKeyRestrictionsElServerKeyRestrictionsEl {
    type O = BlockAssignable<ApikeysKeyRestrictionsElServerKeyRestrictionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApikeysKeyRestrictionsElServerKeyRestrictionsEl {
    #[doc = "A list of the caller IP addresses that are allowed to make API calls with this key."]
    pub allowed_ips: ListField<PrimField<String>>,
}
impl BuildApikeysKeyRestrictionsElServerKeyRestrictionsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElServerKeyRestrictionsEl {
        ApikeysKeyRestrictionsElServerKeyRestrictionsEl {
            allowed_ips: self.allowed_ips,
        }
    }
}
pub struct ApikeysKeyRestrictionsElServerKeyRestrictionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApikeysKeyRestrictionsElServerKeyRestrictionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApikeysKeyRestrictionsElServerKeyRestrictionsElRef {
        ApikeysKeyRestrictionsElServerKeyRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApikeysKeyRestrictionsElServerKeyRestrictionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_ips` after provisioning.\nA list of the caller IP addresses that are allowed to make API calls with this key."]
    pub fn allowed_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_ips", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApikeysKeyRestrictionsElDynamic {
    android_key_restrictions:
        Option<DynamicBlock<ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl>>,
    api_targets: Option<DynamicBlock<ApikeysKeyRestrictionsElApiTargetsEl>>,
    browser_key_restrictions:
        Option<DynamicBlock<ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl>>,
    ios_key_restrictions: Option<DynamicBlock<ApikeysKeyRestrictionsElIosKeyRestrictionsEl>>,
    server_key_restrictions: Option<DynamicBlock<ApikeysKeyRestrictionsElServerKeyRestrictionsEl>>,
}
#[derive(Serialize)]
pub struct ApikeysKeyRestrictionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    android_key_restrictions: Option<Vec<ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_targets: Option<Vec<ApikeysKeyRestrictionsElApiTargetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_key_restrictions: Option<Vec<ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ios_key_restrictions: Option<Vec<ApikeysKeyRestrictionsElIosKeyRestrictionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_key_restrictions: Option<Vec<ApikeysKeyRestrictionsElServerKeyRestrictionsEl>>,
    dynamic: ApikeysKeyRestrictionsElDynamic,
}
impl ApikeysKeyRestrictionsEl {
    #[doc = "Set the field `android_key_restrictions`.\n"]
    pub fn set_android_key_restrictions(
        mut self,
        v: impl Into<BlockAssignable<ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.android_key_restrictions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.android_key_restrictions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `api_targets`.\n"]
    pub fn set_api_targets(
        mut self,
        v: impl Into<BlockAssignable<ApikeysKeyRestrictionsElApiTargetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_targets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `browser_key_restrictions`.\n"]
    pub fn set_browser_key_restrictions(
        mut self,
        v: impl Into<BlockAssignable<ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.browser_key_restrictions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.browser_key_restrictions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ios_key_restrictions`.\n"]
    pub fn set_ios_key_restrictions(
        mut self,
        v: impl Into<BlockAssignable<ApikeysKeyRestrictionsElIosKeyRestrictionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ios_key_restrictions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ios_key_restrictions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `server_key_restrictions`.\n"]
    pub fn set_server_key_restrictions(
        mut self,
        v: impl Into<BlockAssignable<ApikeysKeyRestrictionsElServerKeyRestrictionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.server_key_restrictions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.server_key_restrictions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApikeysKeyRestrictionsEl {
    type O = BlockAssignable<ApikeysKeyRestrictionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApikeysKeyRestrictionsEl {}
impl BuildApikeysKeyRestrictionsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsEl {
        ApikeysKeyRestrictionsEl {
            android_key_restrictions: core::default::Default::default(),
            api_targets: core::default::Default::default(),
            browser_key_restrictions: core::default::Default::default(),
            ios_key_restrictions: core::default::Default::default(),
            server_key_restrictions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApikeysKeyRestrictionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApikeysKeyRestrictionsElRef {
    fn new(shared: StackShared, base: String) -> ApikeysKeyRestrictionsElRef {
        ApikeysKeyRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApikeysKeyRestrictionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `android_key_restrictions` after provisioning.\n"]
    pub fn android_key_restrictions(
        &self,
    ) -> ListRef<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.android_key_restrictions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `api_targets` after provisioning.\n"]
    pub fn api_targets(&self) -> ListRef<ApikeysKeyRestrictionsElApiTargetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.api_targets", self.base))
    }
    #[doc = "Get a reference to the value of field `browser_key_restrictions` after provisioning.\n"]
    pub fn browser_key_restrictions(
        &self,
    ) -> ListRef<ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.browser_key_restrictions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ios_key_restrictions` after provisioning.\n"]
    pub fn ios_key_restrictions(&self) -> ListRef<ApikeysKeyRestrictionsElIosKeyRestrictionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ios_key_restrictions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `server_key_restrictions` after provisioning.\n"]
    pub fn server_key_restrictions(
        &self,
    ) -> ListRef<ApikeysKeyRestrictionsElServerKeyRestrictionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_key_restrictions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApikeysKeyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApikeysKeyTimeoutsEl {
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
impl ToListMappable for ApikeysKeyTimeoutsEl {
    type O = BlockAssignable<ApikeysKeyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApikeysKeyTimeoutsEl {}
impl BuildApikeysKeyTimeoutsEl {
    pub fn build(self) -> ApikeysKeyTimeoutsEl {
        ApikeysKeyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApikeysKeyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApikeysKeyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApikeysKeyTimeoutsElRef {
        ApikeysKeyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApikeysKeyTimeoutsElRef {
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
struct ApikeysKeyDynamic {
    restrictions: Option<DynamicBlock<ApikeysKeyRestrictionsEl>>,
}
