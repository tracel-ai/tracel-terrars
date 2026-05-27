use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BiglakeIcebergCatalogData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    catalog_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credential_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BiglakeIcebergCatalogTimeoutsEl>,
}
struct BiglakeIcebergCatalog_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BiglakeIcebergCatalogData>,
}
#[derive(Clone)]
pub struct BiglakeIcebergCatalog(Rc<BiglakeIcebergCatalog_>);
impl BiglakeIcebergCatalog {
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
    #[doc = "Set the field `credential_mode`.\nThe credential mode used for the catalog. CREDENTIAL_MODE_END_USER - End user credentials, default. The authenticating user must have access to the catalog resources and the corresponding Google Cloud Storage files. CREDENTIAL_MODE_VENDED_CREDENTIALS - Use credential vending. The authenticating user must have access to the catalog resources and the system will provide the caller with downscoped credentials to access the Google Cloud Storage files. All table operations in this mode would require 'X-Iceberg-Access-Delegation' header with 'vended-credentials' value included. System will generate a service account and the catalog administrator must grant the service account appropriate permissions. Possible values: [\"CREDENTIAL_MODE_END_USER\", \"CREDENTIAL_MODE_VENDED_CREDENTIALS\"]"]
    pub fn set_credential_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().credential_mode = Some(v.into());
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
    #[doc = "Set the field `primary_location`.\nThe primary location for mirroring the remote catalog metadata. It must be\na BigLake-supported location, and it should be proximate to the remote\ncatalog's location."]
    pub fn set_primary_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().primary_location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BiglakeIcebergCatalogTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `biglake_service_account` after provisioning.\nOutput only. The service account used for credential vending. It might be empty if credential vending was never enabled for the catalog."]
    pub fn biglake_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.biglake_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `catalog_type` after provisioning.\nThe catalog type of the IcebergCatalog. Currently only supports the type for Google Cloud Storage Buckets. Possible values: [\"CATALOG_TYPE_GCS_BUCKET\"]"]
    pub fn catalog_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time of the IcebergCatalog."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credential_mode` after provisioning.\nThe credential mode used for the catalog. CREDENTIAL_MODE_END_USER - End user credentials, default. The authenticating user must have access to the catalog resources and the corresponding Google Cloud Storage files. CREDENTIAL_MODE_VENDED_CREDENTIALS - Use credential vending. The authenticating user must have access to the catalog resources and the system will provide the caller with downscoped credentials to access the Google Cloud Storage files. All table operations in this mode would require 'X-Iceberg-Access-Delegation' header with 'vended-credentials' value included. System will generate a service account and the catalog administrator must grant the service account appropriate permissions. Possible values: [\"CREDENTIAL_MODE_END_USER\", \"CREDENTIAL_MODE_VENDED_CREDENTIALS\"]"]
    pub fn credential_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_location` after provisioning.\nOutput only. The default storage location for the catalog, e.g., 'gs://my-bucket'."]
    pub fn default_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the IcebergCatalog.\nFor CATALOG_TYPE_GCS_BUCKET typed catalogs, the name needs to be the\nexact same value of the GCS bucket's name. For example, for a bucket:\ngs://bucket-name, the catalog name will be exactly \"bucket-name\"."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_location` after provisioning.\nThe primary location for mirroring the remote catalog metadata. It must be\na BigLake-supported location, and it should be proximate to the remote\ncatalog's location."]
    pub fn primary_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\nOutput only. The replicas for the catalog metadata."]
    pub fn replicas(&self) -> ListRef<BiglakeIcebergCatalogReplicasElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replicas", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_regions` after provisioning.\nOutput only. The GCP region(s) where the physical metadata for the tables is stored, e.g. 'us-central1', 'nam4' or 'us'. This will contain one value for all locations, except for the catalogs that are configured to use custom dual region buckets."]
    pub fn storage_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last modification time of the IcebergCatalog."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeIcebergCatalogTimeoutsElRef {
        BiglakeIcebergCatalogTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BiglakeIcebergCatalog {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BiglakeIcebergCatalog {}
impl ToListMappable for BiglakeIcebergCatalog {
    type O = ListRef<BiglakeIcebergCatalogRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BiglakeIcebergCatalog_ {
    fn extract_resource_type(&self) -> String {
        "google_biglake_iceberg_catalog".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBiglakeIcebergCatalog {
    pub tf_id: String,
    #[doc = "The catalog type of the IcebergCatalog. Currently only supports the type for Google Cloud Storage Buckets. Possible values: [\"CATALOG_TYPE_GCS_BUCKET\"]"]
    pub catalog_type: PrimField<String>,
    #[doc = "The name of the IcebergCatalog.\nFor CATALOG_TYPE_GCS_BUCKET typed catalogs, the name needs to be the\nexact same value of the GCS bucket's name. For example, for a bucket:\ngs://bucket-name, the catalog name will be exactly \"bucket-name\"."]
    pub name: PrimField<String>,
}
impl BuildBiglakeIcebergCatalog {
    pub fn build(self, stack: &mut Stack) -> BiglakeIcebergCatalog {
        let out = BiglakeIcebergCatalog(Rc::new(BiglakeIcebergCatalog_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BiglakeIcebergCatalogData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                catalog_type: self.catalog_type,
                credential_mode: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                primary_location: core::default::Default::default(),
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BiglakeIcebergCatalogRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeIcebergCatalogRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BiglakeIcebergCatalogRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `biglake_service_account` after provisioning.\nOutput only. The service account used for credential vending. It might be empty if credential vending was never enabled for the catalog."]
    pub fn biglake_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.biglake_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `catalog_type` after provisioning.\nThe catalog type of the IcebergCatalog. Currently only supports the type for Google Cloud Storage Buckets. Possible values: [\"CATALOG_TYPE_GCS_BUCKET\"]"]
    pub fn catalog_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time of the IcebergCatalog."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credential_mode` after provisioning.\nThe credential mode used for the catalog. CREDENTIAL_MODE_END_USER - End user credentials, default. The authenticating user must have access to the catalog resources and the corresponding Google Cloud Storage files. CREDENTIAL_MODE_VENDED_CREDENTIALS - Use credential vending. The authenticating user must have access to the catalog resources and the system will provide the caller with downscoped credentials to access the Google Cloud Storage files. All table operations in this mode would require 'X-Iceberg-Access-Delegation' header with 'vended-credentials' value included. System will generate a service account and the catalog administrator must grant the service account appropriate permissions. Possible values: [\"CREDENTIAL_MODE_END_USER\", \"CREDENTIAL_MODE_VENDED_CREDENTIALS\"]"]
    pub fn credential_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_location` after provisioning.\nOutput only. The default storage location for the catalog, e.g., 'gs://my-bucket'."]
    pub fn default_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the IcebergCatalog.\nFor CATALOG_TYPE_GCS_BUCKET typed catalogs, the name needs to be the\nexact same value of the GCS bucket's name. For example, for a bucket:\ngs://bucket-name, the catalog name will be exactly \"bucket-name\"."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_location` after provisioning.\nThe primary location for mirroring the remote catalog metadata. It must be\na BigLake-supported location, and it should be proximate to the remote\ncatalog's location."]
    pub fn primary_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\nOutput only. The replicas for the catalog metadata."]
    pub fn replicas(&self) -> ListRef<BiglakeIcebergCatalogReplicasElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replicas", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_regions` after provisioning.\nOutput only. The GCP region(s) where the physical metadata for the tables is stored, e.g. 'us-central1', 'nam4' or 'us'. This will contain one value for all locations, except for the catalogs that are configured to use custom dual region buckets."]
    pub fn storage_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last modification time of the IcebergCatalog."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeIcebergCatalogTimeoutsElRef {
        BiglakeIcebergCatalogTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BiglakeIcebergCatalogReplicasEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl BiglakeIcebergCatalogReplicasEl {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for BiglakeIcebergCatalogReplicasEl {
    type O = BlockAssignable<BiglakeIcebergCatalogReplicasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBiglakeIcebergCatalogReplicasEl {}
impl BuildBiglakeIcebergCatalogReplicasEl {
    pub fn build(self) -> BiglakeIcebergCatalogReplicasEl {
        BiglakeIcebergCatalogReplicasEl {
            region: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct BiglakeIcebergCatalogReplicasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeIcebergCatalogReplicasElRef {
    fn new(shared: StackShared, base: String) -> BiglakeIcebergCatalogReplicasElRef {
        BiglakeIcebergCatalogReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BiglakeIcebergCatalogReplicasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct BiglakeIcebergCatalogTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BiglakeIcebergCatalogTimeoutsEl {
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
impl ToListMappable for BiglakeIcebergCatalogTimeoutsEl {
    type O = BlockAssignable<BiglakeIcebergCatalogTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBiglakeIcebergCatalogTimeoutsEl {}
impl BuildBiglakeIcebergCatalogTimeoutsEl {
    pub fn build(self) -> BiglakeIcebergCatalogTimeoutsEl {
        BiglakeIcebergCatalogTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BiglakeIcebergCatalogTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeIcebergCatalogTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BiglakeIcebergCatalogTimeoutsElRef {
        BiglakeIcebergCatalogTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BiglakeIcebergCatalogTimeoutsElRef {
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
