use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct OdbNetworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    availability_zone_id: PrimField<String>,
    backup_subnet_cidr: PrimField<String>,
    client_subnet_cidr: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_dns_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_associated_resources: Option<PrimField<bool>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    s3_access: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_policy_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    zero_etl_access: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OdbNetworkTimeoutsEl>,
}

struct OdbNetwork_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OdbNetworkData>,
}

#[derive(Clone)]
pub struct OdbNetwork(Rc<OdbNetwork_>);

impl OdbNetwork {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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

    #[doc = "Set the field `availability_zone`.\nThe name of the Availability Zone (AZ) where the odb network is located. Changing this will force terraform to create new resource"]
    pub fn set_availability_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_domain_name`.\nThe name of the custom domain that the network is located. custom_domain_name and default_dns_prefix both can't be given."]
    pub fn set_custom_domain_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_domain_name = Some(v.into());
        self
    }

    #[doc = "Set the field `default_dns_prefix`.\nThe default DNS prefix for the network resource. Changing this will force terraform to create new resource."]
    pub fn set_default_dns_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_dns_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `delete_associated_resources`.\nIf set to true deletes associated OCI resources. Default false."]
    pub fn set_delete_associated_resources(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_associated_resources = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_policy_document`.\nSpecifies the endpoint policy for Amazon S3 access from the ODB network."]
    pub fn set_s3_policy_document(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_policy_document = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OdbNetworkTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\nThe name of the Availability Zone (AZ) where the odb network is located. Changing this will force terraform to create new resource"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\nThe AZ ID of the AZ where the ODB network is located. Changing this will force terraform to create new resource."]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `backup_subnet_cidr` after provisioning.\nThe CIDR range of the backup subnet for the ODB network. Changing this will force terraform to create new resource.\n\tConstraints:\n\t   - Must not overlap with the CIDR range of the client subnet.\n\t   - Must not overlap with the CIDR ranges of the VPCs that are connected to the\n\t   ODB network.\n\t   - Must not use the following CIDR ranges that are reserved by OCI:\n\t   - 100.106.0.0/16 and 100.107.0.0/16\n\t   - 169.254.0.0/16\n\t   - 224.0.0.0 - 239.255.255.255\n\t   - 240.0.0.0 - 255.255.255.255"]
    pub fn backup_subnet_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_subnet_cidr", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_subnet_cidr` after provisioning.\nThe CIDR notation for the network resource. Changing this will force terraform to create new resource.\n Constraints:\n  \t - Must not overlap with the CIDR range of the backup subnet.\n   \t- Must not overlap with the CIDR ranges of the VPCs that are connected to the\n   ODB network.\n  \t- Must not use the following CIDR ranges that are reserved by OCI:\n  \t - 100.106.0.0/16 and 100.107.0.0/16\n  \t - 169.254.0.0/16\n   \t- 224.0.0.0 - 239.255.255.255\n   \t- 240.0.0.0 - 255.255.255.255"]
    pub fn client_subnet_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_subnet_cidr", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the ODB network was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `custom_domain_name` after provisioning.\nThe name of the custom domain that the network is located. custom_domain_name and default_dns_prefix both can't be given."]
    pub fn custom_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_dns_prefix` after provisioning.\nThe default DNS prefix for the network resource. Changing this will force terraform to create new resource."]
    pub fn default_dns_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_dns_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delete_associated_resources` after provisioning.\nIf set to true deletes associated OCI resources. Default false."]
    pub fn delete_associated_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_associated_resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe user-friendly name for the odb network. Changing this will force terraform to create a new resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_services` after provisioning.\nThe managed services configuration for the ODB network."]
    pub fn managed_services(&self) -> ListRef<OdbNetworkManagedServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_services", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_dns_forwarding_configs` after provisioning.\nThe DNS resolver endpoint in OCI for forwarding DNS queries for the ociPrivateZone domain."]
    pub fn oci_dns_forwarding_configs(&self) -> ListRef<OdbNetworkOciDnsForwardingConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oci_dns_forwarding_configs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_network_anchor_id` after provisioning.\nThe unique identifier of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_network_anchor_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_network_anchor_url` after provisioning.\nThe URL of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_network_anchor_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the ODB network."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_resource_anchor_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_vcn_id` after provisioning.\nThe unique identifier  Oracle Cloud ID (OCID) of the OCI VCN for the ODB network."]
    pub fn oci_vcn_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_vcn_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_vcn_url` after provisioning.\nThe URL of the OCI VCN for the ODB network."]
    pub fn oci_vcn_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_vcn_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peered_cidrs` after provisioning.\nThe list of CIDR ranges from the peered VPC that are allowed access to the ODB network. Please refer odb network peering documentation."]
    pub fn peered_cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.peered_cidrs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the ODB network, expressed as a percentage."]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.percent_progress", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_access` after provisioning.\nSpecifies the configuration for Amazon S3 access from the ODB network."]
    pub fn s3_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_policy_document` after provisioning.\nSpecifies the endpoint policy for Amazon S3 access from the ODB network."]
    pub fn s3_policy_document(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_policy_document", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the network resource."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the ODB network."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `zero_etl_access` after provisioning.\nSpecifies the configuration for Zero-ETL access from the ODB network."]
    pub fn zero_etl_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zero_etl_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbNetworkTimeoutsElRef {
        OdbNetworkTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for OdbNetwork {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for OdbNetwork {}

impl ToListMappable for OdbNetwork {
    type O = ListRef<OdbNetworkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OdbNetwork_ {
    fn extract_resource_type(&self) -> String {
        "aws_odb_network".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOdbNetwork {
    pub tf_id: String,
    #[doc = "The AZ ID of the AZ where the ODB network is located. Changing this will force terraform to create new resource."]
    pub availability_zone_id: PrimField<String>,
    #[doc = "The CIDR range of the backup subnet for the ODB network. Changing this will force terraform to create new resource.\n\tConstraints:\n\t   - Must not overlap with the CIDR range of the client subnet.\n\t   - Must not overlap with the CIDR ranges of the VPCs that are connected to the\n\t   ODB network.\n\t   - Must not use the following CIDR ranges that are reserved by OCI:\n\t   - 100.106.0.0/16 and 100.107.0.0/16\n\t   - 169.254.0.0/16\n\t   - 224.0.0.0 - 239.255.255.255\n\t   - 240.0.0.0 - 255.255.255.255"]
    pub backup_subnet_cidr: PrimField<String>,
    #[doc = "The CIDR notation for the network resource. Changing this will force terraform to create new resource.\n Constraints:\n  \t - Must not overlap with the CIDR range of the backup subnet.\n   \t- Must not overlap with the CIDR ranges of the VPCs that are connected to the\n   ODB network.\n  \t- Must not use the following CIDR ranges that are reserved by OCI:\n  \t - 100.106.0.0/16 and 100.107.0.0/16\n  \t - 169.254.0.0/16\n   \t- 224.0.0.0 - 239.255.255.255\n   \t- 240.0.0.0 - 255.255.255.255"]
    pub client_subnet_cidr: PrimField<String>,
    #[doc = "The user-friendly name for the odb network. Changing this will force terraform to create a new resource."]
    pub display_name: PrimField<String>,
    #[doc = "Specifies the configuration for Amazon S3 access from the ODB network."]
    pub s3_access: PrimField<String>,
    #[doc = "Specifies the configuration for Zero-ETL access from the ODB network."]
    pub zero_etl_access: PrimField<String>,
}

impl BuildOdbNetwork {
    pub fn build(self, stack: &mut Stack) -> OdbNetwork {
        let out = OdbNetwork(Rc::new(OdbNetwork_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OdbNetworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                availability_zone: core::default::Default::default(),
                availability_zone_id: self.availability_zone_id,
                backup_subnet_cidr: self.backup_subnet_cidr,
                client_subnet_cidr: self.client_subnet_cidr,
                custom_domain_name: core::default::Default::default(),
                default_dns_prefix: core::default::Default::default(),
                delete_associated_resources: core::default::Default::default(),
                display_name: self.display_name,
                region: core::default::Default::default(),
                s3_access: self.s3_access,
                s3_policy_document: core::default::Default::default(),
                tags: core::default::Default::default(),
                zero_etl_access: self.zero_etl_access,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OdbNetworkRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbNetworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl OdbNetworkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\nThe name of the Availability Zone (AZ) where the odb network is located. Changing this will force terraform to create new resource"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\nThe AZ ID of the AZ where the ODB network is located. Changing this will force terraform to create new resource."]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `backup_subnet_cidr` after provisioning.\nThe CIDR range of the backup subnet for the ODB network. Changing this will force terraform to create new resource.\n\tConstraints:\n\t   - Must not overlap with the CIDR range of the client subnet.\n\t   - Must not overlap with the CIDR ranges of the VPCs that are connected to the\n\t   ODB network.\n\t   - Must not use the following CIDR ranges that are reserved by OCI:\n\t   - 100.106.0.0/16 and 100.107.0.0/16\n\t   - 169.254.0.0/16\n\t   - 224.0.0.0 - 239.255.255.255\n\t   - 240.0.0.0 - 255.255.255.255"]
    pub fn backup_subnet_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_subnet_cidr", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_subnet_cidr` after provisioning.\nThe CIDR notation for the network resource. Changing this will force terraform to create new resource.\n Constraints:\n  \t - Must not overlap with the CIDR range of the backup subnet.\n   \t- Must not overlap with the CIDR ranges of the VPCs that are connected to the\n   ODB network.\n  \t- Must not use the following CIDR ranges that are reserved by OCI:\n  \t - 100.106.0.0/16 and 100.107.0.0/16\n  \t - 169.254.0.0/16\n   \t- 224.0.0.0 - 239.255.255.255\n   \t- 240.0.0.0 - 255.255.255.255"]
    pub fn client_subnet_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_subnet_cidr", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the ODB network was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `custom_domain_name` after provisioning.\nThe name of the custom domain that the network is located. custom_domain_name and default_dns_prefix both can't be given."]
    pub fn custom_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_dns_prefix` after provisioning.\nThe default DNS prefix for the network resource. Changing this will force terraform to create new resource."]
    pub fn default_dns_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_dns_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delete_associated_resources` after provisioning.\nIf set to true deletes associated OCI resources. Default false."]
    pub fn delete_associated_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_associated_resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe user-friendly name for the odb network. Changing this will force terraform to create a new resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_services` after provisioning.\nThe managed services configuration for the ODB network."]
    pub fn managed_services(&self) -> ListRef<OdbNetworkManagedServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_services", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_dns_forwarding_configs` after provisioning.\nThe DNS resolver endpoint in OCI for forwarding DNS queries for the ociPrivateZone domain."]
    pub fn oci_dns_forwarding_configs(&self) -> ListRef<OdbNetworkOciDnsForwardingConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oci_dns_forwarding_configs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_network_anchor_id` after provisioning.\nThe unique identifier of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_network_anchor_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_network_anchor_url` after provisioning.\nThe URL of the OCI network anchor for the ODB network."]
    pub fn oci_network_anchor_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_network_anchor_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the ODB network."]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_resource_anchor_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_vcn_id` after provisioning.\nThe unique identifier  Oracle Cloud ID (OCID) of the OCI VCN for the ODB network."]
    pub fn oci_vcn_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_vcn_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oci_vcn_url` after provisioning.\nThe URL of the OCI VCN for the ODB network."]
    pub fn oci_vcn_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_vcn_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peered_cidrs` after provisioning.\nThe list of CIDR ranges from the peered VPC that are allowed access to the ODB network. Please refer odb network peering documentation."]
    pub fn peered_cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.peered_cidrs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the ODB network, expressed as a percentage."]
    pub fn percent_progress(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.percent_progress", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_access` after provisioning.\nSpecifies the configuration for Amazon S3 access from the ODB network."]
    pub fn s3_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_policy_document` after provisioning.\nSpecifies the endpoint policy for Amazon S3 access from the ODB network."]
    pub fn s3_policy_document(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_policy_document", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the network resource."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the ODB network."]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `zero_etl_access` after provisioning.\nSpecifies the configuration for Zero-ETL access from the ODB network."]
    pub fn zero_etl_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zero_etl_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbNetworkTimeoutsElRef {
        OdbNetworkTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct OdbNetworkManagedServicesElManagedS3BackupAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_addresses: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl OdbNetworkManagedServicesElManagedS3BackupAccessEl {
    #[doc = "Set the field `ipv4_addresses`.\n"]
    pub fn set_ipv4_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ipv4_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for OdbNetworkManagedServicesElManagedS3BackupAccessEl {
    type O = BlockAssignable<OdbNetworkManagedServicesElManagedS3BackupAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbNetworkManagedServicesElManagedS3BackupAccessEl {}

impl BuildOdbNetworkManagedServicesElManagedS3BackupAccessEl {
    pub fn build(self) -> OdbNetworkManagedServicesElManagedS3BackupAccessEl {
        OdbNetworkManagedServicesElManagedS3BackupAccessEl {
            ipv4_addresses: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct OdbNetworkManagedServicesElManagedS3BackupAccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbNetworkManagedServicesElManagedS3BackupAccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OdbNetworkManagedServicesElManagedS3BackupAccessElRef {
        OdbNetworkManagedServicesElManagedS3BackupAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbNetworkManagedServicesElManagedS3BackupAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\n"]
    pub fn ipv4_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ipv4_addresses", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbNetworkManagedServicesElS3AccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_addresses: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_policy_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl OdbNetworkManagedServicesElS3AccessEl {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_addresses`.\n"]
    pub fn set_ipv4_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
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

impl ToListMappable for OdbNetworkManagedServicesElS3AccessEl {
    type O = BlockAssignable<OdbNetworkManagedServicesElS3AccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbNetworkManagedServicesElS3AccessEl {}

impl BuildOdbNetworkManagedServicesElS3AccessEl {
    pub fn build(self) -> OdbNetworkManagedServicesElS3AccessEl {
        OdbNetworkManagedServicesElS3AccessEl {
            domain_name: core::default::Default::default(),
            ipv4_addresses: core::default::Default::default(),
            s3_policy_document: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct OdbNetworkManagedServicesElS3AccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbNetworkManagedServicesElS3AccessElRef {
    fn new(shared: StackShared, base: String) -> OdbNetworkManagedServicesElS3AccessElRef {
        OdbNetworkManagedServicesElS3AccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbNetworkManagedServicesElS3AccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\n"]
    pub fn ipv4_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ipv4_addresses", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_policy_document` after provisioning.\n"]
    pub fn s3_policy_document(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_policy_document", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbNetworkManagedServicesElServiceNetworkEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_type: Option<PrimField<String>>,
}

impl OdbNetworkManagedServicesElServiceNetworkEndpointEl {
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

impl ToListMappable for OdbNetworkManagedServicesElServiceNetworkEndpointEl {
    type O = BlockAssignable<OdbNetworkManagedServicesElServiceNetworkEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbNetworkManagedServicesElServiceNetworkEndpointEl {}

impl BuildOdbNetworkManagedServicesElServiceNetworkEndpointEl {
    pub fn build(self) -> OdbNetworkManagedServicesElServiceNetworkEndpointEl {
        OdbNetworkManagedServicesElServiceNetworkEndpointEl {
            vpc_endpoint_id: core::default::Default::default(),
            vpc_endpoint_type: core::default::Default::default(),
        }
    }
}

pub struct OdbNetworkManagedServicesElServiceNetworkEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbNetworkManagedServicesElServiceNetworkEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OdbNetworkManagedServicesElServiceNetworkEndpointElRef {
        OdbNetworkManagedServicesElServiceNetworkEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbNetworkManagedServicesElServiceNetworkEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_type` after provisioning.\n"]
    pub fn vpc_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_type", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct OdbNetworkManagedServicesElZeroEtlAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl OdbNetworkManagedServicesElZeroEtlAccessEl {
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

impl ToListMappable for OdbNetworkManagedServicesElZeroEtlAccessEl {
    type O = BlockAssignable<OdbNetworkManagedServicesElZeroEtlAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbNetworkManagedServicesElZeroEtlAccessEl {}

impl BuildOdbNetworkManagedServicesElZeroEtlAccessEl {
    pub fn build(self) -> OdbNetworkManagedServicesElZeroEtlAccessEl {
        OdbNetworkManagedServicesElZeroEtlAccessEl {
            cidr: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct OdbNetworkManagedServicesElZeroEtlAccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbNetworkManagedServicesElZeroEtlAccessElRef {
    fn new(shared: StackShared, base: String) -> OdbNetworkManagedServicesElZeroEtlAccessElRef {
        OdbNetworkManagedServicesElZeroEtlAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbNetworkManagedServicesElZeroEtlAccessElRef {
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
pub struct OdbNetworkManagedServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_s3_backup_access: Option<ListField<OdbNetworkManagedServicesElManagedS3BackupAccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_service_ipv4_cidrs: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_gateway_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_access: Option<ListField<OdbNetworkManagedServicesElS3AccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_network_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_network_endpoint:
        Option<ListField<OdbNetworkManagedServicesElServiceNetworkEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zero_etl_access: Option<ListField<OdbNetworkManagedServicesElZeroEtlAccessEl>>,
}

impl OdbNetworkManagedServicesEl {
    #[doc = "Set the field `managed_s3_backup_access`.\n"]
    pub fn set_managed_s3_backup_access(
        mut self,
        v: impl Into<ListField<OdbNetworkManagedServicesElManagedS3BackupAccessEl>>,
    ) -> Self {
        self.managed_s3_backup_access = Some(v.into());
        self
    }

    #[doc = "Set the field `managed_service_ipv4_cidrs`.\n"]
    pub fn set_managed_service_ipv4_cidrs(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.managed_service_ipv4_cidrs = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_gateway_arn`.\n"]
    pub fn set_resource_gateway_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_gateway_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_access`.\n"]
    pub fn set_s3_access(
        mut self,
        v: impl Into<ListField<OdbNetworkManagedServicesElS3AccessEl>>,
    ) -> Self {
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
        v: impl Into<ListField<OdbNetworkManagedServicesElServiceNetworkEndpointEl>>,
    ) -> Self {
        self.service_network_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `zero_etl_access`.\n"]
    pub fn set_zero_etl_access(
        mut self,
        v: impl Into<ListField<OdbNetworkManagedServicesElZeroEtlAccessEl>>,
    ) -> Self {
        self.zero_etl_access = Some(v.into());
        self
    }
}

impl ToListMappable for OdbNetworkManagedServicesEl {
    type O = BlockAssignable<OdbNetworkManagedServicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbNetworkManagedServicesEl {}

impl BuildOdbNetworkManagedServicesEl {
    pub fn build(self) -> OdbNetworkManagedServicesEl {
        OdbNetworkManagedServicesEl {
            managed_s3_backup_access: core::default::Default::default(),
            managed_service_ipv4_cidrs: core::default::Default::default(),
            resource_gateway_arn: core::default::Default::default(),
            s3_access: core::default::Default::default(),
            service_network_arn: core::default::Default::default(),
            service_network_endpoint: core::default::Default::default(),
            zero_etl_access: core::default::Default::default(),
        }
    }
}

pub struct OdbNetworkManagedServicesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbNetworkManagedServicesElRef {
    fn new(shared: StackShared, base: String) -> OdbNetworkManagedServicesElRef {
        OdbNetworkManagedServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbNetworkManagedServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_s3_backup_access` after provisioning.\n"]
    pub fn managed_s3_backup_access(
        &self,
    ) -> ListRef<OdbNetworkManagedServicesElManagedS3BackupAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_s3_backup_access", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `managed_service_ipv4_cidrs` after provisioning.\n"]
    pub fn managed_service_ipv4_cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.managed_service_ipv4_cidrs", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resource_gateway_arn` after provisioning.\n"]
    pub fn resource_gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_gateway_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_access` after provisioning.\n"]
    pub fn s3_access(&self) -> ListRef<OdbNetworkManagedServicesElS3AccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_access", self.base))
    }

    #[doc = "Get a reference to the value of field `service_network_arn` after provisioning.\n"]
    pub fn service_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_network_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `service_network_endpoint` after provisioning.\n"]
    pub fn service_network_endpoint(
        &self,
    ) -> ListRef<OdbNetworkManagedServicesElServiceNetworkEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_network_endpoint", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `zero_etl_access` after provisioning.\n"]
    pub fn zero_etl_access(&self) -> ListRef<OdbNetworkManagedServicesElZeroEtlAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zero_etl_access", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct OdbNetworkOciDnsForwardingConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci_dns_listener_ip: Option<PrimField<String>>,
}

impl OdbNetworkOciDnsForwardingConfigsEl {
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

impl ToListMappable for OdbNetworkOciDnsForwardingConfigsEl {
    type O = BlockAssignable<OdbNetworkOciDnsForwardingConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbNetworkOciDnsForwardingConfigsEl {}

impl BuildOdbNetworkOciDnsForwardingConfigsEl {
    pub fn build(self) -> OdbNetworkOciDnsForwardingConfigsEl {
        OdbNetworkOciDnsForwardingConfigsEl {
            domain_name: core::default::Default::default(),
            oci_dns_listener_ip: core::default::Default::default(),
        }
    }
}

pub struct OdbNetworkOciDnsForwardingConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbNetworkOciDnsForwardingConfigsElRef {
    fn new(shared: StackShared, base: String) -> OdbNetworkOciDnsForwardingConfigsElRef {
        OdbNetworkOciDnsForwardingConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbNetworkOciDnsForwardingConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_dns_listener_ip` after provisioning.\n"]
    pub fn oci_dns_listener_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_dns_listener_ip", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct OdbNetworkTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl OdbNetworkTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for OdbNetworkTimeoutsEl {
    type O = BlockAssignable<OdbNetworkTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbNetworkTimeoutsEl {}

impl BuildOdbNetworkTimeoutsEl {
    pub fn build(self) -> OdbNetworkTimeoutsEl {
        OdbNetworkTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct OdbNetworkTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OdbNetworkTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OdbNetworkTimeoutsElRef {
        OdbNetworkTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbNetworkTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
