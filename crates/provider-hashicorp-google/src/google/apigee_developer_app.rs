use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeDeveloperAppData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_products: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_family: Option<PrimField<String>>,
    callback_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    developer_email: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_expires_in: Option<PrimField<String>>,
    name: PrimField<String>,
    org_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<ApigeeDeveloperAppAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeDeveloperAppTimeoutsEl>,
    dynamic: ApigeeDeveloperAppDynamic,
}
struct ApigeeDeveloperApp_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeDeveloperAppData>,
}
#[derive(Clone)]
pub struct ApigeeDeveloperApp(Rc<ApigeeDeveloperApp_>);
impl ApigeeDeveloperApp {
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
    #[doc = "Set the field `api_products`.\nList of API products associated with the developer app."]
    pub fn set_api_products(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().api_products = Some(v.into());
        self
    }
    #[doc = "Set the field `app_family`.\nDeveloper app family."]
    pub fn set_app_family(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().app_family = Some(v.into());
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
    #[doc = "Set the field `key_expires_in`.\nExpiration time, in milliseconds, for the consumer key that is generated\nfor the developer app. If not set or left to the default value of -1,\nthe API key never expires. The expiration time can't be updated after it is set."]
    pub fn set_key_expires_in(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_expires_in = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\nScopes to apply to the developer app.\nThe specified scopes must already exist for the API product that\nyou associate with the developer app."]
    pub fn set_scopes(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\nStatus of the credential. Valid values include approved or revoked."]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        self,
        v: impl Into<BlockAssignable<ApigeeDeveloperAppAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attributes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeDeveloperAppTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `api_products` after provisioning.\nList of API products associated with the developer app."]
    pub fn api_products(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.api_products", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_family` after provisioning.\nDeveloper app family."]
    pub fn app_family(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_family", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nID of the developer app. This ID is not user specified but is\nautomatically generated on app creation. appId is a UUID."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `callback_url` after provisioning.\nCallback URL used by OAuth 2.0 authorization servers to communicate\nauthorization codes back to developer apps."]
    pub fn callback_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.callback_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nTime at which the developer was created in milliseconds since epoch."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credentials` after provisioning.\nOutput only. Set of credentials for the developer app consisting of\nthe consumer key/secret pairs associated with the API products."]
    pub fn credentials(&self) -> ListRef<ApigeeDeveloperAppCredentialsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.credentials", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `developer_email` after provisioning.\nEmail address of the developer.\nThis value is used to uniquely identify the developer in Apigee hybrid.\nNote that the email address has to be in lowercase only."]
    pub fn developer_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.developer_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `developer_id` after provisioning.\nID of the developer."]
    pub fn developer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.developer_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_expires_in` after provisioning.\nExpiration time, in milliseconds, for the consumer key that is generated\nfor the developer app. If not set or left to the default value of -1,\nthe API key never expires. The expiration time can't be updated after it is set."]
    pub fn key_expires_in(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_expires_in", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_at` after provisioning.\nTime at which the developer was last modified in milliseconds since epoch."]
    pub fn last_modified_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the developer app."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nScopes to apply to the developer app.\nThe specified scopes must already exist for the API product that\nyou associate with the developer app."]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.scopes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the credential. Valid values include approved or revoked."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<ApigeeDeveloperAppAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeDeveloperAppTimeoutsElRef {
        ApigeeDeveloperAppTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeDeveloperApp {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeDeveloperApp {}
impl ToListMappable for ApigeeDeveloperApp {
    type O = ListRef<ApigeeDeveloperAppRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeDeveloperApp_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_developer_app".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeDeveloperApp {
    pub tf_id: String,
    #[doc = "Callback URL used by OAuth 2.0 authorization servers to communicate\nauthorization codes back to developer apps."]
    pub callback_url: PrimField<String>,
    #[doc = "Email address of the developer.\nThis value is used to uniquely identify the developer in Apigee hybrid.\nNote that the email address has to be in lowercase only."]
    pub developer_email: PrimField<String>,
    #[doc = "Name of the developer app."]
    pub name: PrimField<String>,
    #[doc = "The Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub org_id: PrimField<String>,
}
impl BuildApigeeDeveloperApp {
    pub fn build(self, stack: &mut Stack) -> ApigeeDeveloperApp {
        let out = ApigeeDeveloperApp(Rc::new(ApigeeDeveloperApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeDeveloperAppData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_products: core::default::Default::default(),
                app_family: core::default::Default::default(),
                callback_url: self.callback_url,
                deletion_policy: core::default::Default::default(),
                developer_email: self.developer_email,
                id: core::default::Default::default(),
                key_expires_in: core::default::Default::default(),
                name: self.name,
                org_id: self.org_id,
                scopes: core::default::Default::default(),
                status: core::default::Default::default(),
                attributes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeDeveloperAppRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeDeveloperAppRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeDeveloperAppRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_products` after provisioning.\nList of API products associated with the developer app."]
    pub fn api_products(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.api_products", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_family` after provisioning.\nDeveloper app family."]
    pub fn app_family(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_family", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nID of the developer app. This ID is not user specified but is\nautomatically generated on app creation. appId is a UUID."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `callback_url` after provisioning.\nCallback URL used by OAuth 2.0 authorization servers to communicate\nauthorization codes back to developer apps."]
    pub fn callback_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.callback_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nTime at which the developer was created in milliseconds since epoch."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credentials` after provisioning.\nOutput only. Set of credentials for the developer app consisting of\nthe consumer key/secret pairs associated with the API products."]
    pub fn credentials(&self) -> ListRef<ApigeeDeveloperAppCredentialsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.credentials", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `developer_email` after provisioning.\nEmail address of the developer.\nThis value is used to uniquely identify the developer in Apigee hybrid.\nNote that the email address has to be in lowercase only."]
    pub fn developer_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.developer_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `developer_id` after provisioning.\nID of the developer."]
    pub fn developer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.developer_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_expires_in` after provisioning.\nExpiration time, in milliseconds, for the consumer key that is generated\nfor the developer app. If not set or left to the default value of -1,\nthe API key never expires. The expiration time can't be updated after it is set."]
    pub fn key_expires_in(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_expires_in", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_at` after provisioning.\nTime at which the developer was last modified in milliseconds since epoch."]
    pub fn last_modified_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the developer app."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nScopes to apply to the developer app.\nThe specified scopes must already exist for the API product that\nyou associate with the developer app."]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.scopes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the credential. Valid values include approved or revoked."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<ApigeeDeveloperAppAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeDeveloperAppTimeoutsElRef {
        ApigeeDeveloperAppTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeDeveloperAppCredentialsElApiProductsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    apiproduct: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl ApigeeDeveloperAppCredentialsElApiProductsEl {
    #[doc = "Set the field `apiproduct`.\n"]
    pub fn set_apiproduct(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.apiproduct = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeDeveloperAppCredentialsElApiProductsEl {
    type O = BlockAssignable<ApigeeDeveloperAppCredentialsElApiProductsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeDeveloperAppCredentialsElApiProductsEl {}
impl BuildApigeeDeveloperAppCredentialsElApiProductsEl {
    pub fn build(self) -> ApigeeDeveloperAppCredentialsElApiProductsEl {
        ApigeeDeveloperAppCredentialsElApiProductsEl {
            apiproduct: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct ApigeeDeveloperAppCredentialsElApiProductsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeDeveloperAppCredentialsElApiProductsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeDeveloperAppCredentialsElApiProductsElRef {
        ApigeeDeveloperAppCredentialsElApiProductsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeDeveloperAppCredentialsElApiProductsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `apiproduct` after provisioning.\n"]
    pub fn apiproduct(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.apiproduct", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeDeveloperAppCredentialsElAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ApigeeDeveloperAppCredentialsElAttributesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeDeveloperAppCredentialsElAttributesEl {
    type O = BlockAssignable<ApigeeDeveloperAppCredentialsElAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeDeveloperAppCredentialsElAttributesEl {}
impl BuildApigeeDeveloperAppCredentialsElAttributesEl {
    pub fn build(self) -> ApigeeDeveloperAppCredentialsElAttributesEl {
        ApigeeDeveloperAppCredentialsElAttributesEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ApigeeDeveloperAppCredentialsElAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeDeveloperAppCredentialsElAttributesElRef {
    fn new(shared: StackShared, base: String) -> ApigeeDeveloperAppCredentialsElAttributesElRef {
        ApigeeDeveloperAppCredentialsElAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeDeveloperAppCredentialsElAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeDeveloperAppCredentialsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_products: Option<ListField<ApigeeDeveloperAppCredentialsElApiProductsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<ListField<ApigeeDeveloperAppCredentialsElAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issued_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl ApigeeDeveloperAppCredentialsEl {
    #[doc = "Set the field `api_products`.\n"]
    pub fn set_api_products(
        mut self,
        v: impl Into<ListField<ApigeeDeveloperAppCredentialsElApiProductsEl>>,
    ) -> Self {
        self.api_products = Some(v.into());
        self
    }
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        mut self,
        v: impl Into<ListField<ApigeeDeveloperAppCredentialsElAttributesEl>>,
    ) -> Self {
        self.attributes = Some(v.into());
        self
    }
    #[doc = "Set the field `consumer_key`.\n"]
    pub fn set_consumer_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_key = Some(v.into());
        self
    }
    #[doc = "Set the field `consumer_secret`.\n"]
    pub fn set_consumer_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `expires_at`.\n"]
    pub fn set_expires_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expires_at = Some(v.into());
        self
    }
    #[doc = "Set the field `issued_at`.\n"]
    pub fn set_issued_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issued_at = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeDeveloperAppCredentialsEl {
    type O = BlockAssignable<ApigeeDeveloperAppCredentialsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeDeveloperAppCredentialsEl {}
impl BuildApigeeDeveloperAppCredentialsEl {
    pub fn build(self) -> ApigeeDeveloperAppCredentialsEl {
        ApigeeDeveloperAppCredentialsEl {
            api_products: core::default::Default::default(),
            attributes: core::default::Default::default(),
            consumer_key: core::default::Default::default(),
            consumer_secret: core::default::Default::default(),
            expires_at: core::default::Default::default(),
            issued_at: core::default::Default::default(),
            scopes: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct ApigeeDeveloperAppCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeDeveloperAppCredentialsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeDeveloperAppCredentialsElRef {
        ApigeeDeveloperAppCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeDeveloperAppCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_products` after provisioning.\n"]
    pub fn api_products(&self) -> ListRef<ApigeeDeveloperAppCredentialsElApiProductsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.api_products", self.base))
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<ApigeeDeveloperAppCredentialsElAttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }
    #[doc = "Get a reference to the value of field `consumer_key` after provisioning.\n"]
    pub fn consumer_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_key", self.base))
    }
    #[doc = "Get a reference to the value of field `consumer_secret` after provisioning.\n"]
    pub fn consumer_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }
    #[doc = "Get a reference to the value of field `issued_at` after provisioning.\n"]
    pub fn issued_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issued_at", self.base))
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeDeveloperAppAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ApigeeDeveloperAppAttributesEl {
    #[doc = "Set the field `name`.\nKey of the attribute"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nValue of the attribute"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeDeveloperAppAttributesEl {
    type O = BlockAssignable<ApigeeDeveloperAppAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeDeveloperAppAttributesEl {}
impl BuildApigeeDeveloperAppAttributesEl {
    pub fn build(self) -> ApigeeDeveloperAppAttributesEl {
        ApigeeDeveloperAppAttributesEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ApigeeDeveloperAppAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeDeveloperAppAttributesElRef {
    fn new(shared: StackShared, base: String) -> ApigeeDeveloperAppAttributesElRef {
        ApigeeDeveloperAppAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeDeveloperAppAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nKey of the attribute"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the attribute"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeDeveloperAppTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApigeeDeveloperAppTimeoutsEl {
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
impl ToListMappable for ApigeeDeveloperAppTimeoutsEl {
    type O = BlockAssignable<ApigeeDeveloperAppTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeDeveloperAppTimeoutsEl {}
impl BuildApigeeDeveloperAppTimeoutsEl {
    pub fn build(self) -> ApigeeDeveloperAppTimeoutsEl {
        ApigeeDeveloperAppTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApigeeDeveloperAppTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeDeveloperAppTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeDeveloperAppTimeoutsElRef {
        ApigeeDeveloperAppTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeDeveloperAppTimeoutsElRef {
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
struct ApigeeDeveloperAppDynamic {
    attributes: Option<DynamicBlock<ApigeeDeveloperAppAttributesEl>>,
}
