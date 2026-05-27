use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkConnectivityInternalRangeData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_cidr_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    immutable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_cidr_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overlaps: Option<ListField<PrimField<String>>>,
    peering: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_cidr_range: Option<ListField<PrimField<String>>>,
    usage: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_options: Option<Vec<NetworkConnectivityInternalRangeAllocationOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    migration: Option<Vec<NetworkConnectivityInternalRangeMigrationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkConnectivityInternalRangeTimeoutsEl>,
    dynamic: NetworkConnectivityInternalRangeDynamic,
}
struct NetworkConnectivityInternalRange_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkConnectivityInternalRangeData>,
}
#[derive(Clone)]
pub struct NetworkConnectivityInternalRange(Rc<NetworkConnectivityInternalRange_>);
impl NetworkConnectivityInternalRange {
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
    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_cidr_ranges`.\nOptional. List of IP CIDR ranges to be excluded. Resulting reserved Internal Range will not overlap with any CIDR blocks mentioned in this list.\nOnly IPv4 CIDR ranges are supported."]
    pub fn set_exclude_cidr_ranges(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().exclude_cidr_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `immutable`.\nImmutable ranges cannot have their fields modified, except for labels and description."]
    pub fn set_immutable(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().immutable = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_cidr_range`.\nThe IP range that this internal range defines.\nNOTE: IPv6 ranges are limited to usage=EXTERNAL_TO_VPC and peering=FOR_SELF\nNOTE: For IPv6 Ranges this field is compulsory, i.e. the address range must be specified explicitly."]
    pub fn set_ip_cidr_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_cidr_range = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `overlaps`.\nOptional. Types of resources that are allowed to overlap with the current internal range. Possible values: [\"OVERLAP_ROUTE_RANGE\", \"OVERLAP_EXISTING_SUBNET_RANGE\"]"]
    pub fn set_overlaps(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().overlaps = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_length`.\nAn alternate to ipCidrRange. Can be set when trying to create a reservation that automatically finds a free range of the given size.\nIf both ipCidrRange and prefixLength are set, there is an error if the range sizes do not match. Can also be used during updates to change the range size.\nNOTE: For IPv6 this field only works if ip_cidr_range is set as well, and both fields must match. In other words, with IPv6 this field only works as\na redundant parameter."]
    pub fn set_prefix_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().prefix_length = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `target_cidr_range`.\nOptional. Can be set to narrow down or pick a different address space while searching for a free range.\nIf not set, defaults to the \"10.0.0.0/8\" address space. This can be used to search in other rfc-1918 address spaces like \"172.16.0.0/12\" and \"192.168.0.0/16\" or non-rfc-1918 address spaces used in the VPC."]
    pub fn set_target_cidr_range(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().target_cidr_range = Some(v.into());
        self
    }
    #[doc = "Set the field `allocation_options`.\n"]
    pub fn set_allocation_options(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivityInternalRangeAllocationOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allocation_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allocation_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `migration`.\n"]
    pub fn set_migration(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivityInternalRangeMigrationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().migration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.migration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkConnectivityInternalRangeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_cidr_ranges` after provisioning.\nOptional. List of IP CIDR ranges to be excluded. Resulting reserved Internal Range will not overlap with any CIDR blocks mentioned in this list.\nOnly IPv4 CIDR ranges are supported."]
    pub fn exclude_cidr_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cidr_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `immutable` after provisioning.\nImmutable ranges cannot have their fields modified, except for labels and description."]
    pub fn immutable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.immutable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe IP range that this internal range defines.\nNOTE: IPv6 ranges are limited to usage=EXTERNAL_TO_VPC and peering=FOR_SELF\nNOTE: For IPv6 Ranges this field is compulsory, i.e. the address range must be specified explicitly."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy based route."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nFully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `overlaps` after provisioning.\nOptional. Types of resources that are allowed to overlap with the current internal range. Possible values: [\"OVERLAP_ROUTE_RANGE\", \"OVERLAP_EXISTING_SUBNET_RANGE\"]"]
    pub fn overlaps(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.overlaps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peering` after provisioning.\nThe type of peering set for this internal range. Possible values: [\"FOR_SELF\", \"FOR_PEER\", \"NOT_SHARED\"]"]
    pub fn peering(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_length` after provisioning.\nAn alternate to ipCidrRange. Can be set when trying to create a reservation that automatically finds a free range of the given size.\nIf both ipCidrRange and prefixLength are set, there is an error if the range sizes do not match. Can also be used during updates to change the range size.\nNOTE: For IPv6 this field only works if ip_cidr_range is set as well, and both fields must match. In other words, with IPv6 this field only works as\na redundant parameter."]
    pub fn prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_cidr_range` after provisioning.\nOptional. Can be set to narrow down or pick a different address space while searching for a free range.\nIf not set, defaults to the \"10.0.0.0/8\" address space. This can be used to search in other rfc-1918 address spaces like \"172.16.0.0/12\" and \"192.168.0.0/16\" or non-rfc-1918 address spaces used in the VPC."]
    pub fn target_cidr_range(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `usage` after provisioning.\nThe type of usage set for this InternalRange. Possible values: [\"FOR_VPC\", \"EXTERNAL_TO_VPC\", \"FOR_MIGRATION\"]"]
    pub fn usage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.usage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `users` after provisioning.\nOutput only. The list of resources that refer to this internal range.\nResources that use the internal range for their range allocation are referred to as users of the range.\nOther resources mark themselves as users while doing so by creating a reference to this internal range. Having a user, based on this reference, prevents deletion of the internal range referred to. Can be empty."]
    pub fn users(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allocation_options` after provisioning.\n"]
    pub fn allocation_options(
        &self,
    ) -> ListRef<NetworkConnectivityInternalRangeAllocationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allocation_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `migration` after provisioning.\n"]
    pub fn migration(&self) -> ListRef<NetworkConnectivityInternalRangeMigrationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.migration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityInternalRangeTimeoutsElRef {
        NetworkConnectivityInternalRangeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkConnectivityInternalRange {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkConnectivityInternalRange {}
impl ToListMappable for NetworkConnectivityInternalRange {
    type O = ListRef<NetworkConnectivityInternalRangeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkConnectivityInternalRange_ {
    fn extract_resource_type(&self) -> String {
        "google_network_connectivity_internal_range".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkConnectivityInternalRange {
    pub tf_id: String,
    #[doc = "The name of the policy based route."]
    pub name: PrimField<String>,
    #[doc = "Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network."]
    pub network: PrimField<String>,
    #[doc = "The type of peering set for this internal range. Possible values: [\"FOR_SELF\", \"FOR_PEER\", \"NOT_SHARED\"]"]
    pub peering: PrimField<String>,
    #[doc = "The type of usage set for this InternalRange. Possible values: [\"FOR_VPC\", \"EXTERNAL_TO_VPC\", \"FOR_MIGRATION\"]"]
    pub usage: PrimField<String>,
}
impl BuildNetworkConnectivityInternalRange {
    pub fn build(self, stack: &mut Stack) -> NetworkConnectivityInternalRange {
        let out = NetworkConnectivityInternalRange(Rc::new(NetworkConnectivityInternalRange_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkConnectivityInternalRangeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                exclude_cidr_ranges: core::default::Default::default(),
                id: core::default::Default::default(),
                immutable: core::default::Default::default(),
                ip_cidr_range: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                network: self.network,
                overlaps: core::default::Default::default(),
                peering: self.peering,
                prefix_length: core::default::Default::default(),
                project: core::default::Default::default(),
                target_cidr_range: core::default::Default::default(),
                usage: self.usage,
                allocation_options: core::default::Default::default(),
                migration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkConnectivityInternalRangeRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityInternalRangeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkConnectivityInternalRangeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_cidr_ranges` after provisioning.\nOptional. List of IP CIDR ranges to be excluded. Resulting reserved Internal Range will not overlap with any CIDR blocks mentioned in this list.\nOnly IPv4 CIDR ranges are supported."]
    pub fn exclude_cidr_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_cidr_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `immutable` after provisioning.\nImmutable ranges cannot have their fields modified, except for labels and description."]
    pub fn immutable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.immutable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe IP range that this internal range defines.\nNOTE: IPv6 ranges are limited to usage=EXTERNAL_TO_VPC and peering=FOR_SELF\nNOTE: For IPv6 Ranges this field is compulsory, i.e. the address range must be specified explicitly."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy based route."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nFully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `overlaps` after provisioning.\nOptional. Types of resources that are allowed to overlap with the current internal range. Possible values: [\"OVERLAP_ROUTE_RANGE\", \"OVERLAP_EXISTING_SUBNET_RANGE\"]"]
    pub fn overlaps(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.overlaps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peering` after provisioning.\nThe type of peering set for this internal range. Possible values: [\"FOR_SELF\", \"FOR_PEER\", \"NOT_SHARED\"]"]
    pub fn peering(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_length` after provisioning.\nAn alternate to ipCidrRange. Can be set when trying to create a reservation that automatically finds a free range of the given size.\nIf both ipCidrRange and prefixLength are set, there is an error if the range sizes do not match. Can also be used during updates to change the range size.\nNOTE: For IPv6 this field only works if ip_cidr_range is set as well, and both fields must match. In other words, with IPv6 this field only works as\na redundant parameter."]
    pub fn prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_cidr_range` after provisioning.\nOptional. Can be set to narrow down or pick a different address space while searching for a free range.\nIf not set, defaults to the \"10.0.0.0/8\" address space. This can be used to search in other rfc-1918 address spaces like \"172.16.0.0/12\" and \"192.168.0.0/16\" or non-rfc-1918 address spaces used in the VPC."]
    pub fn target_cidr_range(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `usage` after provisioning.\nThe type of usage set for this InternalRange. Possible values: [\"FOR_VPC\", \"EXTERNAL_TO_VPC\", \"FOR_MIGRATION\"]"]
    pub fn usage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.usage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `users` after provisioning.\nOutput only. The list of resources that refer to this internal range.\nResources that use the internal range for their range allocation are referred to as users of the range.\nOther resources mark themselves as users while doing so by creating a reference to this internal range. Having a user, based on this reference, prevents deletion of the internal range referred to. Can be empty."]
    pub fn users(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allocation_options` after provisioning.\n"]
    pub fn allocation_options(
        &self,
    ) -> ListRef<NetworkConnectivityInternalRangeAllocationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allocation_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `migration` after provisioning.\n"]
    pub fn migration(&self) -> ListRef<NetworkConnectivityInternalRangeMigrationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.migration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityInternalRangeTimeoutsElRef {
        NetworkConnectivityInternalRangeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityInternalRangeAllocationOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_available_ranges_lookup_size: Option<PrimField<f64>>,
}
impl NetworkConnectivityInternalRangeAllocationOptionsEl {
    #[doc = "Set the field `allocation_strategy`.\nOptional. Sets the strategy used to automatically find a free range of a size given by prefixLength. Can be set only when trying to create a reservation that automatically finds the free range to reserve. Possible values: [\"RANDOM\", \"FIRST_AVAILABLE\", \"RANDOM_FIRST_N_AVAILABLE\", \"FIRST_SMALLEST_FITTING\"]"]
    pub fn set_allocation_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocation_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `first_available_ranges_lookup_size`.\nMust be set when allocation_strategy is RANDOM_FIRST_N_AVAILABLE, otherwise must remain unset. Defines the size of the set of free ranges from which RANDOM_FIRST_N_AVAILABLE strategy randomy selects one,\nin other words it sets the N in the RANDOM_FIRST_N_AVAILABLE."]
    pub fn set_first_available_ranges_lookup_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.first_available_ranges_lookup_size = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivityInternalRangeAllocationOptionsEl {
    type O = BlockAssignable<NetworkConnectivityInternalRangeAllocationOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityInternalRangeAllocationOptionsEl {}
impl BuildNetworkConnectivityInternalRangeAllocationOptionsEl {
    pub fn build(self) -> NetworkConnectivityInternalRangeAllocationOptionsEl {
        NetworkConnectivityInternalRangeAllocationOptionsEl {
            allocation_strategy: core::default::Default::default(),
            first_available_ranges_lookup_size: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityInternalRangeAllocationOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityInternalRangeAllocationOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivityInternalRangeAllocationOptionsElRef {
        NetworkConnectivityInternalRangeAllocationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityInternalRangeAllocationOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\nOptional. Sets the strategy used to automatically find a free range of a size given by prefixLength. Can be set only when trying to create a reservation that automatically finds the free range to reserve. Possible values: [\"RANDOM\", \"FIRST_AVAILABLE\", \"RANDOM_FIRST_N_AVAILABLE\", \"FIRST_SMALLEST_FITTING\"]"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `first_available_ranges_lookup_size` after provisioning.\nMust be set when allocation_strategy is RANDOM_FIRST_N_AVAILABLE, otherwise must remain unset. Defines the size of the set of free ranges from which RANDOM_FIRST_N_AVAILABLE strategy randomy selects one,\nin other words it sets the N in the RANDOM_FIRST_N_AVAILABLE."]
    pub fn first_available_ranges_lookup_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.first_available_ranges_lookup_size", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityInternalRangeMigrationEl {
    source: PrimField<String>,
    target: PrimField<String>,
}
impl NetworkConnectivityInternalRangeMigrationEl {}
impl ToListMappable for NetworkConnectivityInternalRangeMigrationEl {
    type O = BlockAssignable<NetworkConnectivityInternalRangeMigrationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityInternalRangeMigrationEl {
    #[doc = "Resource path as an URI of the source resource, for example a subnet.\nThe project for the source resource should match the project for the\nInternalRange.\nAn example /projects/{project}/regions/{region}/subnetworks/{subnet}"]
    pub source: PrimField<String>,
    #[doc = "Resource path of the target resource. The target project can be\ndifferent, as in the cases when migrating to peer networks. The resource\nmay not exist yet.\nFor example /projects/{project}/regions/{region}/subnetworks/{subnet}"]
    pub target: PrimField<String>,
}
impl BuildNetworkConnectivityInternalRangeMigrationEl {
    pub fn build(self) -> NetworkConnectivityInternalRangeMigrationEl {
        NetworkConnectivityInternalRangeMigrationEl {
            source: self.source,
            target: self.target,
        }
    }
}
pub struct NetworkConnectivityInternalRangeMigrationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityInternalRangeMigrationElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivityInternalRangeMigrationElRef {
        NetworkConnectivityInternalRangeMigrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityInternalRangeMigrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\nResource path as an URI of the source resource, for example a subnet.\nThe project for the source resource should match the project for the\nInternalRange.\nAn example /projects/{project}/regions/{region}/subnetworks/{subnet}"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nResource path of the target resource. The target project can be\ndifferent, as in the cases when migrating to peer networks. The resource\nmay not exist yet.\nFor example /projects/{project}/regions/{region}/subnetworks/{subnet}"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityInternalRangeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkConnectivityInternalRangeTimeoutsEl {
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
impl ToListMappable for NetworkConnectivityInternalRangeTimeoutsEl {
    type O = BlockAssignable<NetworkConnectivityInternalRangeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityInternalRangeTimeoutsEl {}
impl BuildNetworkConnectivityInternalRangeTimeoutsEl {
    pub fn build(self) -> NetworkConnectivityInternalRangeTimeoutsEl {
        NetworkConnectivityInternalRangeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityInternalRangeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityInternalRangeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivityInternalRangeTimeoutsElRef {
        NetworkConnectivityInternalRangeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityInternalRangeTimeoutsElRef {
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
struct NetworkConnectivityInternalRangeDynamic {
    allocation_options: Option<DynamicBlock<NetworkConnectivityInternalRangeAllocationOptionsEl>>,
    migration: Option<DynamicBlock<NetworkConnectivityInternalRangeMigrationEl>>,
}
