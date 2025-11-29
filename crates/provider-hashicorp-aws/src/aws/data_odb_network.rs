use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbNetworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataOdbNetwork_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbNetworkData>,
}

#[derive(Clone)]
pub struct DataOdbNetwork(Rc<DataOdbNetwork_>);

impl DataOdbNetwork {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone` after provisioning.\nThe availability zone where the ODB network is located."]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\nThe AZ ID of the AZ where the ODB network is located."]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_subnet_cidr` after provisioning.\n The CIDR range of the backup subnet for the ODB network."]
    pub fn backup_subnet_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_subnet_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_subnet_cidr` after provisioning.\nThe CIDR notation for the network resource."]
    pub fn client_subnet_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_subnet_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the ODB network was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_domain_name` after provisioning.\nThe name of the custom domain that the network is located."]
    pub fn custom_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_dns_prefix` after provisioning.\nThe default DNS prefix for the network resource."]
    pub fn default_dns_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_dns_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for the network resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_services` after provisioning.\nThe managed services configuration for the ODB network."]
    pub fn managed_services(&self) -> ListRef<DataOdbNetworkManagedServicesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.managed_services", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_dns_forwarding_configs` after provisioning.\nThe DNS resolver endpoint in OCI for forwarding DNS queries for the ociPrivateZone domain."]
    pub fn oci_dns_forwarding_configs(&self) -> ListRef<DataOdbNetworkOciDnsForwardingConfigsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oci_dns_forwarding_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_network_anchor_id` after provisioning.\nThe unique identifier of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_network_anchor_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_network_anchor_url` after provisioning.\nThe URL of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_network_anchor_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the ODB network."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_vcn_id` after provisioning.\nThe unique identifier  Oracle Cloud ID (OCID) of the OCI VCN for the ODB network."]
    pub fn oci_vcn_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_vcn_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_vcn_url` after provisioning.\nThe URL of the OCI VCN for the ODB network."]
    pub fn oci_vcn_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_vcn_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peered_cidrs` after provisioning.\nThe list of CIDR ranges from the peered VPC that are allowed access to the ODB network. Please refer odb network peering documentation."]
    pub fn peered_cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.peered_cidrs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the ODB network, expressed as a percentage."]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the network resource."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the ODB network."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataOdbNetwork {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOdbNetwork { }

