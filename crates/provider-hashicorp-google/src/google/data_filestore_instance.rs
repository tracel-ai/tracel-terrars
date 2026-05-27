use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataFilestoreInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataFilestoreInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataFilestoreInstanceData>,
}
#[derive(Clone)]
pub struct DataFilestoreInstance(Rc<DataFilestoreInstance_>);
impl DataFilestoreInstance {
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
    #[doc = "Get a reference to the value of field `directory_services` after provisioning.\nDirectory Services configuration.\nShould only be set if protocol is \"NFS_V4_1\"."]
    pub fn directory_services(&self) -> ListRef<DataFilestoreInstanceDirectoryServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.directory_services", self.extract_ref()),
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
    pub fn effective_replication(&self) -> ListRef<DataFilestoreInstanceEffectiveReplicationElRef> {
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
    #[doc = "Get a reference to the value of field `file_shares` after provisioning.\nFile system shares on the instance. For this version, only a\nsingle file share is supported."]
    pub fn file_shares(&self) -> ListRef<DataFilestoreInstanceFileSharesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_shares", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_replication` after provisioning.\nReplication configuration, once set, this cannot be updated.\nAdditionally this should be specified on the replica instance only, indicating the active as the peer_instance"]
    pub fn initial_replication(&self) -> ListRef<DataFilestoreInstanceInitialReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initial_replication", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `networks` after provisioning.\nVPC networks to which the instance is connected. For this version,\nonly a single network is supported."]
    pub fn networks(&self) -> ListRef<DataFilestoreInstanceNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_config` after provisioning.\nPerformance configuration for the instance. If not provided,\nthe default performance settings will be used."]
    pub fn performance_config(&self) -> ListRef<DataFilestoreInstancePerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.performance_config", self.extract_ref()),
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
}
impl Referable for DataFilestoreInstance {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataFilestoreInstance {}
impl ToListMappable for DataFilestoreInstance {
    type O = ListRef<DataFilestoreInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataFilestoreInstance_ {
    fn extract_datasource_type(&self) -> String {
        "google_filestore_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataFilestoreInstance {
    pub tf_id: String,
    #[doc = "The resource name of the instance."]
    pub name: PrimField<String>,
}
impl BuildDataFilestoreInstance {
    pub fn build(self, stack: &mut Stack) -> DataFilestoreInstance {
        let out = DataFilestoreInstance(Rc::new(DataFilestoreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataFilestoreInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataFilestoreInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataFilestoreInstanceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    #[doc = "Get a reference to the value of field `directory_services` after provisioning.\nDirectory Services configuration.\nShould only be set if protocol is \"NFS_V4_1\"."]
    pub fn directory_services(&self) -> ListRef<DataFilestoreInstanceDirectoryServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.directory_services", self.extract_ref()),
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
    pub fn effective_replication(&self) -> ListRef<DataFilestoreInstanceEffectiveReplicationElRef> {
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
    #[doc = "Get a reference to the value of field `file_shares` after provisioning.\nFile system shares on the instance. For this version, only a\nsingle file share is supported."]
    pub fn file_shares(&self) -> ListRef<DataFilestoreInstanceFileSharesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_shares", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_replication` after provisioning.\nReplication configuration, once set, this cannot be updated.\nAdditionally this should be specified on the replica instance only, indicating the active as the peer_instance"]
    pub fn initial_replication(&self) -> ListRef<DataFilestoreInstanceInitialReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initial_replication", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `networks` after provisioning.\nVPC networks to which the instance is connected. For this version,\nonly a single network is supported."]
    pub fn networks(&self) -> ListRef<DataFilestoreInstanceNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_config` after provisioning.\nPerformance configuration for the instance. If not provided,\nthe default performance settings will be used."]
    pub fn performance_config(&self) -> ListRef<DataFilestoreInstancePerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.performance_config", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataFilestoreInstanceDirectoryServicesElLdapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups_ou: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    servers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    users_ou: Option<PrimField<String>>,
}
impl DataFilestoreInstanceDirectoryServicesElLdapEl {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
    #[doc = "Set the field `groups_ou`.\n"]
    pub fn set_groups_ou(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.groups_ou = Some(v.into());
        self
    }
    #[doc = "Set the field `servers`.\n"]
    pub fn set_servers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.servers = Some(v.into());
        self
    }
    #[doc = "Set the field `users_ou`.\n"]
    pub fn set_users_ou(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.users_ou = Some(v.into());
        self
    }
}
impl ToListMappable for DataFilestoreInstanceDirectoryServicesElLdapEl {
    type O = BlockAssignable<DataFilestoreInstanceDirectoryServicesElLdapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstanceDirectoryServicesElLdapEl {}
impl BuildDataFilestoreInstanceDirectoryServicesElLdapEl {
    pub fn build(self) -> DataFilestoreInstanceDirectoryServicesElLdapEl {
        DataFilestoreInstanceDirectoryServicesElLdapEl {
            domain: core::default::Default::default(),
            groups_ou: core::default::Default::default(),
            servers: core::default::Default::default(),
            users_ou: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstanceDirectoryServicesElLdapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstanceDirectoryServicesElLdapElRef {
    fn new(shared: StackShared, base: String) -> DataFilestoreInstanceDirectoryServicesElLdapElRef {
        DataFilestoreInstanceDirectoryServicesElLdapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstanceDirectoryServicesElLdapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `groups_ou` after provisioning.\n"]
    pub fn groups_ou(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.groups_ou", self.base))
    }
    #[doc = "Get a reference to the value of field `servers` after provisioning.\n"]
    pub fn servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }
    #[doc = "Get a reference to the value of field `users_ou` after provisioning.\n"]
    pub fn users_ou(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.users_ou", self.base))
    }
}
#[derive(Serialize)]
pub struct DataFilestoreInstanceDirectoryServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ldap: Option<ListField<DataFilestoreInstanceDirectoryServicesElLdapEl>>,
}
impl DataFilestoreInstanceDirectoryServicesEl {
    #[doc = "Set the field `ldap`.\n"]
    pub fn set_ldap(
        mut self,
        v: impl Into<ListField<DataFilestoreInstanceDirectoryServicesElLdapEl>>,
    ) -> Self {
        self.ldap = Some(v.into());
        self
    }
}
impl ToListMappable for DataFilestoreInstanceDirectoryServicesEl {
    type O = BlockAssignable<DataFilestoreInstanceDirectoryServicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstanceDirectoryServicesEl {}
impl BuildDataFilestoreInstanceDirectoryServicesEl {
    pub fn build(self) -> DataFilestoreInstanceDirectoryServicesEl {
        DataFilestoreInstanceDirectoryServicesEl {
            ldap: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstanceDirectoryServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstanceDirectoryServicesElRef {
    fn new(shared: StackShared, base: String) -> DataFilestoreInstanceDirectoryServicesElRef {
        DataFilestoreInstanceDirectoryServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstanceDirectoryServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ldap` after provisioning.\n"]
    pub fn ldap(&self) -> ListRef<DataFilestoreInstanceDirectoryServicesElLdapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ldap", self.base))
    }
}
#[derive(Serialize)]
pub struct DataFilestoreInstanceEffectiveReplicationElReplicasEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_active_sync_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_reasons: Option<ListField<PrimField<String>>>,
}
impl DataFilestoreInstanceEffectiveReplicationElReplicasEl {
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
impl ToListMappable for DataFilestoreInstanceEffectiveReplicationElReplicasEl {
    type O = BlockAssignable<DataFilestoreInstanceEffectiveReplicationElReplicasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstanceEffectiveReplicationElReplicasEl {}
impl BuildDataFilestoreInstanceEffectiveReplicationElReplicasEl {
    pub fn build(self) -> DataFilestoreInstanceEffectiveReplicationElReplicasEl {
        DataFilestoreInstanceEffectiveReplicationElReplicasEl {
            last_active_sync_time: core::default::Default::default(),
            peer_instance: core::default::Default::default(),
            state: core::default::Default::default(),
            state_reasons: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstanceEffectiveReplicationElReplicasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstanceEffectiveReplicationElReplicasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataFilestoreInstanceEffectiveReplicationElReplicasElRef {
        DataFilestoreInstanceEffectiveReplicationElReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstanceEffectiveReplicationElReplicasElRef {
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
pub struct DataFilestoreInstanceEffectiveReplicationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas: Option<ListField<DataFilestoreInstanceEffectiveReplicationElReplicasEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
}
impl DataFilestoreInstanceEffectiveReplicationEl {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl Into<ListField<DataFilestoreInstanceEffectiveReplicationElReplicasEl>>,
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
impl ToListMappable for DataFilestoreInstanceEffectiveReplicationEl {
    type O = BlockAssignable<DataFilestoreInstanceEffectiveReplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstanceEffectiveReplicationEl {}
impl BuildDataFilestoreInstanceEffectiveReplicationEl {
    pub fn build(self) -> DataFilestoreInstanceEffectiveReplicationEl {
        DataFilestoreInstanceEffectiveReplicationEl {
            replicas: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstanceEffectiveReplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstanceEffectiveReplicationElRef {
    fn new(shared: StackShared, base: String) -> DataFilestoreInstanceEffectiveReplicationElRef {
        DataFilestoreInstanceEffectiveReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstanceEffectiveReplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> ListRef<DataFilestoreInstanceEffectiveReplicationElReplicasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}
#[derive(Serialize)]
pub struct DataFilestoreInstanceFileSharesElNfsExportOptionsEl {
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
impl DataFilestoreInstanceFileSharesElNfsExportOptionsEl {
    #[doc = "Set the field `access_mode`.\n"]
    pub fn set_access_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `anon_gid`.\n"]
    pub fn set_anon_gid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.anon_gid = Some(v.into());
        self
    }
    #[doc = "Set the field `anon_uid`.\n"]
    pub fn set_anon_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.anon_uid = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_ranges`.\n"]
    pub fn set_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `squash_mode`.\n"]
    pub fn set_squash_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.squash_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataFilestoreInstanceFileSharesElNfsExportOptionsEl {
    type O = BlockAssignable<DataFilestoreInstanceFileSharesElNfsExportOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstanceFileSharesElNfsExportOptionsEl {}
impl BuildDataFilestoreInstanceFileSharesElNfsExportOptionsEl {
    pub fn build(self) -> DataFilestoreInstanceFileSharesElNfsExportOptionsEl {
        DataFilestoreInstanceFileSharesElNfsExportOptionsEl {
            access_mode: core::default::Default::default(),
            anon_gid: core::default::Default::default(),
            anon_uid: core::default::Default::default(),
            ip_ranges: core::default::Default::default(),
            network: core::default::Default::default(),
            squash_mode: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstanceFileSharesElNfsExportOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstanceFileSharesElNfsExportOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataFilestoreInstanceFileSharesElNfsExportOptionsElRef {
        DataFilestoreInstanceFileSharesElNfsExportOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstanceFileSharesElNfsExportOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\n"]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `anon_gid` after provisioning.\n"]
    pub fn anon_gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.anon_gid", self.base))
    }
    #[doc = "Get a reference to the value of field `anon_uid` after provisioning.\n"]
    pub fn anon_uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.anon_uid", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_ranges` after provisioning.\n"]
    pub fn ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_ranges", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `squash_mode` after provisioning.\n"]
    pub fn squash_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataFilestoreInstanceFileSharesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs_export_options: Option<ListField<DataFilestoreInstanceFileSharesElNfsExportOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_backup: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_backupdr_backup: Option<PrimField<String>>,
}
impl DataFilestoreInstanceFileSharesEl {
    #[doc = "Set the field `capacity_gb`.\n"]
    pub fn set_capacity_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.capacity_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `nfs_export_options`.\n"]
    pub fn set_nfs_export_options(
        mut self,
        v: impl Into<ListField<DataFilestoreInstanceFileSharesElNfsExportOptionsEl>>,
    ) -> Self {
        self.nfs_export_options = Some(v.into());
        self
    }
    #[doc = "Set the field `source_backup`.\n"]
    pub fn set_source_backup(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_backup = Some(v.into());
        self
    }
    #[doc = "Set the field `source_backupdr_backup`.\n"]
    pub fn set_source_backupdr_backup(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_backupdr_backup = Some(v.into());
        self
    }
}
impl ToListMappable for DataFilestoreInstanceFileSharesEl {
    type O = BlockAssignable<DataFilestoreInstanceFileSharesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstanceFileSharesEl {}
impl BuildDataFilestoreInstanceFileSharesEl {
    pub fn build(self) -> DataFilestoreInstanceFileSharesEl {
        DataFilestoreInstanceFileSharesEl {
            capacity_gb: core::default::Default::default(),
            name: core::default::Default::default(),
            nfs_export_options: core::default::Default::default(),
            source_backup: core::default::Default::default(),
            source_backupdr_backup: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstanceFileSharesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstanceFileSharesElRef {
    fn new(shared: StackShared, base: String) -> DataFilestoreInstanceFileSharesElRef {
        DataFilestoreInstanceFileSharesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstanceFileSharesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capacity_gb` after provisioning.\n"]
    pub fn capacity_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `nfs_export_options` after provisioning.\n"]
    pub fn nfs_export_options(
        &self,
    ) -> ListRef<DataFilestoreInstanceFileSharesElNfsExportOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nfs_export_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_backup` after provisioning.\n"]
    pub fn source_backup(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_backup", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_backupdr_backup` after provisioning.\n"]
    pub fn source_backupdr_backup(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_backupdr_backup", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataFilestoreInstanceInitialReplicationElReplicasEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_instance: Option<PrimField<String>>,
}
impl DataFilestoreInstanceInitialReplicationElReplicasEl {
    #[doc = "Set the field `peer_instance`.\n"]
    pub fn set_peer_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_instance = Some(v.into());
        self
    }
}
impl ToListMappable for DataFilestoreInstanceInitialReplicationElReplicasEl {
    type O = BlockAssignable<DataFilestoreInstanceInitialReplicationElReplicasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstanceInitialReplicationElReplicasEl {}
impl BuildDataFilestoreInstanceInitialReplicationElReplicasEl {
    pub fn build(self) -> DataFilestoreInstanceInitialReplicationElReplicasEl {
        DataFilestoreInstanceInitialReplicationElReplicasEl {
            peer_instance: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstanceInitialReplicationElReplicasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstanceInitialReplicationElReplicasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataFilestoreInstanceInitialReplicationElReplicasElRef {
        DataFilestoreInstanceInitialReplicationElReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstanceInitialReplicationElReplicasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `peer_instance` after provisioning.\n"]
    pub fn peer_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_instance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataFilestoreInstanceInitialReplicationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas: Option<ListField<DataFilestoreInstanceInitialReplicationElReplicasEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
}
impl DataFilestoreInstanceInitialReplicationEl {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl Into<ListField<DataFilestoreInstanceInitialReplicationElReplicasEl>>,
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
impl ToListMappable for DataFilestoreInstanceInitialReplicationEl {
    type O = BlockAssignable<DataFilestoreInstanceInitialReplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstanceInitialReplicationEl {}
impl BuildDataFilestoreInstanceInitialReplicationEl {
    pub fn build(self) -> DataFilestoreInstanceInitialReplicationEl {
        DataFilestoreInstanceInitialReplicationEl {
            replicas: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstanceInitialReplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstanceInitialReplicationElRef {
    fn new(shared: StackShared, base: String) -> DataFilestoreInstanceInitialReplicationElRef {
        DataFilestoreInstanceInitialReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstanceInitialReplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> ListRef<DataFilestoreInstanceInitialReplicationElReplicasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}
#[derive(Serialize)]
pub struct DataFilestoreInstanceNetworksElPscConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_project: Option<PrimField<String>>,
}
impl DataFilestoreInstanceNetworksElPscConfigEl {
    #[doc = "Set the field `endpoint_project`.\n"]
    pub fn set_endpoint_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_project = Some(v.into());
        self
    }
}
impl ToListMappable for DataFilestoreInstanceNetworksElPscConfigEl {
    type O = BlockAssignable<DataFilestoreInstanceNetworksElPscConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstanceNetworksElPscConfigEl {}
impl BuildDataFilestoreInstanceNetworksElPscConfigEl {
    pub fn build(self) -> DataFilestoreInstanceNetworksElPscConfigEl {
        DataFilestoreInstanceNetworksElPscConfigEl {
            endpoint_project: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstanceNetworksElPscConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstanceNetworksElPscConfigElRef {
    fn new(shared: StackShared, base: String) -> DataFilestoreInstanceNetworksElPscConfigElRef {
        DataFilestoreInstanceNetworksElPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstanceNetworksElPscConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint_project` after provisioning.\n"]
    pub fn endpoint_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_project", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataFilestoreInstanceNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connect_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_config: Option<ListField<DataFilestoreInstanceNetworksElPscConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reserved_ip_range: Option<PrimField<String>>,
}
impl DataFilestoreInstanceNetworksEl {
    #[doc = "Set the field `connect_mode`.\n"]
    pub fn set_connect_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connect_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `modes`.\n"]
    pub fn set_modes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.modes = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        mut self,
        v: impl Into<ListField<DataFilestoreInstanceNetworksElPscConfigEl>>,
    ) -> Self {
        self.psc_config = Some(v.into());
        self
    }
    #[doc = "Set the field `reserved_ip_range`.\n"]
    pub fn set_reserved_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reserved_ip_range = Some(v.into());
        self
    }
}
impl ToListMappable for DataFilestoreInstanceNetworksEl {
    type O = BlockAssignable<DataFilestoreInstanceNetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstanceNetworksEl {}
impl BuildDataFilestoreInstanceNetworksEl {
    pub fn build(self) -> DataFilestoreInstanceNetworksEl {
        DataFilestoreInstanceNetworksEl {
            connect_mode: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
            modes: core::default::Default::default(),
            network: core::default::Default::default(),
            psc_config: core::default::Default::default(),
            reserved_ip_range: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstanceNetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstanceNetworksElRef {
    fn new(shared: StackShared, base: String) -> DataFilestoreInstanceNetworksElRef {
        DataFilestoreInstanceNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstanceNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connect_mode` after provisioning.\n"]
    pub fn connect_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
    #[doc = "Get a reference to the value of field `modes` after provisioning.\n"]
    pub fn modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.modes", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> ListRef<DataFilestoreInstanceNetworksElPscConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.psc_config", self.base))
    }
    #[doc = "Get a reference to the value of field `reserved_ip_range` after provisioning.\n"]
    pub fn reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_ip_range", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataFilestoreInstancePerformanceConfigElFixedIopsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_iops: Option<PrimField<f64>>,
}
impl DataFilestoreInstancePerformanceConfigElFixedIopsEl {
    #[doc = "Set the field `max_iops`.\n"]
    pub fn set_max_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_iops = Some(v.into());
        self
    }
}
impl ToListMappable for DataFilestoreInstancePerformanceConfigElFixedIopsEl {
    type O = BlockAssignable<DataFilestoreInstancePerformanceConfigElFixedIopsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstancePerformanceConfigElFixedIopsEl {}
impl BuildDataFilestoreInstancePerformanceConfigElFixedIopsEl {
    pub fn build(self) -> DataFilestoreInstancePerformanceConfigElFixedIopsEl {
        DataFilestoreInstancePerformanceConfigElFixedIopsEl {
            max_iops: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstancePerformanceConfigElFixedIopsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstancePerformanceConfigElFixedIopsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataFilestoreInstancePerformanceConfigElFixedIopsElRef {
        DataFilestoreInstancePerformanceConfigElFixedIopsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstancePerformanceConfigElFixedIopsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_iops` after provisioning.\n"]
    pub fn max_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_iops", self.base))
    }
}
#[derive(Serialize)]
pub struct DataFilestoreInstancePerformanceConfigElIopsPerTbEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_iops_per_tb: Option<PrimField<f64>>,
}
impl DataFilestoreInstancePerformanceConfigElIopsPerTbEl {
    #[doc = "Set the field `max_iops_per_tb`.\n"]
    pub fn set_max_iops_per_tb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_iops_per_tb = Some(v.into());
        self
    }
}
impl ToListMappable for DataFilestoreInstancePerformanceConfigElIopsPerTbEl {
    type O = BlockAssignable<DataFilestoreInstancePerformanceConfigElIopsPerTbEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstancePerformanceConfigElIopsPerTbEl {}
impl BuildDataFilestoreInstancePerformanceConfigElIopsPerTbEl {
    pub fn build(self) -> DataFilestoreInstancePerformanceConfigElIopsPerTbEl {
        DataFilestoreInstancePerformanceConfigElIopsPerTbEl {
            max_iops_per_tb: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstancePerformanceConfigElIopsPerTbElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstancePerformanceConfigElIopsPerTbElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataFilestoreInstancePerformanceConfigElIopsPerTbElRef {
        DataFilestoreInstancePerformanceConfigElIopsPerTbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstancePerformanceConfigElIopsPerTbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_iops_per_tb` after provisioning.\n"]
    pub fn max_iops_per_tb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_iops_per_tb", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataFilestoreInstancePerformanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_iops: Option<ListField<DataFilestoreInstancePerformanceConfigElFixedIopsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops_per_tb: Option<ListField<DataFilestoreInstancePerformanceConfigElIopsPerTbEl>>,
}
impl DataFilestoreInstancePerformanceConfigEl {
    #[doc = "Set the field `fixed_iops`.\n"]
    pub fn set_fixed_iops(
        mut self,
        v: impl Into<ListField<DataFilestoreInstancePerformanceConfigElFixedIopsEl>>,
    ) -> Self {
        self.fixed_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `iops_per_tb`.\n"]
    pub fn set_iops_per_tb(
        mut self,
        v: impl Into<ListField<DataFilestoreInstancePerformanceConfigElIopsPerTbEl>>,
    ) -> Self {
        self.iops_per_tb = Some(v.into());
        self
    }
}
impl ToListMappable for DataFilestoreInstancePerformanceConfigEl {
    type O = BlockAssignable<DataFilestoreInstancePerformanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataFilestoreInstancePerformanceConfigEl {}
impl BuildDataFilestoreInstancePerformanceConfigEl {
    pub fn build(self) -> DataFilestoreInstancePerformanceConfigEl {
        DataFilestoreInstancePerformanceConfigEl {
            fixed_iops: core::default::Default::default(),
            iops_per_tb: core::default::Default::default(),
        }
    }
}
pub struct DataFilestoreInstancePerformanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataFilestoreInstancePerformanceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataFilestoreInstancePerformanceConfigElRef {
        DataFilestoreInstancePerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataFilestoreInstancePerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed_iops` after provisioning.\n"]
    pub fn fixed_iops(&self) -> ListRef<DataFilestoreInstancePerformanceConfigElFixedIopsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_iops", self.base))
    }
    #[doc = "Get a reference to the value of field `iops_per_tb` after provisioning.\n"]
    pub fn iops_per_tb(&self) -> ListRef<DataFilestoreInstancePerformanceConfigElIopsPerTbElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iops_per_tb", self.base))
    }
}
