use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ManagedKafkaAclData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    acl_id: PrimField<String>,
    cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acl_entries: Option<Vec<ManagedKafkaAclAclEntriesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ManagedKafkaAclTimeoutsEl>,
    dynamic: ManagedKafkaAclDynamic,
}
struct ManagedKafkaAcl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ManagedKafkaAclData>,
}
#[derive(Clone)]
pub struct ManagedKafkaAcl(Rc<ManagedKafkaAcl_>);
impl ManagedKafkaAcl {
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
    #[doc = "Set the field `acl_entries`.\n"]
    pub fn set_acl_entries(
        self,
        v: impl Into<BlockAssignable<ManagedKafkaAclAclEntriesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().acl_entries = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.acl_entries = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ManagedKafkaAclTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `acl_id` after provisioning.\nThe ID to use for the acl, which will become the final component of the acl's name. The structure of 'aclId' defines the Resource Pattern (resource_type, resource_name, pattern_type) of the acl. 'aclId' is structured like one of the following:\nFor acls on the cluster: 'cluster'\nFor acls on a single resource within the cluster: 'topic/{resource_name}' 'consumerGroup/{resource_name}' 'transactionalId/{resource_name}'\nFor acls on all resources that match a prefix: 'topicPrefixed/{resource_name}' 'consumerGroupPrefixed/{resource_name}' 'transactionalIdPrefixed/{resource_name}'\nFor acls on all resources of a given type (i.e. the wildcard literal '*''): 'allTopics' (represents 'topic/*') 'allConsumerGroups' (represents 'consumerGroup/*') 'allTransactionalIds' (represents 'transactionalId/*')."]
    pub fn acl_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.acl_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe cluster name."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n'etag' is used for concurrency control. An 'etag' is returned in the\nresponse to 'GetAcl' and 'CreateAcl'. Callers are required to put that etag\nin the request to 'UpdateAcl' to ensure that their change will be applied\nto the same version of the acl that exists in the Kafka Cluster.\n\nA terminal 'T' character in the etag indicates that the AclEntries were\ntruncated due to repeated field limits."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the acl. The 'ACL_ID' segment is used when connecting directly to the cluster. Must be in the format 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID/acls/ACL_ID'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pattern_type` after provisioning.\nThe acl pattern type derived from the name. One of: LITERAL, PREFIXED."]
    pub fn pattern_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pattern_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_name` after provisioning.\nThe acl resource name derived from the name. For cluster resource_type, this is always \"kafka-cluster\". Can be the wildcard literal \"*\"."]
    pub fn resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe acl resource type derived from the name. One of: CLUSTER, TOPIC, GROUP, TRANSACTIONAL_ID."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaAclTimeoutsElRef {
        ManagedKafkaAclTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ManagedKafkaAcl {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ManagedKafkaAcl {}
impl ToListMappable for ManagedKafkaAcl {
    type O = ListRef<ManagedKafkaAclRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ManagedKafkaAcl_ {
    fn extract_resource_type(&self) -> String {
        "google_managed_kafka_acl".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildManagedKafkaAcl {
    pub tf_id: String,
    #[doc = "The ID to use for the acl, which will become the final component of the acl's name. The structure of 'aclId' defines the Resource Pattern (resource_type, resource_name, pattern_type) of the acl. 'aclId' is structured like one of the following:\nFor acls on the cluster: 'cluster'\nFor acls on a single resource within the cluster: 'topic/{resource_name}' 'consumerGroup/{resource_name}' 'transactionalId/{resource_name}'\nFor acls on all resources that match a prefix: 'topicPrefixed/{resource_name}' 'consumerGroupPrefixed/{resource_name}' 'transactionalIdPrefixed/{resource_name}'\nFor acls on all resources of a given type (i.e. the wildcard literal '*''): 'allTopics' (represents 'topic/*') 'allConsumerGroups' (represents 'consumerGroup/*') 'allTransactionalIds' (represents 'transactionalId/*')."]
    pub acl_id: PrimField<String>,
    #[doc = "The cluster name."]
    pub cluster: PrimField<String>,
    #[doc = "ID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub location: PrimField<String>,
}
impl BuildManagedKafkaAcl {
    pub fn build(self, stack: &mut Stack) -> ManagedKafkaAcl {
        let out = ManagedKafkaAcl(Rc::new(ManagedKafkaAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ManagedKafkaAclData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                acl_id: self.acl_id,
                cluster: self.cluster,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                acl_entries: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ManagedKafkaAclRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaAclRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ManagedKafkaAclRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `acl_id` after provisioning.\nThe ID to use for the acl, which will become the final component of the acl's name. The structure of 'aclId' defines the Resource Pattern (resource_type, resource_name, pattern_type) of the acl. 'aclId' is structured like one of the following:\nFor acls on the cluster: 'cluster'\nFor acls on a single resource within the cluster: 'topic/{resource_name}' 'consumerGroup/{resource_name}' 'transactionalId/{resource_name}'\nFor acls on all resources that match a prefix: 'topicPrefixed/{resource_name}' 'consumerGroupPrefixed/{resource_name}' 'transactionalIdPrefixed/{resource_name}'\nFor acls on all resources of a given type (i.e. the wildcard literal '*''): 'allTopics' (represents 'topic/*') 'allConsumerGroups' (represents 'consumerGroup/*') 'allTransactionalIds' (represents 'transactionalId/*')."]
    pub fn acl_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.acl_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe cluster name."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n'etag' is used for concurrency control. An 'etag' is returned in the\nresponse to 'GetAcl' and 'CreateAcl'. Callers are required to put that etag\nin the request to 'UpdateAcl' to ensure that their change will be applied\nto the same version of the acl that exists in the Kafka Cluster.\n\nA terminal 'T' character in the etag indicates that the AclEntries were\ntruncated due to repeated field limits."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the acl. The 'ACL_ID' segment is used when connecting directly to the cluster. Must be in the format 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID/acls/ACL_ID'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pattern_type` after provisioning.\nThe acl pattern type derived from the name. One of: LITERAL, PREFIXED."]
    pub fn pattern_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pattern_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_name` after provisioning.\nThe acl resource name derived from the name. For cluster resource_type, this is always \"kafka-cluster\". Can be the wildcard literal \"*\"."]
    pub fn resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe acl resource type derived from the name. One of: CLUSTER, TOPIC, GROUP, TRANSACTIONAL_ID."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaAclTimeoutsElRef {
        ManagedKafkaAclTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaAclAclEntriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    operation: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_type: Option<PrimField<String>>,
    principal: PrimField<String>,
}
impl ManagedKafkaAclAclEntriesEl {
    #[doc = "Set the field `host`.\nThe host. Must be set to \"*\" for Managed Service for Apache Kafka."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `permission_type`.\nThe permission type. Accepted values are (case insensitive): ALLOW, DENY."]
    pub fn set_permission_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permission_type = Some(v.into());
        self
    }
}
impl ToListMappable for ManagedKafkaAclAclEntriesEl {
    type O = BlockAssignable<ManagedKafkaAclAclEntriesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaAclAclEntriesEl {
    #[doc = "The operation type. Allowed values are (case insensitive): ALL, READ,\nWRITE, CREATE, DELETE, ALTER, DESCRIBE, CLUSTER_ACTION, DESCRIBE_CONFIGS,\nALTER_CONFIGS, and IDEMPOTENT_WRITE. See https://kafka.apache.org/documentation/#operations_resources_and_protocols\nfor valid combinations of resource_type and operation for different Kafka API requests."]
    pub operation: PrimField<String>,
    #[doc = "The principal. Specified as Google Cloud account, with the Kafka StandardAuthorizer prefix User:\". For example: \"User:test-kafka-client@test-project.iam.gserviceaccount.com\". Can be the wildcard \"User:*\" to refer to all users."]
    pub principal: PrimField<String>,
}
impl BuildManagedKafkaAclAclEntriesEl {
    pub fn build(self) -> ManagedKafkaAclAclEntriesEl {
        ManagedKafkaAclAclEntriesEl {
            host: core::default::Default::default(),
            operation: self.operation,
            permission_type: core::default::Default::default(),
            principal: self.principal,
        }
    }
}
pub struct ManagedKafkaAclAclEntriesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaAclAclEntriesElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaAclAclEntriesElRef {
        ManagedKafkaAclAclEntriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaAclAclEntriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe host. Must be set to \"*\" for Managed Service for Apache Kafka."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\nThe operation type. Allowed values are (case insensitive): ALL, READ,\nWRITE, CREATE, DELETE, ALTER, DESCRIBE, CLUSTER_ACTION, DESCRIBE_CONFIGS,\nALTER_CONFIGS, and IDEMPOTENT_WRITE. See https://kafka.apache.org/documentation/#operations_resources_and_protocols\nfor valid combinations of resource_type and operation for different Kafka API requests."]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
    #[doc = "Get a reference to the value of field `permission_type` after provisioning.\nThe permission type. Accepted values are (case insensitive): ALLOW, DENY."]
    pub fn permission_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permission_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\nThe principal. Specified as Google Cloud account, with the Kafka StandardAuthorizer prefix User:\". For example: \"User:test-kafka-client@test-project.iam.gserviceaccount.com\". Can be the wildcard \"User:*\" to refer to all users."]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize)]
pub struct ManagedKafkaAclTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ManagedKafkaAclTimeoutsEl {
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
impl ToListMappable for ManagedKafkaAclTimeoutsEl {
    type O = BlockAssignable<ManagedKafkaAclTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildManagedKafkaAclTimeoutsEl {}
impl BuildManagedKafkaAclTimeoutsEl {
    pub fn build(self) -> ManagedKafkaAclTimeoutsEl {
        ManagedKafkaAclTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ManagedKafkaAclTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ManagedKafkaAclTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ManagedKafkaAclTimeoutsElRef {
        ManagedKafkaAclTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ManagedKafkaAclTimeoutsElRef {
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
struct ManagedKafkaAclDynamic {
    acl_entries: Option<DynamicBlock<ManagedKafkaAclAclEntriesEl>>,
}