impl ToListMappable for DataOdbNetwork {
    type O = ListRef<DataOdbNetworkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOdbNetwork_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_network".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbNetwork {
    pub tf_id: String,
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildDataOdbNetwork {
    pub fn build(self, stack: &mut Stack) -> DataOdbNetwork {
        let out = DataOdbNetwork(Rc::new(DataOdbNetwork_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbNetworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbNetworkRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbNetworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataOdbNetworkRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone` after provisioning.\nThe availability zone where the ODB network is located."]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\nThe AZ ID of the AZ where the ODB network is located."]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_subnet_cidr` after provisioning.\n The CIDR range of the backup subnet for the ODB network."]
    pub fn backup_subnet_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_subnet_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_subnet_cidr` after provisioning.\nThe CIDR notation for the network resource."]
    pub fn client_subnet_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_subnet_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the ODB network was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_domain_name` after provisioning.\nThe name of the custom domain that the network is located."]
    pub fn custom_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_dns_prefix` after provisioning.\nThe default DNS prefix for the network resource."]
    pub fn default_dns_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_dns_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for the network resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_services` after provisioning.\nThe managed services configuration for the ODB network."]
    pub fn managed_services(&self) -> ListRef<DataOdbNetworkManagedServicesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.managed_services", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_dns_forwarding_configs` after provisioning.\nThe DNS resolver endpoint in OCI for forwarding DNS queries for the ociPrivateZone domain."]
    pub fn oci_dns_forwarding_configs(&self) -> ListRef<DataOdbNetworkOciDnsForwardingConfigsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oci_dns_forwarding_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_network_anchor_id` after provisioning.\nThe unique identifier of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_network_anchor_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_network_anchor_url` after provisioning.\nThe URL of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_network_anchor_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the ODB network."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_vcn_id` after provisioning.\nThe unique identifier  Oracle Cloud ID (OCID) of the OCI VCN for the ODB network."]
    pub fn oci_vcn_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_vcn_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_vcn_url` after provisioning.\nThe URL of the OCI VCN for the ODB network."]
    pub fn oci_vcn_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_vcn_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peered_cidrs` after provisioning.\nThe list of CIDR ranges from the peered VPC that are allowed access to the ODB network. Please refer odb network peering documentation."]
    pub fn peered_cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.peered_cidrs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the ODB network, expressed as a percentage."]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the network resource."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the ODB network."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetworkManagedServicesElManagedS3BackupAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataOdbNetworkManagedServicesElManagedS3BackupAccessEl {
    #[doc = "Set the field `ipv4_addresses`.\n"]
    pub fn set_ipv4_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ipv4_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbNetworkManagedServicesElManagedS3BackupAccessEl {
    type O = BlockAssignable<DataOdbNetworkManagedServicesElManagedS3BackupAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetworkManagedServicesElManagedS3BackupAccessEl {}

impl BuildDataOdbNetworkManagedServicesElManagedS3BackupAccessEl {
    pub fn build(self) -> DataOdbNetworkManagedServicesElManagedS3BackupAccessEl {
        DataOdbNetworkManagedServicesElManagedS3BackupAccessEl {
            ipv4_addresses: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataOdbNetworkManagedServicesElManagedS3BackupAccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbNetworkManagedServicesElManagedS3BackupAccessElRef {
    fn new(shared: StackShared, base: String) -> DataOdbNetworkManagedServicesElManagedS3BackupAccessElRef {
        DataOdbNetworkManagedServicesElManagedS3BackupAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetworkManagedServicesElManagedS3BackupAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\n"]
    pub fn ipv4_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv4_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetworkManagedServicesElS3AccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_policy_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataOdbNetworkManagedServicesElS3AccessEl {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_addresses`.\n"]
    pub fn set_ipv4_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ipv4_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_policy_document`.\n"]
    pub fn set_s3_policy_document(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_policy_document = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbNetworkManagedServicesElS3AccessEl {
    type O = BlockAssignable<DataOdbNetworkManagedServicesElS3AccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetworkManagedServicesElS3AccessEl {}

impl BuildDataOdbNetworkManagedServicesElS3AccessEl {
    pub fn build(self) -> DataOdbNetworkManagedServicesElS3AccessEl {
        DataOdbNetworkManagedServicesElS3AccessEl {
            domain_name: core::default::Default::default(),
            ipv4_addresses: core::default::Default::default(),
            s3_policy_document: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataOdbNetworkManagedServicesElS3AccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbNetworkManagedServicesElS3AccessElRef {
    fn new(shared: StackShared, base: String) -> DataOdbNetworkManagedServicesElS3AccessElRef {
        DataOdbNetworkManagedServicesElS3AccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetworkManagedServicesElS3AccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\n"]
    pub fn ipv4_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv4_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_policy_document` after provisioning.\n"]
    pub fn s3_policy_document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_policy_document", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetworkManagedServicesElServiceNetworkEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_type: Option<PrimField<String>>,
}

impl DataOdbNetworkManagedServicesElServiceNetworkEndpointEl {
    #[doc = "Set the field `vpc_endpoint_id`.\n"]
    pub fn set_vpc_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_endpoint_id = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint_type`.\n"]
    pub fn set_vpc_endpoint_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_endpoint_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbNetworkManagedServicesElServiceNetworkEndpointEl {
    type O = BlockAssignable<DataOdbNetworkManagedServicesElServiceNetworkEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetworkManagedServicesElServiceNetworkEndpointEl {}

impl BuildDataOdbNetworkManagedServicesElServiceNetworkEndpointEl {
    pub fn build(self) -> DataOdbNetworkManagedServicesElServiceNetworkEndpointEl {
        DataOdbNetworkManagedServicesElServiceNetworkEndpointEl {
            vpc_endpoint_id: core::default::Default::default(),
            vpc_endpoint_type: core::default::Default::default(),
        }
    }
}

pub struct DataOdbNetworkManagedServicesElServiceNetworkEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbNetworkManagedServicesElServiceNetworkEndpointElRef {
    fn new(shared: StackShared, base: String) -> DataOdbNetworkManagedServicesElServiceNetworkEndpointElRef {
        DataOdbNetworkManagedServicesElServiceNetworkEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetworkManagedServicesElServiceNetworkEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_type` after provisioning.\n"]
    pub fn vpc_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetworkManagedServicesElZeroTlAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataOdbNetworkManagedServicesElZeroTlAccessEl {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbNetworkManagedServicesElZeroTlAccessEl {
    type O = BlockAssignable<DataOdbNetworkManagedServicesElZeroTlAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetworkManagedServicesElZeroTlAccessEl {}

impl BuildDataOdbNetworkManagedServicesElZeroTlAccessEl {
    pub fn build(self) -> DataOdbNetworkManagedServicesElZeroTlAccessEl {
        DataOdbNetworkManagedServicesElZeroTlAccessEl {
            cidr: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataOdbNetworkManagedServicesElZeroTlAccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbNetworkManagedServicesElZeroTlAccessElRef {
    fn new(shared: StackShared, base: String) -> DataOdbNetworkManagedServicesElZeroTlAccessElRef {
        DataOdbNetworkManagedServicesElZeroTlAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetworkManagedServicesElZeroTlAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetworkManagedServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_s3_backup_access: Option<ListField<DataOdbNetworkManagedServicesElManagedS3BackupAccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_service_ipv4_cidrs: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_gateway_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_access: Option<ListField<DataOdbNetworkManagedServicesElS3AccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_network_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_network_endpoint: Option<ListField<DataOdbNetworkManagedServicesElServiceNetworkEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zero_tl_access: Option<ListField<DataOdbNetworkManagedServicesElZeroTlAccessEl>>,
}

impl DataOdbNetworkManagedServicesEl {
    #[doc = "Set the field `managed_s3_backup_access`.\n"]
    pub fn set_managed_s3_backup_access(
        mut self,
        v: impl Into<ListField<DataOdbNetworkManagedServicesElManagedS3BackupAccessEl>>,
    ) -> Self {
        self.managed_s3_backup_access = Some(v.into());
        self
    }

    #[doc = "Set the field `managed_service_ipv4_cidrs`.\n"]
    pub fn set_managed_service_ipv4_cidrs(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.managed_service_ipv4_cidrs = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_gateway_arn`.\n"]
    pub fn set_resource_gateway_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_gateway_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_access`.\n"]
    pub fn set_s3_access(mut self, v: impl Into<ListField<DataOdbNetworkManagedServicesElS3AccessEl>>) -> Self {
        self.s3_access = Some(v.into());
        self
    }

    #[doc = "Set the field `service_network_arn`.\n"]
    pub fn set_service_network_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_network_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `service_network_endpoint`.\n"]
    pub fn set_service_network_endpoint(
        mut self,
        v: impl Into<ListField<DataOdbNetworkManagedServicesElServiceNetworkEndpointEl>>,
    ) -> Self {
        self.service_network_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `zero_tl_access`.\n"]
    pub fn set_zero_tl_access(mut self, v: impl Into<ListField<DataOdbNetworkManagedServicesElZeroTlAccessEl>>) -> Self {
        self.zero_tl_access = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbNetworkManagedServicesEl {
    type O = BlockAssignable<DataOdbNetworkManagedServicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetworkManagedServicesEl {}

impl BuildDataOdbNetworkManagedServicesEl {
    pub fn build(self) -> DataOdbNetworkManagedServicesEl {
        DataOdbNetworkManagedServicesEl {
            managed_s3_backup_access: core::default::Default::default(),
            managed_service_ipv4_cidrs: core::default::Default::default(),
            resource_gateway_arn: core::default::Default::default(),
            s3_access: core::default::Default::default(),
            service_network_arn: core::default::Default::default(),
            service_network_endpoint: core::default::Default::default(),
            zero_tl_access: core::default::Default::default(),
        }
    }
}

pub struct DataOdbNetworkManagedServicesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbNetworkManagedServicesElRef {
    fn new(shared: StackShared, base: String) -> DataOdbNetworkManagedServicesElRef {
        DataOdbNetworkManagedServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetworkManagedServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_s3_backup_access` after provisioning.\n"]
    pub fn managed_s3_backup_access(&self) -> ListRef<DataOdbNetworkManagedServicesElManagedS3BackupAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.managed_s3_backup_access", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_service_ipv4_cidrs` after provisioning.\n"]
    pub fn managed_service_ipv4_cidrs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.managed_service_ipv4_cidrs", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_gateway_arn` after provisioning.\n"]
    pub fn resource_gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_gateway_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_access` after provisioning.\n"]
    pub fn s3_access(&self) -> ListRef<DataOdbNetworkManagedServicesElS3AccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_access", self.base))
    }

    #[doc = "Get a reference to the value of field `service_network_arn` after provisioning.\n"]
    pub fn service_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_network_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `service_network_endpoint` after provisioning.\n"]
    pub fn service_network_endpoint(&self) -> ListRef<DataOdbNetworkManagedServicesElServiceNetworkEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_network_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `zero_tl_access` after provisioning.\n"]
    pub fn zero_tl_access(&self) -> ListRef<DataOdbNetworkManagedServicesElZeroTlAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zero_tl_access", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetworkOciDnsForwardingConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci_dns_listener_ip: Option<PrimField<String>>,
}

impl DataOdbNetworkOciDnsForwardingConfigsEl {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }

    #[doc = "Set the field `oci_dns_listener_ip`.\n"]
    pub fn set_oci_dns_listener_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oci_dns_listener_ip = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbNetworkOciDnsForwardingConfigsEl {
    type O = BlockAssignable<DataOdbNetworkOciDnsForwardingConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetworkOciDnsForwardingConfigsEl {}

impl BuildDataOdbNetworkOciDnsForwardingConfigsEl {
    pub fn build(self) -> DataOdbNetworkOciDnsForwardingConfigsEl {
        DataOdbNetworkOciDnsForwardingConfigsEl {
            domain_name: core::default::Default::default(),
            oci_dns_listener_ip: core::default::Default::default(),
        }
    }
}

pub struct DataOdbNetworkOciDnsForwardingConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbNetworkOciDnsForwardingConfigsElRef {
    fn new(shared: StackShared, base: String) -> DataOdbNetworkOciDnsForwardingConfigsElRef {
        DataOdbNetworkOciDnsForwardingConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetworkOciDnsForwardingConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_dns_listener_ip` after provisioning.\n"]
    pub fn oci_dns_listener_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_dns_listener_ip", self.base))
    }
}
