use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbDbServerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cloud_exadata_infrastructure_id: PrimField<String>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataOdbDbServer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbDbServerData>,
}

#[derive(Clone)]
pub struct DataOdbDbServer(Rc<DataOdbDbServer_>);

impl DataOdbDbServer {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `autonomous_virtual_machine_ids` after provisioning.\nThe list of unique identifiers for the Autonomous VMs associated with this database server."]
    pub fn autonomous_virtual_machine_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.autonomous_virtual_machine_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `autonomous_vm_cluster_ids` after provisioning.\nThe OCID of the autonomous VM clusters that are associated with the database server."]
    pub fn autonomous_vm_cluster_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.autonomous_vm_cluster_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe identifier of the database server to retrieve information about."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\n The compute model of the database server."]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores enabled on the database server."]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the database server was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe allocated local node storage in GBs on the database server."]
    pub fn db_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_server_patching_details` after provisioning.\nThe scheduling details for the quarterly maintenance window. Patching and\nsystem updates take place during the maintenance window."]
    pub fn db_server_patching_details(&self) -> ListRef<DataOdbDbServerDbServerPatchingDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.db_server_patching_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the database server."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_infrastructure_id` after provisioning.\nThe exadata infrastructure ID of the database server."]
    pub fn exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exadata_infrastructure_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe identifier of the the database server."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_cpu_count` after provisioning.\nThe total number of CPU cores available."]
    pub fn max_cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_cpu_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_db_node_storage_in_gbs` after provisioning.\nThe total local node storage available in GBs."]
    pub fn max_db_node_storage_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_db_node_storage_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_memory_in_gbs` after provisioning.\nThe total memory available in GBs."]
    pub fn max_memory_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_memory_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe allocated memory in GBs on the database server."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the database server to retrieve information about."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe shape of the database server. The shape determines the amount of CPU, storage, and memory resources available."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the database server."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the database server."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vm_cluster_ids` after provisioning.\nThe OCID of the VM clusters that are associated with the database server."]
    pub fn vm_cluster_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.vm_cluster_ids", self.extract_ref()))
    }
}

impl Referable for DataOdbDbServer {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOdbDbServer { }

impl ToListMappable for DataOdbDbServer {
    type O = ListRef<DataOdbDbServerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOdbDbServer_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_db_server".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbDbServer {
    pub tf_id: String,
    #[doc = "The identifier of the database server to retrieve information about."]
    pub cloud_exadata_infrastructure_id: PrimField<String>,
    #[doc = "The identifier of the the database server."]
    pub id: PrimField<String>,
}

impl BuildDataOdbDbServer {
    pub fn build(self, stack: &mut Stack) -> DataOdbDbServer {
        let out = DataOdbDbServer(Rc::new(DataOdbDbServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbDbServerData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cloud_exadata_infrastructure_id: self.cloud_exadata_infrastructure_id,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbDbServerRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbDbServerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataOdbDbServerRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `autonomous_virtual_machine_ids` after provisioning.\nThe list of unique identifiers for the Autonomous VMs associated with this database server."]
    pub fn autonomous_virtual_machine_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.autonomous_virtual_machine_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `autonomous_vm_cluster_ids` after provisioning.\nThe OCID of the autonomous VM clusters that are associated with the database server."]
    pub fn autonomous_vm_cluster_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.autonomous_vm_cluster_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe identifier of the database server to retrieve information about."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\n The compute model of the database server."]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores enabled on the database server."]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the database server was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe allocated local node storage in GBs on the database server."]
    pub fn db_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_server_patching_details` after provisioning.\nThe scheduling details for the quarterly maintenance window. Patching and\nsystem updates take place during the maintenance window."]
    pub fn db_server_patching_details(&self) -> ListRef<DataOdbDbServerDbServerPatchingDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.db_server_patching_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the database server."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_infrastructure_id` after provisioning.\nThe exadata infrastructure ID of the database server."]
    pub fn exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exadata_infrastructure_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe identifier of the the database server."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_cpu_count` after provisioning.\nThe total number of CPU cores available."]
    pub fn max_cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_cpu_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_db_node_storage_in_gbs` after provisioning.\nThe total local node storage available in GBs."]
    pub fn max_db_node_storage_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_db_node_storage_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_memory_in_gbs` after provisioning.\nThe total memory available in GBs."]
    pub fn max_memory_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_memory_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe allocated memory in GBs on the database server."]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the database server to retrieve information about."]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe shape of the database server. The shape determines the amount of CPU, storage, and memory resources available."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the database server."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the database server."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vm_cluster_ids` after provisioning.\nThe OCID of the VM clusters that are associated with the database server."]
    pub fn vm_cluster_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.vm_cluster_ids", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbDbServerDbServerPatchingDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_patch_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patching_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_patching_ended: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_patching_started: Option<PrimField<String>>,
}

impl DataOdbDbServerDbServerPatchingDetailsEl {
    #[doc = "Set the field `estimated_patch_duration`.\n"]
    pub fn set_estimated_patch_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.estimated_patch_duration = Some(v.into());
        self
    }

    #[doc = "Set the field `patching_status`.\n"]
    pub fn set_patching_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.patching_status = Some(v.into());
        self
    }

    #[doc = "Set the field `time_patching_ended`.\n"]
    pub fn set_time_patching_ended(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_patching_ended = Some(v.into());
        self
    }

    #[doc = "Set the field `time_patching_started`.\n"]
    pub fn set_time_patching_started(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_patching_started = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbDbServerDbServerPatchingDetailsEl {
    type O = BlockAssignable<DataOdbDbServerDbServerPatchingDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbDbServerDbServerPatchingDetailsEl {}

impl BuildDataOdbDbServerDbServerPatchingDetailsEl {
    pub fn build(self) -> DataOdbDbServerDbServerPatchingDetailsEl {
        DataOdbDbServerDbServerPatchingDetailsEl {
            estimated_patch_duration: core::default::Default::default(),
            patching_status: core::default::Default::default(),
            time_patching_ended: core::default::Default::default(),
            time_patching_started: core::default::Default::default(),
        }
    }
}

pub struct DataOdbDbServerDbServerPatchingDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbDbServerDbServerPatchingDetailsElRef {
    fn new(shared: StackShared, base: String) -> DataOdbDbServerDbServerPatchingDetailsElRef {
        DataOdbDbServerDbServerPatchingDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbDbServerDbServerPatchingDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `estimated_patch_duration` after provisioning.\n"]
    pub fn estimated_patch_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.estimated_patch_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `patching_status` after provisioning.\n"]
    pub fn patching_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.patching_status", self.base))
    }

    #[doc = "Get a reference to the value of field `time_patching_ended` after provisioning.\n"]
    pub fn time_patching_ended(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_patching_ended", self.base))
    }

    #[doc = "Get a reference to the value of field `time_patching_started` after provisioning.\n"]
    pub fn time_patching_started(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_patching_started", self.base))
    }
}
