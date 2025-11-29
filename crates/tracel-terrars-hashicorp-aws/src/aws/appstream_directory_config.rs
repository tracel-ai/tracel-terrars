use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppstreamDirectoryConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    directory_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    organizational_unit_distinguished_names: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_based_auth_properties: Option<Vec<AppstreamDirectoryConfigCertificateBasedAuthPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_credentials: Option<Vec<AppstreamDirectoryConfigServiceAccountCredentialsEl>>,
    dynamic: AppstreamDirectoryConfigDynamic,
}

struct AppstreamDirectoryConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppstreamDirectoryConfigData>,
}

#[derive(Clone)]
pub struct AppstreamDirectoryConfig(Rc<AppstreamDirectoryConfig_>);

impl AppstreamDirectoryConfig {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_based_auth_properties`.\n"]
    pub fn set_certificate_based_auth_properties(
        self,
        v: impl Into<BlockAssignable<AppstreamDirectoryConfigCertificateBasedAuthPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certificate_based_auth_properties = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.certificate_based_auth_properties = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_account_credentials`.\n"]
    pub fn set_service_account_credentials(
        self,
        v: impl Into<BlockAssignable<AppstreamDirectoryConfigServiceAccountCredentialsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_account_credentials = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_account_credentials = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_distinguished_names` after provisioning.\n"]
    pub fn organizational_unit_distinguished_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.organizational_unit_distinguished_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_based_auth_properties` after provisioning.\n"]
    pub fn certificate_based_auth_properties(
        &self,
    ) -> ListRef<AppstreamDirectoryConfigCertificateBasedAuthPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_based_auth_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_account_credentials` after provisioning.\n"]
    pub fn service_account_credentials(&self) -> ListRef<AppstreamDirectoryConfigServiceAccountCredentialsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_account_credentials", self.extract_ref()))
    }
}

impl Referable for AppstreamDirectoryConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AppstreamDirectoryConfig { }

impl ToListMappable for AppstreamDirectoryConfig {
    type O = ListRef<AppstreamDirectoryConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppstreamDirectoryConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_appstream_directory_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppstreamDirectoryConfig {
    pub tf_id: String,
    #[doc = ""]
    pub directory_name: PrimField<String>,
    #[doc = ""]
    pub organizational_unit_distinguished_names: SetField<PrimField<String>>,
}

impl BuildAppstreamDirectoryConfig {
    pub fn build(self, stack: &mut Stack) -> AppstreamDirectoryConfig {
        let out = AppstreamDirectoryConfig(Rc::new(AppstreamDirectoryConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppstreamDirectoryConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                directory_name: self.directory_name,
                id: core::default::Default::default(),
                organizational_unit_distinguished_names: self.organizational_unit_distinguished_names,
                region: core::default::Default::default(),
                certificate_based_auth_properties: core::default::Default::default(),
                service_account_credentials: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppstreamDirectoryConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamDirectoryConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AppstreamDirectoryConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_distinguished_names` after provisioning.\n"]
    pub fn organizational_unit_distinguished_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.organizational_unit_distinguished_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_based_auth_properties` after provisioning.\n"]
    pub fn certificate_based_auth_properties(
        &self,
    ) -> ListRef<AppstreamDirectoryConfigCertificateBasedAuthPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_based_auth_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_account_credentials` after provisioning.\n"]
    pub fn service_account_credentials(&self) -> ListRef<AppstreamDirectoryConfigServiceAccountCredentialsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_account_credentials", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppstreamDirectoryConfigCertificateBasedAuthPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl AppstreamDirectoryConfigCertificateBasedAuthPropertiesEl {
    #[doc = "Set the field `certificate_authority_arn`.\n"]
    pub fn set_certificate_authority_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_authority_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for AppstreamDirectoryConfigCertificateBasedAuthPropertiesEl {
    type O = BlockAssignable<AppstreamDirectoryConfigCertificateBasedAuthPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamDirectoryConfigCertificateBasedAuthPropertiesEl {}

impl BuildAppstreamDirectoryConfigCertificateBasedAuthPropertiesEl {
    pub fn build(self) -> AppstreamDirectoryConfigCertificateBasedAuthPropertiesEl {
        AppstreamDirectoryConfigCertificateBasedAuthPropertiesEl {
            certificate_authority_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct AppstreamDirectoryConfigCertificateBasedAuthPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamDirectoryConfigCertificateBasedAuthPropertiesElRef {
    fn new(shared: StackShared, base: String) -> AppstreamDirectoryConfigCertificateBasedAuthPropertiesElRef {
        AppstreamDirectoryConfigCertificateBasedAuthPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamDirectoryConfigCertificateBasedAuthPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamDirectoryConfigServiceAccountCredentialsEl {
    account_name: PrimField<String>,
    account_password: PrimField<String>,
}

impl AppstreamDirectoryConfigServiceAccountCredentialsEl { }

impl ToListMappable for AppstreamDirectoryConfigServiceAccountCredentialsEl {
    type O = BlockAssignable<AppstreamDirectoryConfigServiceAccountCredentialsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamDirectoryConfigServiceAccountCredentialsEl {
    #[doc = ""]
    pub account_name: PrimField<String>,
    #[doc = ""]
    pub account_password: PrimField<String>,
}

impl BuildAppstreamDirectoryConfigServiceAccountCredentialsEl {
    pub fn build(self) -> AppstreamDirectoryConfigServiceAccountCredentialsEl {
        AppstreamDirectoryConfigServiceAccountCredentialsEl {
            account_name: self.account_name,
            account_password: self.account_password,
        }
    }
}

pub struct AppstreamDirectoryConfigServiceAccountCredentialsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamDirectoryConfigServiceAccountCredentialsElRef {
    fn new(shared: StackShared, base: String) -> AppstreamDirectoryConfigServiceAccountCredentialsElRef {
        AppstreamDirectoryConfigServiceAccountCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamDirectoryConfigServiceAccountCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_name` after provisioning.\n"]
    pub fn account_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.base))
    }

    #[doc = "Get a reference to the value of field `account_password` after provisioning.\n"]
    pub fn account_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_password", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppstreamDirectoryConfigDynamic {
    certificate_based_auth_properties: Option<DynamicBlock<AppstreamDirectoryConfigCertificateBasedAuthPropertiesEl>>,
    service_account_credentials: Option<DynamicBlock<AppstreamDirectoryConfigServiceAccountCredentialsEl>>,
}
