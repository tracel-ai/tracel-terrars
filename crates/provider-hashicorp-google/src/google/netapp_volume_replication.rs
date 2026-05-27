use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetappVolumeReplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_destination_volume: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_stopping: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_enabled: Option<PrimField<bool>>,
    replication_schedule: PrimField<String>,
    volume_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_mirror: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_volume_parameters:
        Option<Vec<NetappVolumeReplicationDestinationVolumeParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetappVolumeReplicationTimeoutsEl>,
    dynamic: NetappVolumeReplicationDynamic,
}
struct NetappVolumeReplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetappVolumeReplicationData>,
}
#[derive(Clone)]
pub struct NetappVolumeReplication(Rc<NetappVolumeReplication_>);
impl NetappVolumeReplication {
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
    #[doc = "Set the field `delete_destination_volume`.\nA destination volume is created as part of replication creation. The destination volume will not became\nunder Terraform management unless you import it manually. If you delete the replication, this volume\nwill remain.\nSetting this parameter to true will delete the *current* destination volume when destroying the\nreplication. If you reversed the replication direction, this will be your former source volume!\nFor production use, it is recommended to keep this parameter false to avoid accidental volume\ndeletion. Handle with care. Default is false."]
    pub fn set_delete_destination_volume(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_destination_volume = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `force_stopping`.\nOnly replications with mirror_state=MIRRORED can be stopped. A replication in mirror_state=TRANSFERRING\ncurrently receives an update and stopping the update might be undesirable. Set this parameter to true\nto stop anyway. All data transferred to the destination will be discarded and content of destination\nvolume will remain at the state of the last successful update. Default is false."]
    pub fn set_force_stopping(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_stopping = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `replication_enabled`.\nSet to false to stop/break the mirror. Stopping the mirror makes the destination volume read-write\nand act independently from the source volume.\nSet to true to enable/resume the mirror. WARNING: Resuming a mirror overwrites any changes\ndone to the destination volume with the content of the source volume."]
    pub fn set_replication_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().replication_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `wait_for_mirror`.\nReplication resource state is independent of mirror_state. With enough data, it can take many hours\nfor mirror_state to reach MIRRORED. If you want Terraform to wait for the mirror to finish on\ncreate/stop/resume operations, set this parameter to true. Default is false."]
    pub fn set_wait_for_mirror(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_mirror = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_volume_parameters`.\n"]
    pub fn set_destination_volume_parameters(
        self,
        v: impl Into<BlockAssignable<NetappVolumeReplicationDestinationVolumeParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_volume_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .destination_volume_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetappVolumeReplicationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the active directory. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_destination_volume` after provisioning.\nA destination volume is created as part of replication creation. The destination volume will not became\nunder Terraform management unless you import it manually. If you delete the replication, this volume\nwill remain.\nSetting this parameter to true will delete the *current* destination volume when destroying the\nreplication. If you reversed the replication direction, this will be your former source volume!\nFor production use, it is recommended to keep this parameter false to avoid accidental volume\ndeletion. Handle with care. Default is false."]
    pub fn delete_destination_volume(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_destination_volume", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_volume` after provisioning.\nFull resource name of destination volume with format: 'projects/{{project}}/locations/{{location}}/volumes/{{volumeId}}'"]
    pub fn destination_volume(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_volume", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_stopping` after provisioning.\nOnly replications with mirror_state=MIRRORED can be stopped. A replication in mirror_state=TRANSFERRING\ncurrently receives an update and stopping the update might be undesirable. Set this parameter to true\nto stop anyway. All data transferred to the destination will be discarded and content of destination\nvolume will remain at the state of the last successful update. Default is false."]
    pub fn force_stopping(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_stopping", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `healthy` after provisioning.\nCondition of the relationship. Can be one of the following:\n  - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer.\n  - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer."]
    pub fn healthy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hybrid_peering_details` after provisioning.\nHybridPeeringDetails contains details about the hybrid peering."]
    pub fn hybrid_peering_details(
        &self,
    ) -> ListRef<NetappVolumeReplicationHybridPeeringDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hybrid_peering_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hybrid_replication_type` after provisioning.\nHybrid replication type."]
    pub fn hybrid_replication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hybrid_replication_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hybrid_replication_user_commands` after provisioning.\nCopy pastable snapmirror commands to be executed on onprem cluster by the customer."]
    pub fn hybrid_replication_user_commands(
        &self,
    ) -> ListRef<NetappVolumeReplicationHybridReplicationUserCommandsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hybrid_replication_user_commands", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nName of region for this resource. The resource needs to be created in the region of the destination volume."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mirror_state` after provisioning.\nIndicates the state of the mirror between source and destination volumes. Depending on the amount of data\nin your source volume, PREPARING phase can take hours or days. mirrorState = MIRRORED indicates your baseline\ntransfer ended and destination volume became accessible read-only. TRANSFERRING means a MIRRORED volume\ncurrently receives an update. Updated every 5 minutes."]
    pub fn mirror_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mirror_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the replication. Needs to be unique per location."]
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
    #[doc = "Get a reference to the value of field `replication_enabled` after provisioning.\nSet to false to stop/break the mirror. Stopping the mirror makes the destination volume read-write\nand act independently from the source volume.\nSet to true to enable/resume the mirror. WARNING: Resuming a mirror overwrites any changes\ndone to the destination volume with the content of the source volume."]
    pub fn replication_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_schedule` after provisioning.\nSpecifies the replication interval. Possible values: [\"EVERY_10_MINUTES\", \"HOURLY\", \"DAILY\"]"]
    pub fn replication_schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nReverting a replication can swap source and destination volume roles. This field indicates if the 'location' hosts\nthe source or destination volume. For resume and revert and resume operations it is critical to understand\nwhich volume is the source volume, since it will overwrite changes done to the destination volume."]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_volume` after provisioning.\nFull resource name of source volume with format: 'projects/{{project}}/locations/{{location}}/volumes/{{volumeId}}'"]
    pub fn source_volume(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_volume", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nIndicates the state of replication resource. State of the mirror itself is indicated in mirrorState."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nState details of the replication resource."]
    pub fn state_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_stats` after provisioning.\nReplication transfer statistics. All statistics are updated every 5 minutes."]
    pub fn transfer_stats(&self) -> ListRef<NetappVolumeReplicationTransferStatsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transfer_stats", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_name` after provisioning.\nThe name of the existing source volume."]
    pub fn volume_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_mirror` after provisioning.\nReplication resource state is independent of mirror_state. With enough data, it can take many hours\nfor mirror_state to reach MIRRORED. If you want Terraform to wait for the mirror to finish on\ncreate/stop/resume operations, set this parameter to true. Default is false."]
    pub fn wait_for_mirror(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_mirror", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_volume_parameters` after provisioning.\n"]
    pub fn destination_volume_parameters(
        &self,
    ) -> ListRef<NetappVolumeReplicationDestinationVolumeParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_volume_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappVolumeReplicationTimeoutsElRef {
        NetappVolumeReplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetappVolumeReplication {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetappVolumeReplication {}
impl ToListMappable for NetappVolumeReplication {
    type O = ListRef<NetappVolumeReplicationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetappVolumeReplication_ {
    fn extract_resource_type(&self) -> String {
        "google_netapp_volume_replication".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetappVolumeReplication {
    pub tf_id: String,
    #[doc = "Name of region for this resource. The resource needs to be created in the region of the destination volume."]
    pub location: PrimField<String>,
    #[doc = "The name of the replication. Needs to be unique per location."]
    pub name: PrimField<String>,
    #[doc = "Specifies the replication interval. Possible values: [\"EVERY_10_MINUTES\", \"HOURLY\", \"DAILY\"]"]
    pub replication_schedule: PrimField<String>,
    #[doc = "The name of the existing source volume."]
    pub volume_name: PrimField<String>,
}
impl BuildNetappVolumeReplication {
    pub fn build(self, stack: &mut Stack) -> NetappVolumeReplication {
        let out = NetappVolumeReplication(Rc::new(NetappVolumeReplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetappVolumeReplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                delete_destination_volume: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                force_stopping: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                replication_enabled: core::default::Default::default(),
                replication_schedule: self.replication_schedule,
                volume_name: self.volume_name,
                wait_for_mirror: core::default::Default::default(),
                destination_volume_parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetappVolumeReplicationRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeReplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetappVolumeReplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the active directory. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_destination_volume` after provisioning.\nA destination volume is created as part of replication creation. The destination volume will not became\nunder Terraform management unless you import it manually. If you delete the replication, this volume\nwill remain.\nSetting this parameter to true will delete the *current* destination volume when destroying the\nreplication. If you reversed the replication direction, this will be your former source volume!\nFor production use, it is recommended to keep this parameter false to avoid accidental volume\ndeletion. Handle with care. Default is false."]
    pub fn delete_destination_volume(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_destination_volume", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_volume` after provisioning.\nFull resource name of destination volume with format: 'projects/{{project}}/locations/{{location}}/volumes/{{volumeId}}'"]
    pub fn destination_volume(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_volume", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_stopping` after provisioning.\nOnly replications with mirror_state=MIRRORED can be stopped. A replication in mirror_state=TRANSFERRING\ncurrently receives an update and stopping the update might be undesirable. Set this parameter to true\nto stop anyway. All data transferred to the destination will be discarded and content of destination\nvolume will remain at the state of the last successful update. Default is false."]
    pub fn force_stopping(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_stopping", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `healthy` after provisioning.\nCondition of the relationship. Can be one of the following:\n  - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer.\n  - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer."]
    pub fn healthy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hybrid_peering_details` after provisioning.\nHybridPeeringDetails contains details about the hybrid peering."]
    pub fn hybrid_peering_details(
        &self,
    ) -> ListRef<NetappVolumeReplicationHybridPeeringDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hybrid_peering_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hybrid_replication_type` after provisioning.\nHybrid replication type."]
    pub fn hybrid_replication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hybrid_replication_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hybrid_replication_user_commands` after provisioning.\nCopy pastable snapmirror commands to be executed on onprem cluster by the customer."]
    pub fn hybrid_replication_user_commands(
        &self,
    ) -> ListRef<NetappVolumeReplicationHybridReplicationUserCommandsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hybrid_replication_user_commands", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nName of region for this resource. The resource needs to be created in the region of the destination volume."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mirror_state` after provisioning.\nIndicates the state of the mirror between source and destination volumes. Depending on the amount of data\nin your source volume, PREPARING phase can take hours or days. mirrorState = MIRRORED indicates your baseline\ntransfer ended and destination volume became accessible read-only. TRANSFERRING means a MIRRORED volume\ncurrently receives an update. Updated every 5 minutes."]
    pub fn mirror_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mirror_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the replication. Needs to be unique per location."]
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
    #[doc = "Get a reference to the value of field `replication_enabled` after provisioning.\nSet to false to stop/break the mirror. Stopping the mirror makes the destination volume read-write\nand act independently from the source volume.\nSet to true to enable/resume the mirror. WARNING: Resuming a mirror overwrites any changes\ndone to the destination volume with the content of the source volume."]
    pub fn replication_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication_schedule` after provisioning.\nSpecifies the replication interval. Possible values: [\"EVERY_10_MINUTES\", \"HOURLY\", \"DAILY\"]"]
    pub fn replication_schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nReverting a replication can swap source and destination volume roles. This field indicates if the 'location' hosts\nthe source or destination volume. For resume and revert and resume operations it is critical to understand\nwhich volume is the source volume, since it will overwrite changes done to the destination volume."]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_volume` after provisioning.\nFull resource name of source volume with format: 'projects/{{project}}/locations/{{location}}/volumes/{{volumeId}}'"]
    pub fn source_volume(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_volume", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nIndicates the state of replication resource. State of the mirror itself is indicated in mirrorState."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nState details of the replication resource."]
    pub fn state_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_stats` after provisioning.\nReplication transfer statistics. All statistics are updated every 5 minutes."]
    pub fn transfer_stats(&self) -> ListRef<NetappVolumeReplicationTransferStatsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transfer_stats", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_name` after provisioning.\nThe name of the existing source volume."]
    pub fn volume_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wait_for_mirror` after provisioning.\nReplication resource state is independent of mirror_state. With enough data, it can take many hours\nfor mirror_state to reach MIRRORED. If you want Terraform to wait for the mirror to finish on\ncreate/stop/resume operations, set this parameter to true. Default is false."]
    pub fn wait_for_mirror(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_mirror", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_volume_parameters` after provisioning.\n"]
    pub fn destination_volume_parameters(
        &self,
    ) -> ListRef<NetappVolumeReplicationDestinationVolumeParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_volume_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappVolumeReplicationTimeoutsElRef {
        NetappVolumeReplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetappVolumeReplicationHybridPeeringDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command_expiry_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    passphrase: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_cluster_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_svm_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_volume_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ip: Option<PrimField<String>>,
}
impl NetappVolumeReplicationHybridPeeringDetailsEl {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `command_expiry_time`.\n"]
    pub fn set_command_expiry_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.command_expiry_time = Some(v.into());
        self
    }
    #[doc = "Set the field `passphrase`.\n"]
    pub fn set_passphrase(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.passphrase = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_cluster_name`.\n"]
    pub fn set_peer_cluster_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_cluster_name = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_svm_name`.\n"]
    pub fn set_peer_svm_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_svm_name = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_volume_name`.\n"]
    pub fn set_peer_volume_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_volume_name = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_ip`.\n"]
    pub fn set_subnet_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_ip = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeReplicationHybridPeeringDetailsEl {
    type O = BlockAssignable<NetappVolumeReplicationHybridPeeringDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeReplicationHybridPeeringDetailsEl {}
impl BuildNetappVolumeReplicationHybridPeeringDetailsEl {
    pub fn build(self) -> NetappVolumeReplicationHybridPeeringDetailsEl {
        NetappVolumeReplicationHybridPeeringDetailsEl {
            command: core::default::Default::default(),
            command_expiry_time: core::default::Default::default(),
            passphrase: core::default::Default::default(),
            peer_cluster_name: core::default::Default::default(),
            peer_svm_name: core::default::Default::default(),
            peer_volume_name: core::default::Default::default(),
            subnet_ip: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeReplicationHybridPeeringDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeReplicationHybridPeeringDetailsElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeReplicationHybridPeeringDetailsElRef {
        NetappVolumeReplicationHybridPeeringDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeReplicationHybridPeeringDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `command_expiry_time` after provisioning.\n"]
    pub fn command_expiry_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.command_expiry_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `passphrase` after provisioning.\n"]
    pub fn passphrase(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }
    #[doc = "Get a reference to the value of field `peer_cluster_name` after provisioning.\n"]
    pub fn peer_cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_cluster_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peer_svm_name` after provisioning.\n"]
    pub fn peer_svm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_svm_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peer_volume_name` after provisioning.\n"]
    pub fn peer_volume_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_volume_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ip` after provisioning.\n"]
    pub fn subnet_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_ip", self.base))
    }
}
#[derive(Serialize)]
pub struct NetappVolumeReplicationHybridReplicationUserCommandsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    commands: Option<ListField<PrimField<String>>>,
}
impl NetappVolumeReplicationHybridReplicationUserCommandsEl {
    #[doc = "Set the field `commands`.\n"]
    pub fn set_commands(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.commands = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeReplicationHybridReplicationUserCommandsEl {
    type O = BlockAssignable<NetappVolumeReplicationHybridReplicationUserCommandsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeReplicationHybridReplicationUserCommandsEl {}
impl BuildNetappVolumeReplicationHybridReplicationUserCommandsEl {
    pub fn build(self) -> NetappVolumeReplicationHybridReplicationUserCommandsEl {
        NetappVolumeReplicationHybridReplicationUserCommandsEl {
            commands: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeReplicationHybridReplicationUserCommandsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeReplicationHybridReplicationUserCommandsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetappVolumeReplicationHybridReplicationUserCommandsElRef {
        NetappVolumeReplicationHybridReplicationUserCommandsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeReplicationHybridReplicationUserCommandsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `commands` after provisioning.\n"]
    pub fn commands(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.commands", self.base))
    }
}
#[derive(Serialize)]
pub struct NetappVolumeReplicationTransferStatsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lag_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_transfer_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_transfer_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_transfer_end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_transfer_error: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_transfer_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl NetappVolumeReplicationTransferStatsEl {
    #[doc = "Set the field `lag_duration`.\n"]
    pub fn set_lag_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lag_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `last_transfer_bytes`.\n"]
    pub fn set_last_transfer_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_transfer_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `last_transfer_duration`.\n"]
    pub fn set_last_transfer_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_transfer_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `last_transfer_end_time`.\n"]
    pub fn set_last_transfer_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_transfer_end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `last_transfer_error`.\n"]
    pub fn set_last_transfer_error(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_transfer_error = Some(v.into());
        self
    }
    #[doc = "Set the field `total_transfer_duration`.\n"]
    pub fn set_total_transfer_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_transfer_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `transfer_bytes`.\n"]
    pub fn set_transfer_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transfer_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeReplicationTransferStatsEl {
    type O = BlockAssignable<NetappVolumeReplicationTransferStatsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeReplicationTransferStatsEl {}
impl BuildNetappVolumeReplicationTransferStatsEl {
    pub fn build(self) -> NetappVolumeReplicationTransferStatsEl {
        NetappVolumeReplicationTransferStatsEl {
            lag_duration: core::default::Default::default(),
            last_transfer_bytes: core::default::Default::default(),
            last_transfer_duration: core::default::Default::default(),
            last_transfer_end_time: core::default::Default::default(),
            last_transfer_error: core::default::Default::default(),
            total_transfer_duration: core::default::Default::default(),
            transfer_bytes: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeReplicationTransferStatsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeReplicationTransferStatsElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeReplicationTransferStatsElRef {
        NetappVolumeReplicationTransferStatsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeReplicationTransferStatsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lag_duration` after provisioning.\n"]
    pub fn lag_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lag_duration", self.base))
    }
    #[doc = "Get a reference to the value of field `last_transfer_bytes` after provisioning.\n"]
    pub fn last_transfer_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_transfer_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_transfer_duration` after provisioning.\n"]
    pub fn last_transfer_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_transfer_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_transfer_end_time` after provisioning.\n"]
    pub fn last_transfer_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_transfer_end_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_transfer_error` after provisioning.\n"]
    pub fn last_transfer_error(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_transfer_error", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_transfer_duration` after provisioning.\n"]
    pub fn total_transfer_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_transfer_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_bytes` after provisioning.\n"]
    pub fn transfer_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transfer_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cooling_threshold_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier_action: Option<PrimField<String>>,
}
impl NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyEl {
    #[doc = "Set the field `cooling_threshold_days`.\nOptional. Time in days to mark the volume's data block as cold and make it eligible for tiering, can be range from 2-183.\nDefault is 31."]
    pub fn set_cooling_threshold_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cooling_threshold_days = Some(v.into());
        self
    }
    #[doc = "Set the field `tier_action`.\nOptional. Flag indicating if the volume has tiering policy enable/pause. Default is PAUSED. Default value: \"PAUSED\" Possible values: [\"ENABLED\", \"PAUSED\"]"]
    pub fn set_tier_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tier_action = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyEl {
    type O = BlockAssignable<NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeReplicationDestinationVolumeParametersElTieringPolicyEl {}
impl BuildNetappVolumeReplicationDestinationVolumeParametersElTieringPolicyEl {
    pub fn build(self) -> NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyEl {
        NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyEl {
            cooling_threshold_days: core::default::Default::default(),
            tier_action: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyElRef {
        NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cooling_threshold_days` after provisioning.\nOptional. Time in days to mark the volume's data block as cold and make it eligible for tiering, can be range from 2-183.\nDefault is 31."]
    pub fn cooling_threshold_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cooling_threshold_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tier_action` after provisioning.\nOptional. Flag indicating if the volume has tiering policy enable/pause. Default is PAUSED. Default value: \"PAUSED\" Possible values: [\"ENABLED\", \"PAUSED\"]"]
    pub fn tier_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier_action", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetappVolumeReplicationDestinationVolumeParametersElDynamic {
    tiering_policy:
        Option<DynamicBlock<NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyEl>>,
}
#[derive(Serialize)]
pub struct NetappVolumeReplicationDestinationVolumeParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_name: Option<PrimField<String>>,
    storage_pool: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiering_policy:
        Option<Vec<NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyEl>>,
    dynamic: NetappVolumeReplicationDestinationVolumeParametersElDynamic,
}
impl NetappVolumeReplicationDestinationVolumeParametersEl {
    #[doc = "Set the field `description`.\nDescription for the destination volume."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `share_name`.\nShare name for destination volume. If not specified, name of source volume's share name will be used."]
    pub fn set_share_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.share_name = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_id`.\nName for the destination volume to be created. If not specified, the name of the source volume will be used."]
    pub fn set_volume_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_id = Some(v.into());
        self
    }
    #[doc = "Set the field `tiering_policy`.\n"]
    pub fn set_tiering_policy(
        mut self,
        v: impl Into<
            BlockAssignable<NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tiering_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tiering_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetappVolumeReplicationDestinationVolumeParametersEl {
    type O = BlockAssignable<NetappVolumeReplicationDestinationVolumeParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeReplicationDestinationVolumeParametersEl {
    #[doc = "Name of an existing storage pool for the destination volume with format: 'projects/{{project}}/locations/{{location}}/storagePools/{{poolId}}'"]
    pub storage_pool: PrimField<String>,
}
impl BuildNetappVolumeReplicationDestinationVolumeParametersEl {
    pub fn build(self) -> NetappVolumeReplicationDestinationVolumeParametersEl {
        NetappVolumeReplicationDestinationVolumeParametersEl {
            description: core::default::Default::default(),
            share_name: core::default::Default::default(),
            storage_pool: self.storage_pool,
            volume_id: core::default::Default::default(),
            tiering_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetappVolumeReplicationDestinationVolumeParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeReplicationDestinationVolumeParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetappVolumeReplicationDestinationVolumeParametersElRef {
        NetappVolumeReplicationDestinationVolumeParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeReplicationDestinationVolumeParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription for the destination volume."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `share_name` after provisioning.\nShare name for destination volume. If not specified, name of source volume's share name will be used."]
    pub fn share_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_name", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_pool` after provisioning.\nName of an existing storage pool for the destination volume with format: 'projects/{{project}}/locations/{{location}}/storagePools/{{poolId}}'"]
    pub fn storage_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_pool", self.base))
    }
    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\nName for the destination volume to be created. If not specified, the name of the source volume will be used."]
    pub fn volume_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.base))
    }
    #[doc = "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(
        &self,
    ) -> ListRef<NetappVolumeReplicationDestinationVolumeParametersElTieringPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tiering_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetappVolumeReplicationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetappVolumeReplicationTimeoutsEl {
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
impl ToListMappable for NetappVolumeReplicationTimeoutsEl {
    type O = BlockAssignable<NetappVolumeReplicationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeReplicationTimeoutsEl {}
impl BuildNetappVolumeReplicationTimeoutsEl {
    pub fn build(self) -> NetappVolumeReplicationTimeoutsEl {
        NetappVolumeReplicationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeReplicationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeReplicationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeReplicationTimeoutsElRef {
        NetappVolumeReplicationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeReplicationTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct NetappVolumeReplicationDynamic {
    destination_volume_parameters:
        Option<DynamicBlock<NetappVolumeReplicationDestinationVolumeParametersEl>>,
}
