use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeStoragePoolTypesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    storage_pool_type: PrimField<String>,
    zone: PrimField<String>,
}
struct DataComputeStoragePoolTypes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeStoragePoolTypesData>,
}
#[derive(Clone)]
pub struct DataComputeStoragePoolTypes(Rc<DataComputeStoragePoolTypes_>);
impl DataComputeStoragePoolTypes {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deprecated` after provisioning.\nThe deprecation status associated with this storage pool type."]
    pub fn deprecated(&self) -> ListRef<DataComputeStoragePoolTypesDeprecatedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deprecated", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#storagePoolType for storage pool types."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pool_provisioned_capacity_gb` after provisioning.\nMaximum storage pool size in GB."]
    pub fn max_pool_provisioned_capacity_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pool_provisioned_capacity_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pool_provisioned_iops` after provisioning.\nMaximum provisioned IOPS."]
    pub fn max_pool_provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pool_provisioned_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pool_provisioned_throughput` after provisioning.\nMaximum provisioned throughput."]
    pub fn max_pool_provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pool_provisioned_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_pool_provisioned_capacity_gb` after provisioning.\nMinimum storage pool size in GB."]
    pub fn min_pool_provisioned_capacity_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_pool_provisioned_capacity_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_pool_provisioned_iops` after provisioning.\nMinimum provisioned IOPS."]
    pub fn min_pool_provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_pool_provisioned_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_pool_provisioned_throughput` after provisioning.\nMinimum provisioned throughput."]
    pub fn min_pool_provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_pool_provisioned_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_with_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_pool_type` after provisioning.\nName of the storage pool type."]
    pub fn storage_pool_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_pool_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_disk_types` after provisioning.\nThe list of disk types supported in this storage pool type."]
    pub fn supported_disk_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_disk_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe name of the zone."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeStoragePoolTypes {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeStoragePoolTypes {}
impl ToListMappable for DataComputeStoragePoolTypes {
    type O = ListRef<DataComputeStoragePoolTypesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeStoragePoolTypes_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_storage_pool_types".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeStoragePoolTypes {
    pub tf_id: String,
    #[doc = "Name of the storage pool type."]
    pub storage_pool_type: PrimField<String>,
    #[doc = "The name of the zone."]
    pub zone: PrimField<String>,
}
impl BuildDataComputeStoragePoolTypes {
    pub fn build(self, stack: &mut Stack) -> DataComputeStoragePoolTypes {
        let out = DataComputeStoragePoolTypes(Rc::new(DataComputeStoragePoolTypes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeStoragePoolTypesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                project: core::default::Default::default(),
                storage_pool_type: self.storage_pool_type,
                zone: self.zone,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeStoragePoolTypesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeStoragePoolTypesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeStoragePoolTypesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deprecated` after provisioning.\nThe deprecation status associated with this storage pool type."]
    pub fn deprecated(&self) -> ListRef<DataComputeStoragePoolTypesDeprecatedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deprecated", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#storagePoolType for storage pool types."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pool_provisioned_capacity_gb` after provisioning.\nMaximum storage pool size in GB."]
    pub fn max_pool_provisioned_capacity_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pool_provisioned_capacity_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pool_provisioned_iops` after provisioning.\nMaximum provisioned IOPS."]
    pub fn max_pool_provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pool_provisioned_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_pool_provisioned_throughput` after provisioning.\nMaximum provisioned throughput."]
    pub fn max_pool_provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pool_provisioned_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_pool_provisioned_capacity_gb` after provisioning.\nMinimum storage pool size in GB."]
    pub fn min_pool_provisioned_capacity_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_pool_provisioned_capacity_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_pool_provisioned_iops` after provisioning.\nMinimum provisioned IOPS."]
    pub fn min_pool_provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_pool_provisioned_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_pool_provisioned_throughput` after provisioning.\nMinimum provisioned throughput."]
    pub fn min_pool_provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_pool_provisioned_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_with_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_pool_type` after provisioning.\nName of the storage pool type."]
    pub fn storage_pool_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_pool_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_disk_types` after provisioning.\nThe list of disk types supported in this storage pool type."]
    pub fn supported_disk_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_disk_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe name of the zone."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeStoragePoolTypesDeprecatedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deleted: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecated: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    obsolete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataComputeStoragePoolTypesDeprecatedEl {
    #[doc = "Set the field `deleted`.\n"]
    pub fn set_deleted(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deleted = Some(v.into());
        self
    }
    #[doc = "Set the field `deprecated`.\n"]
    pub fn set_deprecated(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deprecated = Some(v.into());
        self
    }
    #[doc = "Set the field `obsolete`.\n"]
    pub fn set_obsolete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.obsolete = Some(v.into());
        self
    }
    #[doc = "Set the field `replacement`.\n"]
    pub fn set_replacement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replacement = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeStoragePoolTypesDeprecatedEl {
    type O = BlockAssignable<DataComputeStoragePoolTypesDeprecatedEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeStoragePoolTypesDeprecatedEl {}
impl BuildDataComputeStoragePoolTypesDeprecatedEl {
    pub fn build(self) -> DataComputeStoragePoolTypesDeprecatedEl {
        DataComputeStoragePoolTypesDeprecatedEl {
            deleted: core::default::Default::default(),
            deprecated: core::default::Default::default(),
            obsolete: core::default::Default::default(),
            replacement: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataComputeStoragePoolTypesDeprecatedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeStoragePoolTypesDeprecatedElRef {
    fn new(shared: StackShared, base: String) -> DataComputeStoragePoolTypesDeprecatedElRef {
        DataComputeStoragePoolTypesDeprecatedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeStoragePoolTypesDeprecatedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.base))
    }
    #[doc = "Get a reference to the value of field `deprecated` after provisioning.\n"]
    pub fn deprecated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deprecated", self.base))
    }
    #[doc = "Get a reference to the value of field `obsolete` after provisioning.\n"]
    pub fn obsolete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.obsolete", self.base))
    }
    #[doc = "Get a reference to the value of field `replacement` after provisioning.\n"]
    pub fn replacement(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replacement", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
