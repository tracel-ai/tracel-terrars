use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSecretManagerSecretsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataSecretManagerSecrets_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSecretManagerSecretsData>,
}
#[derive(Clone)]
pub struct DataSecretManagerSecrets(Rc<DataSecretManagerSecrets_>);
impl DataSecretManagerSecrets {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `filter`.\nFilter string, adhering to the rules in List-operation filtering (https://cloud.google.com/secret-manager/docs/filtering).\nList only secrets matching the filter. If filter is empty, all secrets are listed."]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter string, adhering to the rules in List-operation filtering (https://cloud.google.com/secret-manager/docs/filtering).\nList only secrets matching the filter. If filter is empty, all secrets are listed."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> ListRef<DataSecretManagerSecretsSecretsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secrets", self.extract_ref()),
        )
    }
}
impl Referable for DataSecretManagerSecrets {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSecretManagerSecrets {}
impl ToListMappable for DataSecretManagerSecrets {
    type O = ListRef<DataSecretManagerSecretsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSecretManagerSecrets_ {
    fn extract_datasource_type(&self) -> String {
        "google_secret_manager_secrets".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSecretManagerSecrets {
    pub tf_id: String,
}
impl BuildDataSecretManagerSecrets {
    pub fn build(self, stack: &mut Stack) -> DataSecretManagerSecrets {
        let out = DataSecretManagerSecrets(Rc::new(DataSecretManagerSecrets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSecretManagerSecretsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSecretManagerSecretsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSecretManagerSecretsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter string, adhering to the rules in List-operation filtering (https://cloud.google.com/secret-manager/docs/filtering).\nList only secrets matching the filter. If filter is empty, all secrets are listed."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> ListRef<DataSecretManagerSecretsSecretsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secrets", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl
{
    type O = BlockAssignable<
        DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {}
impl BuildDataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {
    pub fn build(
        self,
    ) -> DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {
        DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef {
        DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationElAutoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_encryption: Option<
        ListField<DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl>,
    >,
}
impl DataSecretManagerSecretsSecretsElReplicationElAutoEl {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v: impl Into<
            ListField<
                DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl,
            >,
        >,
    ) -> Self {
        self.customer_managed_encryption = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretsSecretsElReplicationElAutoEl {
    type O = BlockAssignable<DataSecretManagerSecretsSecretsElReplicationElAutoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretsSecretsElReplicationElAutoEl {}
impl BuildDataSecretManagerSecretsSecretsElReplicationElAutoEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElReplicationElAutoEl {
        DataSecretManagerSecretsSecretsElReplicationElAutoEl {
            customer_managed_encryption: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretsSecretsElReplicationElAutoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretsSecretsElReplicationElAutoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerSecretsSecretsElReplicationElAutoElRef {
        DataSecretManagerSecretsSecretsElReplicationElAutoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretsSecretsElReplicationElAutoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl
    DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl
{
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl { type O = BlockAssignable < DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl
{}
impl BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl { pub fn build (self) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl { DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl { kms_key_name : core :: default :: Default :: default () , } } }
pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef { fn new (shared : StackShared , base : String) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef { DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef { shared : shared , base : base . to_string () , } } }
impl DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"] pub fn kms_key_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.kms_key_name" , self . base)) } }
#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl { # [serde (skip_serializing_if = "Option::is_none")] customer_managed_encryption : Option < ListField < DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl > > , # [serde (skip_serializing_if = "Option::is_none")] location : Option < PrimField < String > > , }
impl DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v : impl Into < ListField < DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl > >,
    ) -> Self {
        self.customer_managed_encryption = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {
    type O = BlockAssignable<DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {}
impl BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {
        DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {
            customer_managed_encryption: core::default::Default::default(),
            location: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef {
        DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]    pub fn customer_managed_encryption (& self) -> ListRef < DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas:
        Option<ListField<DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl>>,
}
impl DataSecretManagerSecretsSecretsElReplicationElUserManagedEl {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl Into<ListField<DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl>>,
    ) -> Self {
        self.replicas = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretsSecretsElReplicationElUserManagedEl {
    type O = BlockAssignable<DataSecretManagerSecretsSecretsElReplicationElUserManagedEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedEl {}
impl BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedEl {
        DataSecretManagerSecretsSecretsElReplicationElUserManagedEl {
            replicas: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef {
        DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(
        &self,
    ) -> ListRef<DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto: Option<ListField<DataSecretManagerSecretsSecretsElReplicationElAutoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_managed: Option<ListField<DataSecretManagerSecretsSecretsElReplicationElUserManagedEl>>,
}
impl DataSecretManagerSecretsSecretsElReplicationEl {
    #[doc = "Set the field `auto`.\n"]
    pub fn set_auto(
        mut self,
        v: impl Into<ListField<DataSecretManagerSecretsSecretsElReplicationElAutoEl>>,
    ) -> Self {
        self.auto = Some(v.into());
        self
    }
    #[doc = "Set the field `user_managed`.\n"]
    pub fn set_user_managed(
        mut self,
        v: impl Into<ListField<DataSecretManagerSecretsSecretsElReplicationElUserManagedEl>>,
    ) -> Self {
        self.user_managed = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretsSecretsElReplicationEl {
    type O = BlockAssignable<DataSecretManagerSecretsSecretsElReplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretsSecretsElReplicationEl {}
impl BuildDataSecretManagerSecretsSecretsElReplicationEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElReplicationEl {
        DataSecretManagerSecretsSecretsElReplicationEl {
            auto: core::default::Default::default(),
            user_managed: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretsSecretsElReplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretsSecretsElReplicationElRef {
    fn new(shared: StackShared, base: String) -> DataSecretManagerSecretsSecretsElReplicationElRef {
        DataSecretManagerSecretsSecretsElReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretsSecretsElReplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto` after provisioning.\n"]
    pub fn auto(&self) -> ListRef<DataSecretManagerSecretsSecretsElReplicationElAutoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto", self.base))
    }
    #[doc = "Get a reference to the value of field `user_managed` after provisioning.\n"]
    pub fn user_managed(
        &self,
    ) -> ListRef<DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_managed", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElRotationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    next_rotation_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_period: Option<PrimField<String>>,
}
impl DataSecretManagerSecretsSecretsElRotationEl {
    #[doc = "Set the field `next_rotation_time`.\n"]
    pub fn set_next_rotation_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_rotation_time = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_period`.\n"]
    pub fn set_rotation_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rotation_period = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretsSecretsElRotationEl {
    type O = BlockAssignable<DataSecretManagerSecretsSecretsElRotationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretsSecretsElRotationEl {}
impl BuildDataSecretManagerSecretsSecretsElRotationEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElRotationEl {
        DataSecretManagerSecretsSecretsElRotationEl {
            next_rotation_time: core::default::Default::default(),
            rotation_period: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretsSecretsElRotationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretsSecretsElRotationElRef {
    fn new(shared: StackShared, base: String) -> DataSecretManagerSecretsSecretsElRotationElRef {
        DataSecretManagerSecretsSecretsElRotationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretsSecretsElRotationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `next_rotation_time` after provisioning.\n"]
    pub fn next_rotation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_rotation_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rotation_period` after provisioning.\n"]
    pub fn rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_period", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElTopicsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataSecretManagerSecretsSecretsElTopicsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretsSecretsElTopicsEl {
    type O = BlockAssignable<DataSecretManagerSecretsSecretsElTopicsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretsSecretsElTopicsEl {}
impl BuildDataSecretManagerSecretsSecretsElTopicsEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElTopicsEl {
        DataSecretManagerSecretsSecretsElTopicsEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretsSecretsElTopicsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretsSecretsElTopicsElRef {
    fn new(shared: StackShared, base: String) -> DataSecretManagerSecretsSecretsElTopicsElRef {
        DataSecretManagerSecretsSecretsElTopicsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretsSecretsElTopicsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication: Option<ListField<DataSecretManagerSecretsSecretsElReplicationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<ListField<DataSecretManagerSecretsSecretsElRotationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terraform_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topics: Option<ListField<DataSecretManagerSecretsSecretsElTopicsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_aliases: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_destroy_ttl: Option<PrimField<String>>,
}
impl DataSecretManagerSecretsSecretsEl {
    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_annotations`.\n"]
    pub fn set_effective_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.effective_annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_labels`.\n"]
    pub fn set_effective_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.effective_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expire_time = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `replication`.\n"]
    pub fn set_replication(
        mut self,
        v: impl Into<ListField<DataSecretManagerSecretsSecretsElReplicationEl>>,
    ) -> Self {
        self.replication = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation`.\n"]
    pub fn set_rotation(
        mut self,
        v: impl Into<ListField<DataSecretManagerSecretsSecretsElRotationEl>>,
    ) -> Self {
        self.rotation = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_id`.\n"]
    pub fn set_secret_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_id = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.terraform_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `topics`.\n"]
    pub fn set_topics(
        mut self,
        v: impl Into<ListField<DataSecretManagerSecretsSecretsElTopicsEl>>,
    ) -> Self {
        self.topics = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `version_aliases`.\n"]
    pub fn set_version_aliases(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.version_aliases = Some(v.into());
        self
    }
    #[doc = "Set the field `version_destroy_ttl`.\n"]
    pub fn set_version_destroy_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_destroy_ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretsSecretsEl {
    type O = BlockAssignable<DataSecretManagerSecretsSecretsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretsSecretsEl {}
impl BuildDataSecretManagerSecretsSecretsEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsEl {
        DataSecretManagerSecretsSecretsEl {
            annotations: core::default::Default::default(),
            create_time: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            deletion_protection: core::default::Default::default(),
            effective_annotations: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            expire_time: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
            replication: core::default::Default::default(),
            rotation: core::default::Default::default(),
            secret_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
            topics: core::default::Default::default(),
            ttl: core::default::Default::default(),
            version_aliases: core::default::Default::default(),
            version_destroy_ttl: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretsSecretsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretsSecretsElRef {
    fn new(shared: StackShared, base: String) -> DataSecretManagerSecretsSecretsElRef {
        DataSecretManagerSecretsSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretsSecretsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\n"]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `replication` after provisioning.\n"]
    pub fn replication(&self) -> ListRef<DataSecretManagerSecretsSecretsElReplicationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replication", self.base))
    }
    #[doc = "Get a reference to the value of field `rotation` after provisioning.\n"]
    pub fn rotation(&self) -> ListRef<DataSecretManagerSecretsSecretsElRotationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rotation", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> ListRef<DataSecretManagerSecretsSecretsElTopicsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.topics", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `version_aliases` after provisioning.\n"]
    pub fn version_aliases(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.version_aliases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version_destroy_ttl` after provisioning.\n"]
    pub fn version_destroy_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_destroy_ttl", self.base),
        )
    }
}
