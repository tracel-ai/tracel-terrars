use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VpcAccessConnectorData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_cidr_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_throughput: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet: Option<Vec<VpcAccessConnectorSubnetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpcAccessConnectorTimeoutsEl>,
    dynamic: VpcAccessConnectorDynamic,
}
struct VpcAccessConnector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcAccessConnectorData>,
}
#[derive(Clone)]
pub struct VpcAccessConnector(Rc<VpcAccessConnector_>);
impl VpcAccessConnector {
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
    #[doc = "Set the field `ip_cidr_range`.\nThe range of internal addresses that follows RFC 4632 notation. Example: '10.132.0.0/28'."]
    pub fn set_ip_cidr_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_cidr_range = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nMachine type of VM Instance underlying connector. Default is e2-micro"]
    pub fn set_machine_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instances`.\nMaximum value of instances in autoscaling group underlying the connector. Value must be between 3 and 10, inclusive. Must be\nhigher than the value specified by min_instances. Required alongside 'min_instances' if not using 'min_throughput'/'max_throughput'."]
    pub fn set_max_instances(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `max_throughput`.\nMaximum throughput of the connector in Mbps, must be greater than 'min_throughput'. Default is 300. Refers to the expected throughput\nwhen using an e2-micro machine type. Value must be a multiple of 100 from 300 through 1000. Must be higher than the value specified by\nmin_throughput. Only one of 'max_throughput' and 'max_instances' can be specified. The use of max_throughput is discouraged in favor of max_instances."]
    pub fn set_max_throughput(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instances`.\nMinimum value of instances in autoscaling group underlying the connector. Value must be between 2 and 9, inclusive. Must be\nlower than the value specified by max_instances. Required alongside 'max_instances' if not using 'min_throughput'/'max_throughput'."]
    pub fn set_min_instances(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `min_throughput`.\nMinimum throughput of the connector in Mbps. Default and min is 200. Refers to the expected throughput when using an e2-micro machine type.\nValue must be a multiple of 100 from 200 through 900. Must be lower than the value specified by max_throughput.\nOnly one of 'min_throughput' and 'min_instances' can be specified. The use of min_throughput is discouraged in favor of min_instances."]
    pub fn set_min_throughput(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nName or self_link of the VPC network. Required if 'ip_cidr_range' is set."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where the VPC Access connector resides. If it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(self, v: impl Into<BlockAssignable<VpcAccessConnectorSubnetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subnet = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subnet = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpcAccessConnectorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `connected_projects` after provisioning.\nList of projects using the connector."]
    pub fn connected_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connected_projects", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe range of internal addresses that follows RFC 4632 notation. Example: '10.132.0.0/28'."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nMachine type of VM Instance underlying connector. Default is e2-micro"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.machine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_instances` after provisioning.\nMaximum value of instances in autoscaling group underlying the connector. Value must be between 3 and 10, inclusive. Must be\nhigher than the value specified by min_instances. Required alongside 'min_instances' if not using 'min_throughput'/'max_throughput'."]
    pub fn max_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_throughput` after provisioning.\nMaximum throughput of the connector in Mbps, must be greater than 'min_throughput'. Default is 300. Refers to the expected throughput\nwhen using an e2-micro machine type. Value must be a multiple of 100 from 300 through 1000. Must be higher than the value specified by\nmin_throughput. Only one of 'max_throughput' and 'max_instances' can be specified. The use of max_throughput is discouraged in favor of max_instances."]
    pub fn max_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_instances` after provisioning.\nMinimum value of instances in autoscaling group underlying the connector. Value must be between 2 and 9, inclusive. Must be\nlower than the value specified by max_instances. Required alongside 'max_instances' if not using 'min_throughput'/'max_throughput'."]
    pub fn min_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_throughput` after provisioning.\nMinimum throughput of the connector in Mbps. Default and min is 200. Refers to the expected throughput when using an e2-micro machine type.\nValue must be a multiple of 100 from 200 through 900. Must be lower than the value specified by max_throughput.\nOnly one of 'min_throughput' and 'min_instances' can be specified. The use of min_throughput is discouraged in favor of min_instances."]
    pub fn min_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the resource (Max 25 characters)."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nName or self_link of the VPC network. Required if 'ip_cidr_range' is set."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the VPC Access connector resides. If it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this VPC connector"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the VPC access connector."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> ListRef<VpcAccessConnectorSubnetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcAccessConnectorTimeoutsElRef {
        VpcAccessConnectorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VpcAccessConnector {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VpcAccessConnector {}
impl ToListMappable for VpcAccessConnector {
    type O = ListRef<VpcAccessConnectorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VpcAccessConnector_ {
    fn extract_resource_type(&self) -> String {
        "google_vpc_access_connector".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVpcAccessConnector {
    pub tf_id: String,
    #[doc = "The name of the resource (Max 25 characters)."]
    pub name: PrimField<String>,
}
impl BuildVpcAccessConnector {
    pub fn build(self, stack: &mut Stack) -> VpcAccessConnector {
        let out = VpcAccessConnector(Rc::new(VpcAccessConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcAccessConnectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_cidr_range: core::default::Default::default(),
                machine_type: core::default::Default::default(),
                max_instances: core::default::Default::default(),
                max_throughput: core::default::Default::default(),
                min_instances: core::default::Default::default(),
                min_throughput: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                subnet: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VpcAccessConnectorRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcAccessConnectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VpcAccessConnectorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connected_projects` after provisioning.\nList of projects using the connector."]
    pub fn connected_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connected_projects", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe range of internal addresses that follows RFC 4632 notation. Example: '10.132.0.0/28'."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nMachine type of VM Instance underlying connector. Default is e2-micro"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.machine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_instances` after provisioning.\nMaximum value of instances in autoscaling group underlying the connector. Value must be between 3 and 10, inclusive. Must be\nhigher than the value specified by min_instances. Required alongside 'min_instances' if not using 'min_throughput'/'max_throughput'."]
    pub fn max_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_throughput` after provisioning.\nMaximum throughput of the connector in Mbps, must be greater than 'min_throughput'. Default is 300. Refers to the expected throughput\nwhen using an e2-micro machine type. Value must be a multiple of 100 from 300 through 1000. Must be higher than the value specified by\nmin_throughput. Only one of 'max_throughput' and 'max_instances' can be specified. The use of max_throughput is discouraged in favor of max_instances."]
    pub fn max_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_instances` after provisioning.\nMinimum value of instances in autoscaling group underlying the connector. Value must be between 2 and 9, inclusive. Must be\nlower than the value specified by max_instances. Required alongside 'max_instances' if not using 'min_throughput'/'max_throughput'."]
    pub fn min_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_throughput` after provisioning.\nMinimum throughput of the connector in Mbps. Default and min is 200. Refers to the expected throughput when using an e2-micro machine type.\nValue must be a multiple of 100 from 200 through 900. Must be lower than the value specified by max_throughput.\nOnly one of 'min_throughput' and 'min_instances' can be specified. The use of min_throughput is discouraged in favor of min_instances."]
    pub fn min_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the resource (Max 25 characters)."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nName or self_link of the VPC network. Required if 'ip_cidr_range' is set."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the VPC Access connector resides. If it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this VPC connector"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the VPC access connector."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> ListRef<VpcAccessConnectorSubnetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcAccessConnectorTimeoutsElRef {
        VpcAccessConnectorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VpcAccessConnectorSubnetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}
impl VpcAccessConnectorSubnetEl {
    #[doc = "Set the field `name`.\nSubnet name (relative, not fully qualified). E.g. if the full subnet selfLink is\nhttps://compute.googleapis.com/compute/v1/projects/{project}/regions/{region}/subnetworks/{subnetName} the correct input for this field would be {subnetName}\""]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nProject in which the subnet exists. If not set, this project is assumed to be the project for which the connector create request was issued."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for VpcAccessConnectorSubnetEl {
    type O = BlockAssignable<VpcAccessConnectorSubnetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpcAccessConnectorSubnetEl {}
impl BuildVpcAccessConnectorSubnetEl {
    pub fn build(self) -> VpcAccessConnectorSubnetEl {
        VpcAccessConnectorSubnetEl {
            name: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}
pub struct VpcAccessConnectorSubnetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcAccessConnectorSubnetElRef {
    fn new(shared: StackShared, base: String) -> VpcAccessConnectorSubnetElRef {
        VpcAccessConnectorSubnetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpcAccessConnectorSubnetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nSubnet name (relative, not fully qualified). E.g. if the full subnet selfLink is\nhttps://compute.googleapis.com/compute/v1/projects/{project}/regions/{region}/subnetworks/{subnetName} the correct input for this field would be {subnetName}\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject in which the subnet exists. If not set, this project is assumed to be the project for which the connector create request was issued."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct VpcAccessConnectorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VpcAccessConnectorTimeoutsEl {
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
impl ToListMappable for VpcAccessConnectorTimeoutsEl {
    type O = BlockAssignable<VpcAccessConnectorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpcAccessConnectorTimeoutsEl {}
impl BuildVpcAccessConnectorTimeoutsEl {
    pub fn build(self) -> VpcAccessConnectorTimeoutsEl {
        VpcAccessConnectorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VpcAccessConnectorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcAccessConnectorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpcAccessConnectorTimeoutsElRef {
        VpcAccessConnectorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpcAccessConnectorTimeoutsElRef {
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
struct VpcAccessConnectorDynamic {
    subnet: Option<DynamicBlock<VpcAccessConnectorSubnetEl>>,
}
