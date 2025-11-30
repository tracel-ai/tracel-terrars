use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataOdbDbServersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cloud_exadata_infrastructure_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataOdbDbServers_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbDbServersData>,
}

#[derive(Clone)]
pub struct DataOdbDbServers(Rc<DataOdbDbServers_>);

impl DataOdbDbServers {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe cloud exadata infrastructure ID. Mandatory field."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_servers` after provisioning.\nList of database servers associated with cloud_exadata_infrastructure_id."]
    pub fn db_servers(&self) -> ListRef<DataOdbDbServersDbServersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.db_servers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}

impl Referable for DataOdbDbServers {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataOdbDbServers {}

impl ToListMappable for DataOdbDbServers {
    type O = ListRef<DataOdbDbServersRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOdbDbServers_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_db_servers".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbDbServers {
    pub tf_id: String,
    #[doc = "The cloud exadata infrastructure ID. Mandatory field."]
    pub cloud_exadata_infrastructure_id: PrimField<String>,
}

impl BuildDataOdbDbServers {
    pub fn build(self, stack: &mut Stack) -> DataOdbDbServers {
        let out = DataOdbDbServers(Rc::new(DataOdbDbServers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbDbServersData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cloud_exadata_infrastructure_id: self.cloud_exadata_infrastructure_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbDbServersRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbDbServersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataOdbDbServersRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe cloud exadata infrastructure ID. Mandatory field."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `db_servers` after provisioning.\nList of database servers associated with cloud_exadata_infrastructure_id."]
    pub fn db_servers(&self) -> ListRef<DataOdbDbServersDbServersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.db_servers", self.extract_ref()),
        )
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
pub struct DataOdbDbServersDbServersElDbServerPatchingDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_patch_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patching_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_patching_ended: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_patching_started: Option<PrimField<String>>,
}

impl DataOdbDbServersDbServersElDbServerPatchingDetailsEl {
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

impl ToListMappable for DataOdbDbServersDbServersElDbServerPatchingDetailsEl {
    type O = BlockAssignable<DataOdbDbServersDbServersElDbServerPatchingDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbDbServersDbServersElDbServerPatchingDetailsEl {}

impl BuildDataOdbDbServersDbServersElDbServerPatchingDetailsEl {
    pub fn build(self) -> DataOdbDbServersDbServersElDbServerPatchingDetailsEl {
        DataOdbDbServersDbServersElDbServerPatchingDetailsEl {
            estimated_patch_duration: core::default::Default::default(),
            patching_status: core::default::Default::default(),
            time_patching_ended: core::default::Default::default(),
            time_patching_started: core::default::Default::default(),
        }
    }
}

pub struct DataOdbDbServersDbServersElDbServerPatchingDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbDbServersDbServersElDbServerPatchingDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOdbDbServersDbServersElDbServerPatchingDetailsElRef {
        DataOdbDbServersDbServersElDbServerPatchingDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbDbServersDbServersElDbServerPatchingDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `estimated_patch_duration` after provisioning.\n"]
    pub fn estimated_patch_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.estimated_patch_duration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `patching_status` after provisioning.\n"]
    pub fn patching_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.patching_status", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `time_patching_ended` after provisioning.\n"]
    pub fn time_patching_ended(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_patching_ended", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `time_patching_started` after provisioning.\n"]
    pub fn time_patching_started(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_patching_started", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataOdbDbServersDbServersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autonomous_virtual_machine_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autonomous_vm_cluster_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_core_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_node_storage_size_in_gbs: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_server_patching_details:
        Option<ListField<DataOdbDbServersDbServersElDbServerPatchingDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exadata_infrastructure_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_cpu_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_db_node_storage_in_gbs: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_memory_in_gbs: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size_in_gbs: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci_resource_anchor_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shape: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_cluster_ids: Option<ListField<PrimField<String>>>,
}

impl DataOdbDbServersDbServersEl {
    #[doc = "Set the field `autonomous_virtual_machine_ids`.\n"]
    pub fn set_autonomous_virtual_machine_ids(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.autonomous_virtual_machine_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `autonomous_vm_cluster_ids`.\n"]
    pub fn set_autonomous_vm_cluster_ids(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.autonomous_vm_cluster_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `compute_model`.\n"]
    pub fn set_compute_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compute_model = Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_core_count`.\n"]
    pub fn set_cpu_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_core_count = Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc = "Set the field `db_node_storage_size_in_gbs`.\n"]
    pub fn set_db_node_storage_size_in_gbs(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.db_node_storage_size_in_gbs = Some(v.into());
        self
    }

    #[doc = "Set the field `db_server_patching_details`.\n"]
    pub fn set_db_server_patching_details(
        mut self,
        v: impl Into<ListField<DataOdbDbServersDbServersElDbServerPatchingDetailsEl>>,
    ) -> Self {
        self.db_server_patching_details = Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }

    #[doc = "Set the field `exadata_infrastructure_id`.\n"]
    pub fn set_exadata_infrastructure_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exadata_infrastructure_id = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `max_cpu_count`.\n"]
    pub fn set_max_cpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_cpu_count = Some(v.into());
        self
    }

    #[doc = "Set the field `max_db_node_storage_in_gbs`.\n"]
    pub fn set_max_db_node_storage_in_gbs(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_db_node_storage_in_gbs = Some(v.into());
        self
    }

    #[doc = "Set the field `max_memory_in_gbs`.\n"]
    pub fn set_max_memory_in_gbs(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_memory_in_gbs = Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size_in_gbs`.\n"]
    pub fn set_memory_size_in_gbs(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_size_in_gbs = Some(v.into());
        self
    }

    #[doc = "Set the field `oci_resource_anchor_name`.\n"]
    pub fn set_oci_resource_anchor_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oci_resource_anchor_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ocid`.\n"]
    pub fn set_ocid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ocid = Some(v.into());
        self
    }

    #[doc = "Set the field `shape`.\n"]
    pub fn set_shape(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shape = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `status_reason`.\n"]
    pub fn set_status_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_reason = Some(v.into());
        self
    }

    #[doc = "Set the field `vm_cluster_ids`.\n"]
    pub fn set_vm_cluster_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.vm_cluster_ids = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbDbServersDbServersEl {
    type O = BlockAssignable<DataOdbDbServersDbServersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbDbServersDbServersEl {}

impl BuildDataOdbDbServersDbServersEl {
    pub fn build(self) -> DataOdbDbServersDbServersEl {
        DataOdbDbServersDbServersEl {
            autonomous_virtual_machine_ids: core::default::Default::default(),
            autonomous_vm_cluster_ids: core::default::Default::default(),
            compute_model: core::default::Default::default(),
            cpu_core_count: core::default::Default::default(),
            created_at: core::default::Default::default(),
            db_node_storage_size_in_gbs: core::default::Default::default(),
            db_server_patching_details: core::default::Default::default(),
            display_name: core::default::Default::default(),
            exadata_infrastructure_id: core::default::Default::default(),
            id: core::default::Default::default(),
            max_cpu_count: core::default::Default::default(),
            max_db_node_storage_in_gbs: core::default::Default::default(),
            max_memory_in_gbs: core::default::Default::default(),
            memory_size_in_gbs: core::default::Default::default(),
            oci_resource_anchor_name: core::default::Default::default(),
            ocid: core::default::Default::default(),
            shape: core::default::Default::default(),
            status: core::default::Default::default(),
            status_reason: core::default::Default::default(),
            vm_cluster_ids: core::default::Default::default(),
        }
    }
}

pub struct DataOdbDbServersDbServersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbDbServersDbServersElRef {
    fn new(shared: StackShared, base: String) -> DataOdbDbServersDbServersElRef {
        DataOdbDbServersDbServersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbDbServersDbServersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autonomous_virtual_machine_ids` after provisioning.\n"]
    pub fn autonomous_virtual_machine_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autonomous_virtual_machine_ids", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `autonomous_vm_cluster_ids` after provisioning.\n"]
    pub fn autonomous_vm_cluster_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autonomous_vm_cluster_ids", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `compute_model` after provisioning.\n"]
    pub fn compute_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_model", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\n"]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_core_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\n"]
    pub fn db_node_storage_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_node_storage_size_in_gbs", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `db_server_patching_details` after provisioning.\n"]
    pub fn db_server_patching_details(
        &self,
    ) -> ListRef<DataOdbDbServersDbServersElDbServerPatchingDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.db_server_patching_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `exadata_infrastructure_id` after provisioning.\n"]
    pub fn exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exadata_infrastructure_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `max_cpu_count` after provisioning.\n"]
    pub fn max_cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_cpu_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_db_node_storage_in_gbs` after provisioning.\n"]
    pub fn max_db_node_storage_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_db_node_storage_in_gbs", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_memory_in_gbs` after provisioning.\n"]
    pub fn max_memory_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_memory_in_gbs", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\n"]
    pub fn memory_size_in_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_in_gbs", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\n"]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_resource_anchor_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc = "Get a reference to the value of field `shape` after provisioning.\n"]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vm_cluster_ids` after provisioning.\n"]
    pub fn vm_cluster_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vm_cluster_ids", self.base),
        )
    }
}
