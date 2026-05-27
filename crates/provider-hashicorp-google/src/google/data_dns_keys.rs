use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataDnsKeysData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    managed_zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataDnsKeys_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDnsKeysData>,
}
#[derive(Clone)]
pub struct DataDnsKeys(Rc<DataDnsKeys_>);
impl DataDnsKeys {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_signing_keys` after provisioning.\n"]
    pub fn key_signing_keys(&self) -> ListRef<DataDnsKeysKeySigningKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_signing_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_zone` after provisioning.\n"]
    pub fn managed_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_signing_keys` after provisioning.\n"]
    pub fn zone_signing_keys(&self) -> ListRef<DataDnsKeysZoneSigningKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_signing_keys", self.extract_ref()),
        )
    }
}
impl Referable for DataDnsKeys {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataDnsKeys {}
impl ToListMappable for DataDnsKeys {
    type O = ListRef<DataDnsKeysRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataDnsKeys_ {
    fn extract_datasource_type(&self) -> String {
        "google_dns_keys".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataDnsKeys {
    pub tf_id: String,
    #[doc = ""]
    pub managed_zone: PrimField<String>,
}
impl BuildDataDnsKeys {
    pub fn build(self, stack: &mut Stack) -> DataDnsKeys {
        let out = DataDnsKeys(Rc::new(DataDnsKeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDnsKeysData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                managed_zone: self.managed_zone,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataDnsKeysRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDnsKeysRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataDnsKeysRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_signing_keys` after provisioning.\n"]
    pub fn key_signing_keys(&self) -> ListRef<DataDnsKeysKeySigningKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_signing_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `managed_zone` after provisioning.\n"]
    pub fn managed_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone_signing_keys` after provisioning.\n"]
    pub fn zone_signing_keys(&self) -> ListRef<DataDnsKeysZoneSigningKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_signing_keys", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataDnsKeysKeySigningKeysElDigestsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    digest: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataDnsKeysKeySigningKeysElDigestsEl {
    #[doc = "Set the field `digest`.\n"]
    pub fn set_digest(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.digest = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataDnsKeysKeySigningKeysElDigestsEl {
    type O = BlockAssignable<DataDnsKeysKeySigningKeysElDigestsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDnsKeysKeySigningKeysElDigestsEl {}
impl BuildDataDnsKeysKeySigningKeysElDigestsEl {
    pub fn build(self) -> DataDnsKeysKeySigningKeysElDigestsEl {
        DataDnsKeysKeySigningKeysElDigestsEl {
            digest: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataDnsKeysKeySigningKeysElDigestsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDnsKeysKeySigningKeysElDigestsElRef {
    fn new(shared: StackShared, base: String) -> DataDnsKeysKeySigningKeysElDigestsElRef {
        DataDnsKeysKeySigningKeysElDigestsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDnsKeysKeySigningKeysElDigestsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `digest` after provisioning.\n"]
    pub fn digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataDnsKeysKeySigningKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    digests: Option<ListField<DataDnsKeysKeySigningKeysElDigestsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ds_record: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_active: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_tag: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<PrimField<String>>,
}
impl DataDnsKeysKeySigningKeysEl {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `creation_time`.\n"]
    pub fn set_creation_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.creation_time = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `digests`.\n"]
    pub fn set_digests(
        mut self,
        v: impl Into<ListField<DataDnsKeysKeySigningKeysElDigestsEl>>,
    ) -> Self {
        self.digests = Some(v.into());
        self
    }
    #[doc = "Set the field `ds_record`.\n"]
    pub fn set_ds_record(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ds_record = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_active`.\n"]
    pub fn set_is_active(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_active = Some(v.into());
        self
    }
    #[doc = "Set the field `key_length`.\n"]
    pub fn set_key_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.key_length = Some(v.into());
        self
    }
    #[doc = "Set the field `key_tag`.\n"]
    pub fn set_key_tag(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.key_tag = Some(v.into());
        self
    }
    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_key = Some(v.into());
        self
    }
}
impl ToListMappable for DataDnsKeysKeySigningKeysEl {
    type O = BlockAssignable<DataDnsKeysKeySigningKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDnsKeysKeySigningKeysEl {}
impl BuildDataDnsKeysKeySigningKeysEl {
    pub fn build(self) -> DataDnsKeysKeySigningKeysEl {
        DataDnsKeysKeySigningKeysEl {
            algorithm: core::default::Default::default(),
            creation_time: core::default::Default::default(),
            description: core::default::Default::default(),
            digests: core::default::Default::default(),
            ds_record: core::default::Default::default(),
            id: core::default::Default::default(),
            is_active: core::default::Default::default(),
            key_length: core::default::Default::default(),
            key_tag: core::default::Default::default(),
            public_key: core::default::Default::default(),
        }
    }
}
pub struct DataDnsKeysKeySigningKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDnsKeysKeySigningKeysElRef {
    fn new(shared: StackShared, base: String) -> DataDnsKeysKeySigningKeysElRef {
        DataDnsKeysKeySigningKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDnsKeysKeySigningKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `digests` after provisioning.\n"]
    pub fn digests(&self) -> ListRef<DataDnsKeysKeySigningKeysElDigestsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.digests", self.base))
    }
    #[doc = "Get a reference to the value of field `ds_record` after provisioning.\n"]
    pub fn ds_record(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ds_record", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `is_active` after provisioning.\n"]
    pub fn is_active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_active", self.base))
    }
    #[doc = "Get a reference to the value of field `key_length` after provisioning.\n"]
    pub fn key_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_length", self.base))
    }
    #[doc = "Get a reference to the value of field `key_tag` after provisioning.\n"]
    pub fn key_tag(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.base))
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}
#[derive(Serialize)]
pub struct DataDnsKeysZoneSigningKeysElDigestsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    digest: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataDnsKeysZoneSigningKeysElDigestsEl {
    #[doc = "Set the field `digest`.\n"]
    pub fn set_digest(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.digest = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataDnsKeysZoneSigningKeysElDigestsEl {
    type O = BlockAssignable<DataDnsKeysZoneSigningKeysElDigestsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDnsKeysZoneSigningKeysElDigestsEl {}
impl BuildDataDnsKeysZoneSigningKeysElDigestsEl {
    pub fn build(self) -> DataDnsKeysZoneSigningKeysElDigestsEl {
        DataDnsKeysZoneSigningKeysElDigestsEl {
            digest: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataDnsKeysZoneSigningKeysElDigestsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDnsKeysZoneSigningKeysElDigestsElRef {
    fn new(shared: StackShared, base: String) -> DataDnsKeysZoneSigningKeysElDigestsElRef {
        DataDnsKeysZoneSigningKeysElDigestsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDnsKeysZoneSigningKeysElDigestsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `digest` after provisioning.\n"]
    pub fn digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataDnsKeysZoneSigningKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    digests: Option<ListField<DataDnsKeysZoneSigningKeysElDigestsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_active: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_tag: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<PrimField<String>>,
}
impl DataDnsKeysZoneSigningKeysEl {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `creation_time`.\n"]
    pub fn set_creation_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.creation_time = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `digests`.\n"]
    pub fn set_digests(
        mut self,
        v: impl Into<ListField<DataDnsKeysZoneSigningKeysElDigestsEl>>,
    ) -> Self {
        self.digests = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_active`.\n"]
    pub fn set_is_active(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_active = Some(v.into());
        self
    }
    #[doc = "Set the field `key_length`.\n"]
    pub fn set_key_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.key_length = Some(v.into());
        self
    }
    #[doc = "Set the field `key_tag`.\n"]
    pub fn set_key_tag(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.key_tag = Some(v.into());
        self
    }
    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_key = Some(v.into());
        self
    }
}
impl ToListMappable for DataDnsKeysZoneSigningKeysEl {
    type O = BlockAssignable<DataDnsKeysZoneSigningKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDnsKeysZoneSigningKeysEl {}
impl BuildDataDnsKeysZoneSigningKeysEl {
    pub fn build(self) -> DataDnsKeysZoneSigningKeysEl {
        DataDnsKeysZoneSigningKeysEl {
            algorithm: core::default::Default::default(),
            creation_time: core::default::Default::default(),
            description: core::default::Default::default(),
            digests: core::default::Default::default(),
            id: core::default::Default::default(),
            is_active: core::default::Default::default(),
            key_length: core::default::Default::default(),
            key_tag: core::default::Default::default(),
            public_key: core::default::Default::default(),
        }
    }
}
pub struct DataDnsKeysZoneSigningKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDnsKeysZoneSigningKeysElRef {
    fn new(shared: StackShared, base: String) -> DataDnsKeysZoneSigningKeysElRef {
        DataDnsKeysZoneSigningKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDnsKeysZoneSigningKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `digests` after provisioning.\n"]
    pub fn digests(&self) -> ListRef<DataDnsKeysZoneSigningKeysElDigestsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.digests", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `is_active` after provisioning.\n"]
    pub fn is_active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_active", self.base))
    }
    #[doc = "Get a reference to the value of field `key_length` after provisioning.\n"]
    pub fn key_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_length", self.base))
    }
    #[doc = "Get a reference to the value of field `key_tag` after provisioning.\n"]
    pub fn key_tag(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.base))
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}
