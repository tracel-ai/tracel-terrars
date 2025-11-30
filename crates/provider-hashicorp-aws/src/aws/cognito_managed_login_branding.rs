use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CognitoManagedLoginBrandingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    client_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_cognito_provided_values: Option<PrimField<bool>>,
    user_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset: Option<Vec<CognitoManagedLoginBrandingAssetEl>>,
    dynamic: CognitoManagedLoginBrandingDynamic,
}

struct CognitoManagedLoginBranding_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CognitoManagedLoginBrandingData>,
}

#[derive(Clone)]
pub struct CognitoManagedLoginBranding(Rc<CognitoManagedLoginBranding_>);

impl CognitoManagedLoginBranding {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().settings = Some(v.into());
        self
    }

    #[doc = "Set the field `use_cognito_provided_values`.\n"]
    pub fn set_use_cognito_provided_values(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_cognito_provided_values = Some(v.into());
        self
    }

    #[doc = "Set the field `asset`.\n"]
    pub fn set_asset(
        self,
        v: impl Into<BlockAssignable<CognitoManagedLoginBrandingAssetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().asset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.asset = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `managed_login_branding_id` after provisioning.\n"]
    pub fn managed_login_branding_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_login_branding_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `settings_all` after provisioning.\n"]
    pub fn settings_all(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `use_cognito_provided_values` after provisioning.\n"]
    pub fn use_cognito_provided_values(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_cognito_provided_values", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_id", self.extract_ref()),
        )
    }
}

impl Referable for CognitoManagedLoginBranding {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CognitoManagedLoginBranding {}

impl ToListMappable for CognitoManagedLoginBranding {
    type O = ListRef<CognitoManagedLoginBrandingRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CognitoManagedLoginBranding_ {
    fn extract_resource_type(&self) -> String {
        "aws_cognito_managed_login_branding".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoManagedLoginBranding {
    pub tf_id: String,
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}

impl BuildCognitoManagedLoginBranding {
    pub fn build(self, stack: &mut Stack) -> CognitoManagedLoginBranding {
        let out = CognitoManagedLoginBranding(Rc::new(CognitoManagedLoginBranding_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CognitoManagedLoginBrandingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                client_id: self.client_id,
                region: core::default::Default::default(),
                settings: core::default::Default::default(),
                use_cognito_provided_values: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
                asset: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoManagedLoginBrandingRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoManagedLoginBrandingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CognitoManagedLoginBrandingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `managed_login_branding_id` after provisioning.\n"]
    pub fn managed_login_branding_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_login_branding_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `settings_all` after provisioning.\n"]
    pub fn settings_all(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `use_cognito_provided_values` after provisioning.\n"]
    pub fn use_cognito_provided_values(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_cognito_provided_values", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_pool_id", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CognitoManagedLoginBrandingAssetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes: Option<PrimField<String>>,
    category: PrimField<String>,
    color_mode: PrimField<String>,
    extension: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<PrimField<String>>,
}

impl CognitoManagedLoginBrandingAssetEl {
    #[doc = "Set the field `bytes`.\n"]
    pub fn set_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bytes = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_id = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoManagedLoginBrandingAssetEl {
    type O = BlockAssignable<CognitoManagedLoginBrandingAssetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoManagedLoginBrandingAssetEl {
    #[doc = ""]
    pub category: PrimField<String>,
    #[doc = ""]
    pub color_mode: PrimField<String>,
    #[doc = ""]
    pub extension: PrimField<String>,
}

impl BuildCognitoManagedLoginBrandingAssetEl {
    pub fn build(self) -> CognitoManagedLoginBrandingAssetEl {
        CognitoManagedLoginBrandingAssetEl {
            bytes: core::default::Default::default(),
            category: self.category,
            color_mode: self.color_mode,
            extension: self.extension,
            resource_id: core::default::Default::default(),
        }
    }
}

pub struct CognitoManagedLoginBrandingAssetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoManagedLoginBrandingAssetElRef {
    fn new(shared: StackShared, base: String) -> CognitoManagedLoginBrandingAssetElRef {
        CognitoManagedLoginBrandingAssetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoManagedLoginBrandingAssetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bytes` after provisioning.\n"]
    pub fn bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `color_mode` after provisioning.\n"]
    pub fn color_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `extension` after provisioning.\n"]
    pub fn extension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extension", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoManagedLoginBrandingDynamic {
    asset: Option<DynamicBlock<CognitoManagedLoginBrandingAssetEl>>,
}
