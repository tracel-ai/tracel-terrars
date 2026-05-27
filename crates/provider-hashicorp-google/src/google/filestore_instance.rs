use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FilestoreInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_replica_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    tier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_services: Option<Vec<FilestoreInstanceDirectoryServicesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_shares: Option<Vec<FilestoreInstanceFileSharesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_replication: Option<Vec<FilestoreInstanceInitialReplicationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks: Option<Vec<FilestoreInstanceNetworksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_config: Option<Vec<FilestoreInstancePerformanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FilestoreInstanceTimeoutsEl>,
    dynamic: FilestoreInstanceDynamic,
}
struct FilestoreInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FilestoreInstanceData>,
}
#[derive(Clone)]
pub struct FilestoreInstance(Rc<FilestoreInstance_>);
impl FilestoreInstance {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection_enabled`.\nIndicates whether the instance is protected against deletion."]
    pub fn set_deletion_protection_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection_reason`.\nThe reason for enabling deletion protection."]
    pub fn set_deletion_protection_reason(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_protection_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA description of the instance."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `desired_replica_state`.\nThe desired_replica_state field controls the state of a replica. Terraform will attempt to make the actual state of the replica match the desired state. Default value: \"READY\" Possible values: [\"PAUSED\", \"READY\"]"]
    pub fn set_desired_replica_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desired_replica_state = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\nKMS key name used for data encryption."]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe name of the location of the instance. This can be a region for ENTERPRISE tier instances."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\nEither NFSv3, for using NFS version 3 as file sharing protocol,\nor NFSv4.1, for using NFS version 4.1 as file sharing protocol.\nNFSv4.1 can be used with HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE.\nThe default is NFSv3. Default value: \"NFS_V3\" Possible values: [\"NFS_V3\", \"NFS_V4_1\"]"]
    pub fn set_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nA map of resource manager tags. Resource manager tag keys\nand values have the same definition as resource manager\ntags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is\nignored when empty. The field is immutable and causes\nresource replacement when mutated. This field is only set\nat create time and modifying this field after creation\nwill trigger recreation. To apply tags to an existing\nresource, see the 'google_tags_tag_value' resource."]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe name of the Filestore zone of the instance."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Set the field `directory_services`.\n"]
    pub fn set_directory_services(
        self,
        v: impl Into<BlockAssignable<FilestoreInstanceDirectoryServicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().directory_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.directory_services = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `file_shares`.\n"]
    pub fn set_file_shares(
        self,
        v: impl Into<BlockAssignable<FilestoreInstanceFileSharesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().file_shares = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.file_shares = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `initial_replication`.\n"]
    pub fn set_initial_replication(
        self,
        v: impl Into<BlockAssignable<FilestoreInstanceInitialReplicationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_replication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_replication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `networks`.\n"]
    pub fn set_networks(self, v: impl Into<BlockAssignable<FilestoreInstanceNetworksEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `performance_config`.\n"]
    pub fn set_performance_config(
        self,
        v: impl Into<BlockAssignable<FilestoreInstancePerformanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().performance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.performance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FilestoreInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nIndicates whether the instance is protected against deletion."]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection_reason` after provisioning.\nThe reason for enabling deletion protection."]
    pub fn deletion_protection_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_replica_state` after provisioning.\nThe desired_replica_state field controls the state of a replica. Terraform will attempt to make the actual state of the replica match the desired state. Default value: \"READY\" Possible values: [\"PAUSED\", \"READY\"]"]
    pub fn desired_replica_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_replica_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_replication` after provisioning.\nOutput only fields for replication configuration."]
    pub fn effective_replication(&self) -> ListRef<FilestoreInstanceEffectiveReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_replication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer-specified ETag for the instance resource to prevent\nsimultaneous updates from overwriting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nKMS key name used for data encryption."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the instance. This can be a region for ENTERPRISE tier instances."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the instance."]
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
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nEither NFSv3, for using NFS version 3 as file sharing protocol,\nor NFSv4.1, for using NFS version 4.1 as file sharing protocol.\nNFSv4.1 can be used with HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE.\nThe default is NFSv3. Default value: \"NFS_V3\" Possible values: [\"NFS_V3\", \"NFS_V4_1\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags. Resource manager tag keys\nand values have the same definition as resource manager\ntags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is\nignored when empty. The field is immutable and causes\nresource replacement when mutated. This field is only set\nat create time and modifying this field after creation\nwill trigger recreation. To apply tags to an existing\nresource, see the 'google_tags_tag_value' resource."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe service tier of the instance.\nPossible values include: STANDARD, PREMIUM, BASIC_HDD, BASIC_SSD, HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe name of the Filestore zone of the instance."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_services` after provisioning.\n"]
    pub fn directory_services(&self) -> ListRef<FilestoreInstanceDirectoryServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.directory_services", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_shares` after provisioning.\n"]
    pub fn file_shares(&self) -> ListRef<FilestoreInstanceFileSharesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_shares", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `initial_replication` after provisioning.\n"]
    pub fn initial_replication(&self) -> ListRef<FilestoreInstanceInitialReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initial_replication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> ListRef<FilestoreInstanceNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_config` after provisioning.\n"]
    pub fn performance_config(&self) -> ListRef<FilestoreInstancePerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.performance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FilestoreInstanceTimeoutsElRef {
        FilestoreInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FilestoreInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FilestoreInstance {}
impl ToListMappable for FilestoreInstance {
    type O = ListRef<FilestoreInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FilestoreInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_filestore_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFilestoreInstance {
    pub tf_id: String,
    #[doc = "The resource name of the instance."]
    pub name: PrimField<String>,
    #[doc = "The service tier of the instance.\nPossible values include: STANDARD, PREMIUM, BASIC_HDD, BASIC_SSD, HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE"]
    pub tier: PrimField<String>,
}
impl BuildFilestoreInstance {
    pub fn build(self, stack: &mut Stack) -> FilestoreInstance {
        let out = FilestoreInstance(Rc::new(FilestoreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FilestoreInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                deletion_protection_enabled: core::default::Default::default(),
                deletion_protection_reason: core::default::Default::default(),
                description: core::default::Default::default(),
                desired_replica_state: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                protocol: core::default::Default::default(),
                tags: core::default::Default::default(),
                tier: self.tier,
                zone: core::default::Default::default(),
                directory_services: core::default::Default::default(),
                file_shares: core::default::Default::default(),
                initial_replication: core::default::Default::default(),
                networks: core::default::Default::default(),
                performance_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FilestoreInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FilestoreInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nIndicates whether the instance is protected against deletion."]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection_reason` after provisioning.\nThe reason for enabling deletion protection."]
    pub fn deletion_protection_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_replica_state` after provisioning.\nThe desired_replica_state field controls the state of a replica. Terraform will attempt to make the actual state of the replica match the desired state. Default value: \"READY\" Possible values: [\"PAUSED\", \"READY\"]"]
    pub fn desired_replica_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_replica_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_replication` after provisioning.\nOutput only fields for replication configuration."]
    pub fn effective_replication(&self) -> ListRef<FilestoreInstanceEffectiveReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_replication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer-specified ETag for the instance resource to prevent\nsimultaneous updates from overwriting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nKMS key name used for data encryption."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the instance. This can be a region for ENTERPRISE tier instances."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the instance."]
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
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nEither NFSv3, for using NFS version 3 as file sharing protocol,\nor NFSv4.1, for using NFS version 4.1 as file sharing protocol.\nNFSv4.1 can be used with HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE.\nThe default is NFSv3. Default value: \"NFS_V3\" Possible values: [\"NFS_V3\", \"NFS_V4_1\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags. Resource manager tag keys\nand values have the same definition as resource manager\ntags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is\nignored when empty. The field is immutable and causes\nresource replacement when mutated. This field is only set\nat create time and modifying this field after creation\nwill trigger recreation. To apply tags to an existing\nresource, see the 'google_tags_tag_value' resource."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe service tier of the instance.\nPossible values include: STANDARD, PREMIUM, BASIC_HDD, BASIC_SSD, HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe name of the Filestore zone of the instance."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_services` after provisioning.\n"]
    pub fn directory_services(&self) -> ListRef<FilestoreInstanceDirectoryServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.directory_services", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_shares` after provisioning.\n"]
    pub fn file_shares(&self) -> ListRef<FilestoreInstanceFileSharesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_shares", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `initial_replication` after provisioning.\n"]
    pub fn initial_replication(&self) -> ListRef<FilestoreInstanceInitialReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initial_replication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> ListRef<FilestoreInstanceNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_config` after provisioning.\n"]
    pub fn performance_config(&self) -> ListRef<FilestoreInstancePerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.performance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FilestoreInstanceTimeoutsElRef {
        FilestoreInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FilestoreInstanceEffectiveReplicationElReplicasEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_active_sync_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_reasons: Option<ListField<PrimField<String>>>,
}
impl FilestoreInstanceEffectiveReplicationElReplicasEl {
    #[doc = "Set the field `last_active_sync_time`.\n"]
    pub fn set_last_active_sync_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_active_sync_time = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_instance`.\n"]
    pub fn set_peer_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `state_reasons`.\n"]
    pub fn set_state_reasons(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.state_reasons = Some(v.into());
        self
    }
}
impl ToListMappable for FilestoreInstanceEffectiveReplicationElReplicasEl {
    type O = BlockAssignable<FilestoreInstanceEffectiveReplicationElReplicasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstanceEffectiveReplicationElReplicasEl {}
impl BuildFilestoreInstanceEffectiveReplicationElReplicasEl {
    pub fn build(self) -> FilestoreInstanceEffectiveReplicationElReplicasEl {
        FilestoreInstanceEffectiveReplicationElReplicasEl {
            last_active_sync_time: core::default::Default::default(),
            peer_instance: core::default::Default::default(),
            state: core::default::Default::default(),
            state_reasons: core::default::Default::default(),
        }
    }
}
pub struct FilestoreInstanceEffectiveReplicationElReplicasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceEffectiveReplicationElReplicasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FilestoreInstanceEffectiveReplicationElReplicasElRef {
        FilestoreInstanceEffectiveReplicationElReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstanceEffectiveReplicationElReplicasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_active_sync_time` after provisioning.\n"]
    pub fn last_active_sync_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_active_sync_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peer_instance` after provisioning.\n"]
    pub fn peer_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `state_reasons` after provisioning.\n"]
    pub fn state_reasons(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state_reasons", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FilestoreInstanceEffectiveReplicationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas: Option<ListField<FilestoreInstanceEffectiveReplicationElReplicasEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
}
impl FilestoreInstanceEffectiveReplicationEl {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl Into<ListField<FilestoreInstanceEffectiveReplicationElReplicasEl>>,
    ) -> Self {
        self.replicas = Some(v.into());
        self
    }
    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }
}
impl ToListMappable for FilestoreInstanceEffectiveReplicationEl {
    type O = BlockAssignable<FilestoreInstanceEffectiveReplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstanceEffectiveReplicationEl {}
impl BuildFilestoreInstanceEffectiveReplicationEl {
    pub fn build(self) -> FilestoreInstanceEffectiveReplicationEl {
        FilestoreInstanceEffectiveReplicationEl {
            replicas: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}
pub struct FilestoreInstanceEffectiveReplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceEffectiveReplicationElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceEffectiveReplicationElRef {
        FilestoreInstanceEffectiveReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstanceEffectiveReplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> ListRef<FilestoreInstanceEffectiveReplicationElReplicasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}
#[derive(Serialize)]
pub struct FilestoreInstanceDirectoryServicesElLdapEl {
    domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups_ou: Option<PrimField<String>>,
    servers: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    users_ou: Option<PrimField<String>>,
}
impl FilestoreInstanceDirectoryServicesElLdapEl {
    #[doc = "Set the field `groups_ou`.\nThe groups Organizational Unit (OU) is optional. This parameter is a hint\nto allow faster lookup in the LDAP namespace. In case that this parameter\nis not provided, Filestore instance will query the whole LDAP namespace."]
    pub fn set_groups_ou(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.groups_ou = Some(v.into());
        self
    }
    #[doc = "Set the field `users_ou`.\nThe users Organizational Unit (OU) is optional. This parameter is a hint\nto allow faster lookup in the LDAP namespace. In case that this parameter\nis not provided, Filestore instance will query the whole LDAP namespace."]
    pub fn set_users_ou(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.users_ou = Some(v.into());
        self
    }
}
impl ToListMappable for FilestoreInstanceDirectoryServicesElLdapEl {
    type O = BlockAssignable<FilestoreInstanceDirectoryServicesElLdapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstanceDirectoryServicesElLdapEl {
    #[doc = "The LDAP domain name in the format of 'my-domain.com'."]
    pub domain: PrimField<String>,
    #[doc = "The servers names are used for specifying the LDAP servers names.\nThe LDAP servers names can come with two formats:\n1. DNS name, for example: 'ldap.example1.com', 'ldap.example2.com'.\n2. IP address, for example: '10.0.0.1', '10.0.0.2', '10.0.0.3'.\nAll servers names must be in the same format: either all DNS names or all\nIP addresses."]
    pub servers: ListField<PrimField<String>>,
}
impl BuildFilestoreInstanceDirectoryServicesElLdapEl {
    pub fn build(self) -> FilestoreInstanceDirectoryServicesElLdapEl {
        FilestoreInstanceDirectoryServicesElLdapEl {
            domain: self.domain,
            groups_ou: core::default::Default::default(),
            servers: self.servers,
            users_ou: core::default::Default::default(),
        }
    }
}
pub struct FilestoreInstanceDirectoryServicesElLdapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceDirectoryServicesElLdapElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceDirectoryServicesElLdapElRef {
        FilestoreInstanceDirectoryServicesElLdapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstanceDirectoryServicesElLdapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe LDAP domain name in the format of 'my-domain.com'."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `groups_ou` after provisioning.\nThe groups Organizational Unit (OU) is optional. This parameter is a hint\nto allow faster lookup in the LDAP namespace. In case that this parameter\nis not provided, Filestore instance will query the whole LDAP namespace."]
    pub fn groups_ou(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.groups_ou", self.base))
    }
    #[doc = "Get a reference to the value of field `servers` after provisioning.\nThe servers names are used for specifying the LDAP servers names.\nThe LDAP servers names can come with two formats:\n1. DNS name, for example: 'ldap.example1.com', 'ldap.example2.com'.\n2. IP address, for example: '10.0.0.1', '10.0.0.2', '10.0.0.3'.\nAll servers names must be in the same format: either all DNS names or all\nIP addresses."]
    pub fn servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }
    #[doc = "Get a reference to the value of field `users_ou` after provisioning.\nThe users Organizational Unit (OU) is optional. This parameter is a hint\nto allow faster lookup in the LDAP namespace. In case that this parameter\nis not provided, Filestore instance will query the whole LDAP namespace."]
    pub fn users_ou(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.users_ou", self.base))
    }
}
#[derive(Serialize, Default)]
struct FilestoreInstanceDirectoryServicesElDynamic {
    ldap: Option<DynamicBlock<FilestoreInstanceDirectoryServicesElLdapEl>>,
}
#[derive(Serialize)]
pub struct FilestoreInstanceDirectoryServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ldap: Option<Vec<FilestoreInstanceDirectoryServicesElLdapEl>>,
    dynamic: FilestoreInstanceDirectoryServicesElDynamic,
}
impl FilestoreInstanceDirectoryServicesEl {
    #[doc = "Set the field `ldap`.\n"]
    pub fn set_ldap(
        mut self,
        v: impl Into<BlockAssignable<FilestoreInstanceDirectoryServicesElLdapEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ldap = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ldap = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FilestoreInstanceDirectoryServicesEl {
    type O = BlockAssignable<FilestoreInstanceDirectoryServicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstanceDirectoryServicesEl {}
impl BuildFilestoreInstanceDirectoryServicesEl {
    pub fn build(self) -> FilestoreInstanceDirectoryServicesEl {
        FilestoreInstanceDirectoryServicesEl {
            ldap: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FilestoreInstanceDirectoryServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceDirectoryServicesElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceDirectoryServicesElRef {
        FilestoreInstanceDirectoryServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstanceDirectoryServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ldap` after provisioning.\n"]
    pub fn ldap(&self) -> ListRef<FilestoreInstanceDirectoryServicesElLdapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ldap", self.base))
    }
}
#[derive(Serialize)]
pub struct FilestoreInstanceFileSharesElNfsExportOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anon_gid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anon_uid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squash_mode: Option<PrimField<String>>,
}
impl FilestoreInstanceFileSharesElNfsExportOptionsEl {
    #[doc = "Set the field `access_mode`.\nEither READ_ONLY, for allowing only read requests on the exported directory,\nor READ_WRITE, for allowing both read and write requests. The default is READ_WRITE. Default value: \"READ_WRITE\" Possible values: [\"READ_ONLY\", \"READ_WRITE\"]"]
    pub fn set_access_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `anon_gid`.\nAn integer representing the anonymous group id with a default value of 65534.\nAnon_gid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn set_anon_gid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.anon_gid = Some(v.into());
        self
    }
    #[doc = "Set the field `anon_uid`.\nAn integer representing the anonymous user id with a default value of 65534.\nAnon_uid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn set_anon_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.anon_uid = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_ranges`.\nList of either IPv4 addresses, or ranges in CIDR notation which may mount the file share.\nOverlapping IP ranges are not allowed, both within and across NfsExportOptions. An error will be returned.\nThe limit is 64 IP ranges/addresses for each FileShareConfig among all NfsExportOptions."]
    pub fn set_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe source VPC network for 'ip_ranges'.\nRequired for instances using Private Service Connect, optional otherwise."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `squash_mode`.\nEither NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH,\nfor not allowing root access. The default is NO_ROOT_SQUASH. Default value: \"NO_ROOT_SQUASH\" Possible values: [\"NO_ROOT_SQUASH\", \"ROOT_SQUASH\"]"]
    pub fn set_squash_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.squash_mode = Some(v.into());
        self
    }
}
impl ToListMappable for FilestoreInstanceFileSharesElNfsExportOptionsEl {
    type O = BlockAssignable<FilestoreInstanceFileSharesElNfsExportOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstanceFileSharesElNfsExportOptionsEl {}
impl BuildFilestoreInstanceFileSharesElNfsExportOptionsEl {
    pub fn build(self) -> FilestoreInstanceFileSharesElNfsExportOptionsEl {
        FilestoreInstanceFileSharesElNfsExportOptionsEl {
            access_mode: core::default::Default::default(),
            anon_gid: core::default::Default::default(),
            anon_uid: core::default::Default::default(),
            ip_ranges: core::default::Default::default(),
            network: core::default::Default::default(),
            squash_mode: core::default::Default::default(),
        }
    }
}
pub struct FilestoreInstanceFileSharesElNfsExportOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceFileSharesElNfsExportOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FilestoreInstanceFileSharesElNfsExportOptionsElRef {
        FilestoreInstanceFileSharesElNfsExportOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstanceFileSharesElNfsExportOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\nEither READ_ONLY, for allowing only read requests on the exported directory,\nor READ_WRITE, for allowing both read and write requests. The default is READ_WRITE. Default value: \"READ_WRITE\" Possible values: [\"READ_ONLY\", \"READ_WRITE\"]"]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `anon_gid` after provisioning.\nAn integer representing the anonymous group id with a default value of 65534.\nAnon_gid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn anon_gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.anon_gid", self.base))
    }
    #[doc = "Get a reference to the value of field `anon_uid` after provisioning.\nAn integer representing the anonymous user id with a default value of 65534.\nAnon_uid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn anon_uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.anon_uid", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_ranges` after provisioning.\nList of either IPv4 addresses, or ranges in CIDR notation which may mount the file share.\nOverlapping IP ranges are not allowed, both within and across NfsExportOptions. An error will be returned.\nThe limit is 64 IP ranges/addresses for each FileShareConfig among all NfsExportOptions."]
    pub fn ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_ranges", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe source VPC network for 'ip_ranges'.\nRequired for instances using Private Service Connect, optional otherwise."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `squash_mode` after provisioning.\nEither NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH,\nfor not allowing root access. The default is NO_ROOT_SQUASH. Default value: \"NO_ROOT_SQUASH\" Possible values: [\"NO_ROOT_SQUASH\", \"ROOT_SQUASH\"]"]
    pub fn squash_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_mode", self.base))
    }
}
#[derive(Serialize, Default)]
struct FilestoreInstanceFileSharesElDynamic {
    nfs_export_options: Option<DynamicBlock<FilestoreInstanceFileSharesElNfsExportOptionsEl>>,
}
#[derive(Serialize)]
pub struct FilestoreInstanceFileSharesEl {
    capacity_gb: PrimField<f64>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_backup: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_backupdr_backup: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs_export_options: Option<Vec<FilestoreInstanceFileSharesElNfsExportOptionsEl>>,
    dynamic: FilestoreInstanceFileSharesElDynamic,
}
impl FilestoreInstanceFileSharesEl {
    #[doc = "Set the field `source_backup`.\nThe resource name of the backup, in the format\nprojects/{projectId}/locations/{locationId}/backups/{backupId},\nthat this file share has been restored from."]
    pub fn set_source_backup(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_backup = Some(v.into());
        self
    }
    #[doc = "Set the field `source_backupdr_backup`.\nThe resource name of the BackupDR backup, in the format\n'projects/{project_id}/locations/{location_id}/backupVaults/{backupvault_id}/dataSources/{datasource_id}/backups/{backup_id}',\nthat this file share has been restored from."]
    pub fn set_source_backupdr_backup(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_backupdr_backup = Some(v.into());
        self
    }
    #[doc = "Set the field `nfs_export_options`.\n"]
    pub fn set_nfs_export_options(
        mut self,
        v: impl Into<BlockAssignable<FilestoreInstanceFileSharesElNfsExportOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.nfs_export_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.nfs_export_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FilestoreInstanceFileSharesEl {
    type O = BlockAssignable<FilestoreInstanceFileSharesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstanceFileSharesEl {
    #[doc = "File share capacity in GiB. This must be at least 1024 GiB\nfor the standard tier, or 2560 GiB for the premium tier."]
    pub capacity_gb: PrimField<f64>,
    #[doc = "The name of the fileshare (16 characters or less)"]
    pub name: PrimField<String>,
}
impl BuildFilestoreInstanceFileSharesEl {
    pub fn build(self) -> FilestoreInstanceFileSharesEl {
        FilestoreInstanceFileSharesEl {
            capacity_gb: self.capacity_gb,
            name: self.name,
            source_backup: core::default::Default::default(),
            source_backupdr_backup: core::default::Default::default(),
            nfs_export_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FilestoreInstanceFileSharesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceFileSharesElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceFileSharesElRef {
        FilestoreInstanceFileSharesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstanceFileSharesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capacity_gb` after provisioning.\nFile share capacity in GiB. This must be at least 1024 GiB\nfor the standard tier, or 2560 GiB for the premium tier."]
    pub fn capacity_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the fileshare (16 characters or less)"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `source_backup` after provisioning.\nThe resource name of the backup, in the format\nprojects/{projectId}/locations/{locationId}/backups/{backupId},\nthat this file share has been restored from."]
    pub fn source_backup(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_backup", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_backupdr_backup` after provisioning.\nThe resource name of the BackupDR backup, in the format\n'projects/{project_id}/locations/{location_id}/backupVaults/{backupvault_id}/dataSources/{datasource_id}/backups/{backup_id}',\nthat this file share has been restored from."]
    pub fn source_backupdr_backup(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_backupdr_backup", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nfs_export_options` after provisioning.\n"]
    pub fn nfs_export_options(
        &self,
    ) -> ListRef<FilestoreInstanceFileSharesElNfsExportOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nfs_export_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FilestoreInstanceInitialReplicationElReplicasEl {
    peer_instance: PrimField<String>,
}
impl FilestoreInstanceInitialReplicationElReplicasEl {}
impl ToListMappable for FilestoreInstanceInitialReplicationElReplicasEl {
    type O = BlockAssignable<FilestoreInstanceInitialReplicationElReplicasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstanceInitialReplicationElReplicasEl {
    #[doc = "The peer instance."]
    pub peer_instance: PrimField<String>,
}
impl BuildFilestoreInstanceInitialReplicationElReplicasEl {
    pub fn build(self) -> FilestoreInstanceInitialReplicationElReplicasEl {
        FilestoreInstanceInitialReplicationElReplicasEl {
            peer_instance: self.peer_instance,
        }
    }
}
pub struct FilestoreInstanceInitialReplicationElReplicasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceInitialReplicationElReplicasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FilestoreInstanceInitialReplicationElReplicasElRef {
        FilestoreInstanceInitialReplicationElReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstanceInitialReplicationElReplicasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `peer_instance` after provisioning.\nThe peer instance."]
    pub fn peer_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_instance", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct FilestoreInstanceInitialReplicationElDynamic {
    replicas: Option<DynamicBlock<FilestoreInstanceInitialReplicationElReplicasEl>>,
}
#[derive(Serialize)]
pub struct FilestoreInstanceInitialReplicationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas: Option<Vec<FilestoreInstanceInitialReplicationElReplicasEl>>,
    dynamic: FilestoreInstanceInitialReplicationElDynamic,
}
impl FilestoreInstanceInitialReplicationEl {
    #[doc = "Set the field `role`.\nThe replication role. Default value: \"STANDBY\" Possible values: [\"ROLE_UNSPECIFIED\", \"ACTIVE\", \"STANDBY\"]"]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl Into<BlockAssignable<FilestoreInstanceInitialReplicationElReplicasEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.replicas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.replicas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FilestoreInstanceInitialReplicationEl {
    type O = BlockAssignable<FilestoreInstanceInitialReplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstanceInitialReplicationEl {}
impl BuildFilestoreInstanceInitialReplicationEl {
    pub fn build(self) -> FilestoreInstanceInitialReplicationEl {
        FilestoreInstanceInitialReplicationEl {
            role: core::default::Default::default(),
            replicas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FilestoreInstanceInitialReplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceInitialReplicationElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceInitialReplicationElRef {
        FilestoreInstanceInitialReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstanceInitialReplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nThe replication role. Default value: \"STANDBY\" Possible values: [\"ROLE_UNSPECIFIED\", \"ACTIVE\", \"STANDBY\"]"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> ListRef<FilestoreInstanceInitialReplicationElReplicasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}
#[derive(Serialize)]
pub struct FilestoreInstanceNetworksElPscConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_project: Option<PrimField<String>>,
}
impl FilestoreInstanceNetworksElPscConfigEl {
    #[doc = "Set the field `endpoint_project`.\nConsumer service project in which the Private Service Connect endpoint\nwould be set up. This is optional, and only relevant in case the network\nis a shared VPC. If this is not specified, the endpoint would be set up\nin the VPC host project."]
    pub fn set_endpoint_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_project = Some(v.into());
        self
    }
}
impl ToListMappable for FilestoreInstanceNetworksElPscConfigEl {
    type O = BlockAssignable<FilestoreInstanceNetworksElPscConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstanceNetworksElPscConfigEl {}
impl BuildFilestoreInstanceNetworksElPscConfigEl {
    pub fn build(self) -> FilestoreInstanceNetworksElPscConfigEl {
        FilestoreInstanceNetworksElPscConfigEl {
            endpoint_project: core::default::Default::default(),
        }
    }
}
pub struct FilestoreInstanceNetworksElPscConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceNetworksElPscConfigElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceNetworksElPscConfigElRef {
        FilestoreInstanceNetworksElPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstanceNetworksElPscConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint_project` after provisioning.\nConsumer service project in which the Private Service Connect endpoint\nwould be set up. This is optional, and only relevant in case the network\nis a shared VPC. If this is not specified, the endpoint would be set up\nin the VPC host project."]
    pub fn endpoint_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_project", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct FilestoreInstanceNetworksElDynamic {
    psc_config: Option<DynamicBlock<FilestoreInstanceNetworksElPscConfigEl>>,
}
#[derive(Serialize)]
pub struct FilestoreInstanceNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connect_mode: Option<PrimField<String>>,
    modes: ListField<PrimField<String>>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reserved_ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_config: Option<Vec<FilestoreInstanceNetworksElPscConfigEl>>,
    dynamic: FilestoreInstanceNetworksElDynamic,
}
impl FilestoreInstanceNetworksEl {
    #[doc = "Set the field `connect_mode`.\nThe network connect mode of the Filestore instance.\nIf not provided, the connect mode defaults to\nDIRECT_PEERING. Default value: \"DIRECT_PEERING\" Possible values: [\"DIRECT_PEERING\", \"PRIVATE_SERVICE_ACCESS\", \"PRIVATE_SERVICE_CONNECT\"]"]
    pub fn set_connect_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connect_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `reserved_ip_range`.\nA /29 CIDR block that identifies the range of IP\naddresses reserved for this instance."]
    pub fn set_reserved_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reserved_ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        mut self,
        v: impl Into<BlockAssignable<FilestoreInstanceNetworksElPscConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.psc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FilestoreInstanceNetworksEl {
    type O = BlockAssignable<FilestoreInstanceNetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstanceNetworksEl {
    #[doc = "IP versions for which the instance has\nIP addresses assigned. Possible values: [\"ADDRESS_MODE_UNSPECIFIED\", \"MODE_IPV4\", \"MODE_IPV6\"]"]
    pub modes: ListField<PrimField<String>>,
    #[doc = "The name of the GCE VPC network to which the\ninstance is connected."]
    pub network: PrimField<String>,
}
impl BuildFilestoreInstanceNetworksEl {
    pub fn build(self) -> FilestoreInstanceNetworksEl {
        FilestoreInstanceNetworksEl {
            connect_mode: core::default::Default::default(),
            modes: self.modes,
            network: self.network,
            reserved_ip_range: core::default::Default::default(),
            psc_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FilestoreInstanceNetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceNetworksElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceNetworksElRef {
        FilestoreInstanceNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstanceNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connect_mode` after provisioning.\nThe network connect mode of the Filestore instance.\nIf not provided, the connect mode defaults to\nDIRECT_PEERING. Default value: \"DIRECT_PEERING\" Possible values: [\"DIRECT_PEERING\", \"PRIVATE_SERVICE_ACCESS\", \"PRIVATE_SERVICE_CONNECT\"]"]
    pub fn connect_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\nA list of IPv4 or IPv6 addresses."]
    pub fn ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
    #[doc = "Get a reference to the value of field `modes` after provisioning.\nIP versions for which the instance has\nIP addresses assigned. Possible values: [\"ADDRESS_MODE_UNSPECIFIED\", \"MODE_IPV4\", \"MODE_IPV6\"]"]
    pub fn modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.modes", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name of the GCE VPC network to which the\ninstance is connected."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `reserved_ip_range` after provisioning.\nA /29 CIDR block that identifies the range of IP\naddresses reserved for this instance."]
    pub fn reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_ip_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> ListRef<FilestoreInstanceNetworksElPscConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }
}
#[derive(Serialize)]
pub struct FilestoreInstancePerformanceConfigElFixedIopsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_iops: Option<PrimField<f64>>,
}
impl FilestoreInstancePerformanceConfigElFixedIopsEl {
    #[doc = "Set the field `max_iops`.\nThe number of IOPS to provision for the instance.\nmax_iops must be in multiple of 1000."]
    pub fn set_max_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_iops = Some(v.into());
        self
    }
}
impl ToListMappable for FilestoreInstancePerformanceConfigElFixedIopsEl {
    type O = BlockAssignable<FilestoreInstancePerformanceConfigElFixedIopsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstancePerformanceConfigElFixedIopsEl {}
impl BuildFilestoreInstancePerformanceConfigElFixedIopsEl {
    pub fn build(self) -> FilestoreInstancePerformanceConfigElFixedIopsEl {
        FilestoreInstancePerformanceConfigElFixedIopsEl {
            max_iops: core::default::Default::default(),
        }
    }
}
pub struct FilestoreInstancePerformanceConfigElFixedIopsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstancePerformanceConfigElFixedIopsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FilestoreInstancePerformanceConfigElFixedIopsElRef {
        FilestoreInstancePerformanceConfigElFixedIopsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstancePerformanceConfigElFixedIopsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_iops` after provisioning.\nThe number of IOPS to provision for the instance.\nmax_iops must be in multiple of 1000."]
    pub fn max_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_iops", self.base))
    }
}
#[derive(Serialize)]
pub struct FilestoreInstancePerformanceConfigElIopsPerTbEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_iops_per_tb: Option<PrimField<f64>>,
}
impl FilestoreInstancePerformanceConfigElIopsPerTbEl {
    #[doc = "Set the field `max_iops_per_tb`.\nThe instance max IOPS will be calculated by multiplying\nthe capacity of the instance (TB) by max_iops_per_tb,\nand rounding to the nearest 1000. The instance max IOPS\nwill be changed dynamically based on the instance\ncapacity."]
    pub fn set_max_iops_per_tb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_iops_per_tb = Some(v.into());
        self
    }
}
impl ToListMappable for FilestoreInstancePerformanceConfigElIopsPerTbEl {
    type O = BlockAssignable<FilestoreInstancePerformanceConfigElIopsPerTbEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstancePerformanceConfigElIopsPerTbEl {}
impl BuildFilestoreInstancePerformanceConfigElIopsPerTbEl {
    pub fn build(self) -> FilestoreInstancePerformanceConfigElIopsPerTbEl {
        FilestoreInstancePerformanceConfigElIopsPerTbEl {
            max_iops_per_tb: core::default::Default::default(),
        }
    }
}
pub struct FilestoreInstancePerformanceConfigElIopsPerTbElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstancePerformanceConfigElIopsPerTbElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FilestoreInstancePerformanceConfigElIopsPerTbElRef {
        FilestoreInstancePerformanceConfigElIopsPerTbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstancePerformanceConfigElIopsPerTbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_iops_per_tb` after provisioning.\nThe instance max IOPS will be calculated by multiplying\nthe capacity of the instance (TB) by max_iops_per_tb,\nand rounding to the nearest 1000. The instance max IOPS\nwill be changed dynamically based on the instance\ncapacity."]
    pub fn max_iops_per_tb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_iops_per_tb", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct FilestoreInstancePerformanceConfigElDynamic {
    fixed_iops: Option<DynamicBlock<FilestoreInstancePerformanceConfigElFixedIopsEl>>,
    iops_per_tb: Option<DynamicBlock<FilestoreInstancePerformanceConfigElIopsPerTbEl>>,
}
#[derive(Serialize)]
pub struct FilestoreInstancePerformanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_iops: Option<Vec<FilestoreInstancePerformanceConfigElFixedIopsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops_per_tb: Option<Vec<FilestoreInstancePerformanceConfigElIopsPerTbEl>>,
    dynamic: FilestoreInstancePerformanceConfigElDynamic,
}
impl FilestoreInstancePerformanceConfigEl {
    #[doc = "Set the field `fixed_iops`.\n"]
    pub fn set_fixed_iops(
        mut self,
        v: impl Into<BlockAssignable<FilestoreInstancePerformanceConfigElFixedIopsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_iops = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_iops = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `iops_per_tb`.\n"]
    pub fn set_iops_per_tb(
        mut self,
        v: impl Into<BlockAssignable<FilestoreInstancePerformanceConfigElIopsPerTbEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iops_per_tb = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iops_per_tb = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FilestoreInstancePerformanceConfigEl {
    type O = BlockAssignable<FilestoreInstancePerformanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstancePerformanceConfigEl {}
impl BuildFilestoreInstancePerformanceConfigEl {
    pub fn build(self) -> FilestoreInstancePerformanceConfigEl {
        FilestoreInstancePerformanceConfigEl {
            fixed_iops: core::default::Default::default(),
            iops_per_tb: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FilestoreInstancePerformanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstancePerformanceConfigElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstancePerformanceConfigElRef {
        FilestoreInstancePerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstancePerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed_iops` after provisioning.\n"]
    pub fn fixed_iops(&self) -> ListRef<FilestoreInstancePerformanceConfigElFixedIopsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_iops", self.base))
    }
    #[doc = "Get a reference to the value of field `iops_per_tb` after provisioning.\n"]
    pub fn iops_per_tb(&self) -> ListRef<FilestoreInstancePerformanceConfigElIopsPerTbElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iops_per_tb", self.base))
    }
}
#[derive(Serialize)]
pub struct FilestoreInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FilestoreInstanceTimeoutsEl {
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
impl ToListMappable for FilestoreInstanceTimeoutsEl {
    type O = BlockAssignable<FilestoreInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFilestoreInstanceTimeoutsEl {}
impl BuildFilestoreInstanceTimeoutsEl {
    pub fn build(self) -> FilestoreInstanceTimeoutsEl {
        FilestoreInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FilestoreInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FilestoreInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceTimeoutsElRef {
        FilestoreInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FilestoreInstanceTimeoutsElRef {
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
struct FilestoreInstanceDynamic {
    directory_services: Option<DynamicBlock<FilestoreInstanceDirectoryServicesEl>>,
    file_shares: Option<DynamicBlock<FilestoreInstanceFileSharesEl>>,
    initial_replication: Option<DynamicBlock<FilestoreInstanceInitialReplicationEl>>,
    networks: Option<DynamicBlock<FilestoreInstanceNetworksEl>>,
    performance_config: Option<DynamicBlock<FilestoreInstancePerformanceConfigEl>>,
}
