use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueDataCatalogEncryptionSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_catalog_encryption_settings: Option<Vec<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl>>,
    dynamic: GlueDataCatalogEncryptionSettingsDynamic,
}

struct GlueDataCatalogEncryptionSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlueDataCatalogEncryptionSettingsData>,
}

#[derive(Clone)]
pub struct GlueDataCatalogEncryptionSettings(Rc<GlueDataCatalogEncryptionSettings_>);

impl GlueDataCatalogEncryptionSettings {
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

    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().catalog_id = Some(v.into());
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

    #[doc = "Set the field `data_catalog_encryption_settings`.\n"]
    pub fn set_data_catalog_encryption_settings(
        self,
        v: impl Into<BlockAssignable<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_catalog_encryption_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_catalog_encryption_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_catalog_encryption_settings` after provisioning.\n"]
    pub fn data_catalog_encryption_settings(
        &self,
    ) -> ListRef<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_catalog_encryption_settings", self.extract_ref()))
    }
}

impl Referable for GlueDataCatalogEncryptionSettings {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GlueDataCatalogEncryptionSettings { }

impl ToListMappable for GlueDataCatalogEncryptionSettings {
    type O = ListRef<GlueDataCatalogEncryptionSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GlueDataCatalogEncryptionSettings_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_data_catalog_encryption_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueDataCatalogEncryptionSettings {
    pub tf_id: String,
}

impl BuildGlueDataCatalogEncryptionSettings {
    pub fn build(self, stack: &mut Stack) -> GlueDataCatalogEncryptionSettings {
        let out = GlueDataCatalogEncryptionSettings(Rc::new(GlueDataCatalogEncryptionSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlueDataCatalogEncryptionSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                catalog_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                data_catalog_encryption_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueDataCatalogEncryptionSettingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueDataCatalogEncryptionSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl GlueDataCatalogEncryptionSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_catalog_encryption_settings` after provisioning.\n"]
    pub fn data_catalog_encryption_settings(
        &self,
    ) -> ListRef<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_catalog_encryption_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_kms_key_id: Option<PrimField<String>>,
    return_connection_password_encrypted: PrimField<bool>,
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    #[doc = "Set the field `aws_kms_key_id`.\n"]
    pub fn set_aws_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aws_kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    type O =
        BlockAssignable<
            GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    #[doc = ""]
    pub return_connection_password_encrypted: PrimField<bool>,
}

impl BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
    pub fn build(
        self,
    ) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl {
            aws_kms_key_id: core::default::Default::default(),
            return_connection_password_encrypted: self.return_connection_password_encrypted,
        }
    }
}

pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_kms_key_id` after provisioning.\n"]
    pub fn aws_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `return_connection_password_encrypted` after provisioning.\n"]
    pub fn return_connection_password_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.return_connection_password_encrypted", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    catalog_encryption_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_encryption_service_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_aws_kms_key_id: Option<PrimField<String>>,
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    #[doc = "Set the field `catalog_encryption_service_role`.\n"]
    pub fn set_catalog_encryption_service_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_encryption_service_role = Some(v.into());
        self
    }

    #[doc = "Set the field `sse_aws_kms_key_id`.\n"]
    pub fn set_sse_aws_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sse_aws_kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    type O = BlockAssignable<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    #[doc = ""]
    pub catalog_encryption_mode: PrimField<String>,
}

impl BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    pub fn build(self) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
            catalog_encryption_mode: self.catalog_encryption_mode,
            catalog_encryption_service_role: core::default::Default::default(),
            sse_aws_kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_encryption_mode` after provisioning.\n"]
    pub fn catalog_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_encryption_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `catalog_encryption_service_role` after provisioning.\n"]
    pub fn catalog_encryption_service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_encryption_service_role", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_aws_kms_key_id` after provisioning.\n"]
    pub fn sse_aws_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sse_aws_kms_key_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElDynamic {
    connection_password_encryption: Option<
        DynamicBlock<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl>,
    >,
    encryption_at_rest: Option<
        DynamicBlock<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl>,
    >,
}

#[derive(Serialize)]
pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_password_encryption: Option<
        Vec<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_at_rest: Option<Vec<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl>>,
    dynamic: GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElDynamic,
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    #[doc = "Set the field `connection_password_encryption`.\n"]
    pub fn set_connection_password_encryption(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connection_password_encryption = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_password_encryption = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_at_rest`.\n"]
    pub fn set_encryption_at_rest(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_at_rest = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_at_rest = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    type O = BlockAssignable<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {}

impl BuildGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    pub fn build(self) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
            connection_password_encryption: core::default::Default::default(),
            encryption_at_rest: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    fn new(shared: StackShared, base: String) -> GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
        GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_password_encryption` after provisioning.\n"]
    pub fn connection_password_encryption(
        &self,
    ) -> ListRef<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connection_password_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(
        &self,
    ) -> ListRef<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_at_rest", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueDataCatalogEncryptionSettingsDynamic {
    data_catalog_encryption_settings: Option<
        DynamicBlock<GlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl>,
    >,
}
