use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ParallelstoreInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    capacity_gib: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_stripe_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_stripe_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reserved_ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ParallelstoreInstanceTimeoutsEl>,
}
struct ParallelstoreInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ParallelstoreInstanceData>,
}
#[derive(Clone)]
pub struct ParallelstoreInstance(Rc<ParallelstoreInstance_>);
impl ParallelstoreInstance {
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
    #[doc = "Set the field `deployment_type`.\nParallelstore Instance deployment type.\n  Possible values:\n  DEPLOYMENT_TYPE_UNSPECIFIED\n  SCRATCH\n  PERSISTENT"]
    pub fn set_deployment_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deployment_type = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of the instance. 2048 characters or less."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `directory_stripe_level`.\nStripe level for directories.\nMIN when directory has a small number of files.\nMAX when directory has a large number of files.\n  Possible values:\n  DIRECTORY_STRIPE_LEVEL_UNSPECIFIED\n  DIRECTORY_STRIPE_LEVEL_MIN\n  DIRECTORY_STRIPE_LEVEL_BALANCED\n  DIRECTORY_STRIPE_LEVEL_MAX"]
    pub fn set_directory_stripe_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().directory_stripe_level = Some(v.into());
        self
    }
    #[doc = "Set the field `file_stripe_level`.\nStripe level for files.\nMIN better suited for small size files.\nMAX higher throughput performance for larger files.\n  Possible values:\n  FILE_STRIPE_LEVEL_UNSPECIFIED\n  FILE_STRIPE_LEVEL_MIN\n  FILE_STRIPE_LEVEL_BALANCED\n  FILE_STRIPE_LEVEL_MAX"]
    pub fn set_file_stripe_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().file_stripe_level = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nCloud Labels are a flexible and lightweight mechanism for\norganizing cloud resources into groups that reflect a customer's organizational\nneeds and deployment strategies. Cloud Labels can be used to filter collections\nof resources. They can be used to control how resource metrics are aggregated.\nAnd they can be used as arguments to policy management rules (e.g. route, firewall,\nload balancing, etc.).\n\n* Label keys must be between 1 and 63 characters long and must conform to\n the following regular expression: 'a-z{0,62}'.\n* Label values must be between 0 and 63 characters long and must conform\n to the regular expression '[a-z0-9_-]{0,63}'.\n* No more than 64 labels can be associated with a given resource.\n\nSee https://goo.gl/xmQnxf for more information on and examples of labels.\n\nIf you plan to use labels in your own code, please note that additional\ncharacters may be allowed in the future. Therefore, you are advised to use\nan internal label representation, such as JSON, which doesn't rely upon\nspecific characters being disallowed.  For example, representing labels\nas the string:  'name + \"_\" + value' would prove problematic if we were to\nallow '\"_\"' in a future release. \"\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nImmutable. The name of the Google Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc)\nto which the instance is connected."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `reserved_ip_range`.\nImmutable. Contains the id of the allocated IP address range\nassociated with the private service access connection for example, \\\"test-default\\\"\nassociated with IP range 10.0.0.0/29. If no range id is provided all ranges will\nbe considered."]
    pub fn set_reserved_ip_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().reserved_ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ParallelstoreInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `access_points` after provisioning.\nOutput only. List of access_points.\nContains a list of IPv4 addresses used for client side configuration."]
    pub fn access_points(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_points", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_gib` after provisioning.\nRequired. Immutable. Storage capacity of Parallelstore instance in Gibibytes (GiB)."]
    pub fn capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `daos_version` after provisioning.\nThe version of DAOS software running in the instance."]
    pub fn daos_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.daos_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\nParallelstore Instance deployment type.\n  Possible values:\n  DEPLOYMENT_TYPE_UNSPECIFIED\n  SCRATCH\n  PERSISTENT"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the instance. 2048 characters or less."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_stripe_level` after provisioning.\nStripe level for directories.\nMIN when directory has a small number of files.\nMAX when directory has a large number of files.\n  Possible values:\n  DIRECTORY_STRIPE_LEVEL_UNSPECIFIED\n  DIRECTORY_STRIPE_LEVEL_MIN\n  DIRECTORY_STRIPE_LEVEL_BALANCED\n  DIRECTORY_STRIPE_LEVEL_MAX"]
    pub fn directory_stripe_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_stripe_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_reserved_ip_range` after provisioning.\nImmutable. Contains the id of the allocated IP address\nrange associated with the private service access connection for example, \\\"test-default\\\"\nassociated with IP range 10.0.0.0/29. This field is populated by the service\nand contains the value currently used by the service."]
    pub fn effective_reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_reserved_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_stripe_level` after provisioning.\nStripe level for files.\nMIN better suited for small size files.\nMAX higher throughput performance for larger files.\n  Possible values:\n  FILE_STRIPE_LEVEL_UNSPECIFIED\n  FILE_STRIPE_LEVEL_MIN\n  FILE_STRIPE_LEVEL_BALANCED\n  FILE_STRIPE_LEVEL_MAX"]
    pub fn file_stripe_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_stripe_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe logical name of the Parallelstore instance in the user project with the following restrictions:\n  * Must contain only lowercase letters, numbers, and hyphens.\n  * Must start with a letter.\n  * Must be between 1-63 characters.\n  * Must end with a number or a letter.\n  * Must be unique within the customer project/ location"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nCloud Labels are a flexible and lightweight mechanism for\norganizing cloud resources into groups that reflect a customer's organizational\nneeds and deployment strategies. Cloud Labels can be used to filter collections\nof resources. They can be used to control how resource metrics are aggregated.\nAnd they can be used as arguments to policy management rules (e.g. route, firewall,\nload balancing, etc.).\n\n* Label keys must be between 1 and 63 characters long and must conform to\n the following regular expression: 'a-z{0,62}'.\n* Label values must be between 0 and 63 characters long and must conform\n to the regular expression '[a-z0-9_-]{0,63}'.\n* No more than 64 labels can be associated with a given resource.\n\nSee https://goo.gl/xmQnxf for more information on and examples of labels.\n\nIf you plan to use labels in your own code, please note that additional\ncharacters may be allowed in the future. Therefore, you are advised to use\nan internal label representation, such as JSON, which doesn't rely upon\nspecific characters being disallowed.  For example, representing labels\nas the string:  'name + \"_\" + value' would prove problematic if we were to\nallow '\"_\"' in a future release. \"\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the instance, in the format\n'projects/{project}/locations/{location}/instances/{instance_id}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nImmutable. The name of the Google Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc)\nto which the instance is connected."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reserved_ip_range` after provisioning.\nImmutable. Contains the id of the allocated IP address range\nassociated with the private service access connection for example, \\\"test-default\\\"\nassociated with IP range 10.0.0.0/29. If no range id is provided all ranges will\nbe considered."]
    pub fn reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe instance state.\n  Possible values:\n  STATE_UNSPECIFIED\n  CREATING\n  ACTIVE\n  DELETING\n  FAILED\n  UPGRADING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the instance was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ParallelstoreInstanceTimeoutsElRef {
        ParallelstoreInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ParallelstoreInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ParallelstoreInstance {}
impl ToListMappable for ParallelstoreInstance {
    type O = ListRef<ParallelstoreInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ParallelstoreInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_parallelstore_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildParallelstoreInstance {
    pub tf_id: String,
    #[doc = "Required. Immutable. Storage capacity of Parallelstore instance in Gibibytes (GiB)."]
    pub capacity_gib: PrimField<String>,
    #[doc = "The logical name of the Parallelstore instance in the user project with the following restrictions:\n  * Must contain only lowercase letters, numbers, and hyphens.\n  * Must start with a letter.\n  * Must be between 1-63 characters.\n  * Must end with a number or a letter.\n  * Must be unique within the customer project/ location"]
    pub instance_id: PrimField<String>,
    #[doc = "Part of 'parent'. See documentation of 'projectsId'."]
    pub location: PrimField<String>,
}
impl BuildParallelstoreInstance {
    pub fn build(self, stack: &mut Stack) -> ParallelstoreInstance {
        let out = ParallelstoreInstance(Rc::new(ParallelstoreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ParallelstoreInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                capacity_gib: self.capacity_gib,
                deletion_policy: core::default::Default::default(),
                deployment_type: core::default::Default::default(),
                description: core::default::Default::default(),
                directory_stripe_level: core::default::Default::default(),
                file_stripe_level: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                labels: core::default::Default::default(),
                location: self.location,
                network: core::default::Default::default(),
                project: core::default::Default::default(),
                reserved_ip_range: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ParallelstoreInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for ParallelstoreInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ParallelstoreInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_points` after provisioning.\nOutput only. List of access_points.\nContains a list of IPv4 addresses used for client side configuration."]
    pub fn access_points(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_points", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_gib` after provisioning.\nRequired. Immutable. Storage capacity of Parallelstore instance in Gibibytes (GiB)."]
    pub fn capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `daos_version` after provisioning.\nThe version of DAOS software running in the instance."]
    pub fn daos_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.daos_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\nParallelstore Instance deployment type.\n  Possible values:\n  DEPLOYMENT_TYPE_UNSPECIFIED\n  SCRATCH\n  PERSISTENT"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the instance. 2048 characters or less."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_stripe_level` after provisioning.\nStripe level for directories.\nMIN when directory has a small number of files.\nMAX when directory has a large number of files.\n  Possible values:\n  DIRECTORY_STRIPE_LEVEL_UNSPECIFIED\n  DIRECTORY_STRIPE_LEVEL_MIN\n  DIRECTORY_STRIPE_LEVEL_BALANCED\n  DIRECTORY_STRIPE_LEVEL_MAX"]
    pub fn directory_stripe_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_stripe_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_reserved_ip_range` after provisioning.\nImmutable. Contains the id of the allocated IP address\nrange associated with the private service access connection for example, \\\"test-default\\\"\nassociated with IP range 10.0.0.0/29. This field is populated by the service\nand contains the value currently used by the service."]
    pub fn effective_reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_reserved_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_stripe_level` after provisioning.\nStripe level for files.\nMIN better suited for small size files.\nMAX higher throughput performance for larger files.\n  Possible values:\n  FILE_STRIPE_LEVEL_UNSPECIFIED\n  FILE_STRIPE_LEVEL_MIN\n  FILE_STRIPE_LEVEL_BALANCED\n  FILE_STRIPE_LEVEL_MAX"]
    pub fn file_stripe_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_stripe_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe logical name of the Parallelstore instance in the user project with the following restrictions:\n  * Must contain only lowercase letters, numbers, and hyphens.\n  * Must start with a letter.\n  * Must be between 1-63 characters.\n  * Must end with a number or a letter.\n  * Must be unique within the customer project/ location"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nCloud Labels are a flexible and lightweight mechanism for\norganizing cloud resources into groups that reflect a customer's organizational\nneeds and deployment strategies. Cloud Labels can be used to filter collections\nof resources. They can be used to control how resource metrics are aggregated.\nAnd they can be used as arguments to policy management rules (e.g. route, firewall,\nload balancing, etc.).\n\n* Label keys must be between 1 and 63 characters long and must conform to\n the following regular expression: 'a-z{0,62}'.\n* Label values must be between 0 and 63 characters long and must conform\n to the regular expression '[a-z0-9_-]{0,63}'.\n* No more than 64 labels can be associated with a given resource.\n\nSee https://goo.gl/xmQnxf for more information on and examples of labels.\n\nIf you plan to use labels in your own code, please note that additional\ncharacters may be allowed in the future. Therefore, you are advised to use\nan internal label representation, such as JSON, which doesn't rely upon\nspecific characters being disallowed.  For example, representing labels\nas the string:  'name + \"_\" + value' would prove problematic if we were to\nallow '\"_\"' in a future release. \"\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the instance, in the format\n'projects/{project}/locations/{location}/instances/{instance_id}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nImmutable. The name of the Google Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc)\nto which the instance is connected."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reserved_ip_range` after provisioning.\nImmutable. Contains the id of the allocated IP address range\nassociated with the private service access connection for example, \\\"test-default\\\"\nassociated with IP range 10.0.0.0/29. If no range id is provided all ranges will\nbe considered."]
    pub fn reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe instance state.\n  Possible values:\n  STATE_UNSPECIFIED\n  CREATING\n  ACTIVE\n  DELETING\n  FAILED\n  UPGRADING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the instance was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ParallelstoreInstanceTimeoutsElRef {
        ParallelstoreInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ParallelstoreInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ParallelstoreInstanceTimeoutsEl {
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
impl ToListMappable for ParallelstoreInstanceTimeoutsEl {
    type O = BlockAssignable<ParallelstoreInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildParallelstoreInstanceTimeoutsEl {}
impl BuildParallelstoreInstanceTimeoutsEl {
    pub fn build(self) -> ParallelstoreInstanceTimeoutsEl {
        ParallelstoreInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ParallelstoreInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ParallelstoreInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ParallelstoreInstanceTimeoutsElRef {
        ParallelstoreInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ParallelstoreInstanceTimeoutsElRef {
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
