use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataGlueDataCatalogEncryptionSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    catalog_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataGlueDataCatalogEncryptionSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGlueDataCatalogEncryptionSettingsData>,
}
#[derive(Clone)]
pub struct DataGlueDataCatalogEncryptionSettings(Rc<DataGlueDataCatalogEncryptionSettings_>);
impl DataGlueDataCatalogEncryptionSettings {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_catalog_encryption_settings` after provisioning.\n"]
    pub fn data_catalog_encryption_settings(
        &self,
    ) -> ListRef<DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_catalog_encryption_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for DataGlueDataCatalogEncryptionSettings {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataGlueDataCatalogEncryptionSettings {}
impl ToListMappable for DataGlueDataCatalogEncryptionSettings {
    type O = ListRef<DataGlueDataCatalogEncryptionSettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataGlueDataCatalogEncryptionSettings_ {
    fn extract_datasource_type(&self) -> String {
        "aws_glue_data_catalog_encryption_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataGlueDataCatalogEncryptionSettings {
    pub tf_id: String,
    #[doc = ""]
    pub catalog_id: PrimField<String>,
}
impl BuildDataGlueDataCatalogEncryptionSettings {
    pub fn build(self, stack: &mut Stack) -> DataGlueDataCatalogEncryptionSettings {
        let out = DataGlueDataCatalogEncryptionSettings(Rc::new(
            DataGlueDataCatalogEncryptionSettings_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataGlueDataCatalogEncryptionSettingsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    catalog_id: self.catalog_id,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataGlueDataCatalogEncryptionSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueDataCatalogEncryptionSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataGlueDataCatalogEncryptionSettingsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_catalog_encryption_settings` after provisioning.\n"]
    pub fn data_catalog_encryption_settings(
        &self,
    ) -> ListRef<DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_catalog_encryption_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_connection_password_encrypted: Option<PrimField<bool>>,
}
impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl { # [doc = "Set the field `aws_kms_key_id`.\n"] pub fn set_aws_kms_key_id (mut self , v : impl Into < PrimField < String > >) -> Self { self . aws_kms_key_id = Some (v . into ()) ; self } # [doc = "Set the field `return_connection_password_encrypted`.\n"] pub fn set_return_connection_password_encrypted (mut self , v : impl Into < PrimField < bool > >) -> Self { self . return_connection_password_encrypted = Some (v . into ()) ; self } }
impl ToListMappable for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl { type O = BlockAssignable < DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl
{}
impl BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl { pub fn build (self) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl { DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl { aws_kms_key_id : core :: default :: Default :: default () , return_connection_password_encrypted : core :: default :: Default :: default () , } } }
pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef { fn new (shared : StackShared , base : String) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef { DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef { shared : shared , base : base . to_string () , } } }
impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `aws_kms_key_id` after provisioning.\n"] pub fn aws_kms_key_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.aws_kms_key_id" , self . base)) } # [doc = "Get a reference to the value of field `return_connection_password_encrypted` after provisioning.\n"] pub fn return_connection_password_encrypted (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.return_connection_password_encrypted" , self . base)) } }
#[derive(Serialize)]
pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_encryption_service_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_aws_kms_key_id: Option<PrimField<String>>,
}
impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    #[doc = "Set the field `catalog_encryption_mode`.\n"]
    pub fn set_catalog_encryption_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_encryption_mode = Some(v.into());
        self
    }
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
impl ToListMappable
    for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl
{
    type O = BlockAssignable<
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl
{}
impl BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
    pub fn build(
        self,
    ) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl
    {
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl {
            catalog_encryption_mode: core::default::Default::default(),
            catalog_encryption_service_role: core::default::Default::default(),
            sse_aws_kms_key_id: core::default::Default::default(),
        }
    }
}
pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef
    {
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog_encryption_mode` after provisioning.\n"]
    pub fn catalog_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_encryption_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `catalog_encryption_service_role` after provisioning.\n"]
    pub fn catalog_encryption_service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_encryption_service_role", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sse_aws_kms_key_id` after provisioning.\n"]
    pub fn sse_aws_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sse_aws_kms_key_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] connection_password_encryption : Option < ListField < DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl > > , # [serde (skip_serializing_if = "Option::is_none")] encryption_at_rest : Option < ListField < DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl > > , }
impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    #[doc = "Set the field `connection_password_encryption`.\n"]
    pub fn set_connection_password_encryption(
        mut self,
        v : impl Into < ListField < DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionEl > >,
    ) -> Self {
        self.connection_password_encryption = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_at_rest`.\n"]
    pub fn set_encryption_at_rest(
        mut self,
        v : impl Into < ListField < DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestEl > >,
    ) -> Self {
        self.encryption_at_rest = Some(v.into());
        self
    }
}
impl ToListMappable for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    type O = BlockAssignable<DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {}
impl BuildDataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
    pub fn build(self) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsEl {
            connection_password_encryption: core::default::Default::default(),
            encryption_at_rest: core::default::Default::default(),
        }
    }
}
pub struct DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_password_encryption` after provisioning.\n"]    pub fn connection_password_encryption (& self) -> ListRef < DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElConnectionPasswordEncryptionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_password_encryption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(
        &self,
    ) -> ListRef<
        DataGlueDataCatalogEncryptionSettingsDataCatalogEncryptionSettingsElEncryptionAtRestElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_at_rest", self.base),
        )
    }
}
