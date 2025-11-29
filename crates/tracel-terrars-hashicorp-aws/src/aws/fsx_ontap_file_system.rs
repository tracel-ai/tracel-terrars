use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxOntapFileSystemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_backup_retention_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_automatic_backup_start_time: Option<PrimField<String>>,
    deployment_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_ip_address_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fsx_admin_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ha_pairs: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    preferred_subnet_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_table_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    storage_capacity: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_type: Option<PrimField<String>>,
    subnet_ids: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput_capacity_per_ha_pair: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_iops_configuration: Option<Vec<FsxOntapFileSystemDiskIopsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FsxOntapFileSystemTimeoutsEl>,
    dynamic: FsxOntapFileSystemDynamic,
}

struct FsxOntapFileSystem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FsxOntapFileSystemData>,
}

#[derive(Clone)]
pub struct FsxOntapFileSystem(Rc<FsxOntapFileSystem_>);

impl FsxOntapFileSystem {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `automatic_backup_retention_days`.\n"]
    pub fn set_automatic_backup_retention_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().automatic_backup_retention_days = Some(v.into());
        self
    }

    #[doc = "Set the field `daily_automatic_backup_start_time`.\n"]
    pub fn set_daily_automatic_backup_start_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().daily_automatic_backup_start_time = Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_ip_address_range`.\n"]
    pub fn set_endpoint_ip_address_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().endpoint_ip_address_range = Some(v.into());
        self
    }

    #[doc = "Set the field `fsx_admin_password`.\n"]
    pub fn set_fsx_admin_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fsx_admin_password = Some(v.into());
        self
    }

    #[doc = "Set the field `ha_pairs`.\n"]
    pub fn set_ha_pairs(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ha_pairs = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_ids`.\n"]
    pub fn set_route_table_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().route_table_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_type = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `throughput_capacity`.\n"]
    pub fn set_throughput_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throughput_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `throughput_capacity_per_ha_pair`.\n"]
    pub fn set_throughput_capacity_per_ha_pair(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throughput_capacity_per_ha_pair = Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_maintenance_start_time`.\n"]
    pub fn set_weekly_maintenance_start_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().weekly_maintenance_start_time = Some(v.into());
        self
    }

    #[doc = "Set the field `disk_iops_configuration`.\n"]
    pub fn set_disk_iops_configuration(
        self,
        v: impl Into<BlockAssignable<FsxOntapFileSystemDiskIopsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_iops_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_iops_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FsxOntapFileSystemTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_backup_retention_days` after provisioning.\n"]
    pub fn automatic_backup_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_backup_retention_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.daily_automatic_backup_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_range` after provisioning.\n"]
    pub fn endpoint_ip_address_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> ListRef<FsxOntapFileSystemEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fsx_admin_password` after provisioning.\n"]
    pub fn fsx_admin_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fsx_admin_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ha_pairs` after provisioning.\n"]
    pub fn ha_pairs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ha_pairs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_subnet_id` after provisioning.\n"]
    pub fn preferred_subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_subnet_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_table_ids` after provisioning.\n"]
    pub fn route_table_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.route_table_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_capacity` after provisioning.\n"]
    pub fn throughput_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_capacity_per_ha_pair` after provisioning.\n"]
    pub fn throughput_capacity_per_ha_pair(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput_capacity_per_ha_pair", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_start_time` after provisioning.\n"]
    pub fn weekly_maintenance_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.weekly_maintenance_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_iops_configuration` after provisioning.\n"]
    pub fn disk_iops_configuration(&self) -> ListRef<FsxOntapFileSystemDiskIopsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk_iops_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapFileSystemTimeoutsElRef {
        FsxOntapFileSystemTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for FsxOntapFileSystem {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FsxOntapFileSystem { }

impl ToListMappable for FsxOntapFileSystem {
    type O = ListRef<FsxOntapFileSystemRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FsxOntapFileSystem_ {
    fn extract_resource_type(&self) -> String {
        "aws_fsx_ontap_file_system".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxOntapFileSystem {
    pub tf_id: String,
    #[doc = ""]
    pub deployment_type: PrimField<String>,
    #[doc = ""]
    pub preferred_subnet_id: PrimField<String>,
    #[doc = ""]
    pub storage_capacity: PrimField<f64>,
    #[doc = ""]
    pub subnet_ids: ListField<PrimField<String>>,
}

impl BuildFsxOntapFileSystem {
    pub fn build(self, stack: &mut Stack) -> FsxOntapFileSystem {
        let out = FsxOntapFileSystem(Rc::new(FsxOntapFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FsxOntapFileSystemData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                automatic_backup_retention_days: core::default::Default::default(),
                daily_automatic_backup_start_time: core::default::Default::default(),
                deployment_type: self.deployment_type,
                endpoint_ip_address_range: core::default::Default::default(),
                fsx_admin_password: core::default::Default::default(),
                ha_pairs: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                preferred_subnet_id: self.preferred_subnet_id,
                region: core::default::Default::default(),
                route_table_ids: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                storage_capacity: self.storage_capacity,
                storage_type: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                throughput_capacity: core::default::Default::default(),
                throughput_capacity_per_ha_pair: core::default::Default::default(),
                weekly_maintenance_start_time: core::default::Default::default(),
                disk_iops_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxOntapFileSystemRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapFileSystemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl FsxOntapFileSystemRef {
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

    #[doc = "Get a reference to the value of field `automatic_backup_retention_days` after provisioning.\n"]
    pub fn automatic_backup_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_backup_retention_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_automatic_backup_start_time` after provisioning.\n"]
    pub fn daily_automatic_backup_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.daily_automatic_backup_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_range` after provisioning.\n"]
    pub fn endpoint_ip_address_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> ListRef<FsxOntapFileSystemEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fsx_admin_password` after provisioning.\n"]
    pub fn fsx_admin_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fsx_admin_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ha_pairs` after provisioning.\n"]
    pub fn ha_pairs(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ha_pairs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_subnet_id` after provisioning.\n"]
    pub fn preferred_subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_subnet_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_table_ids` after provisioning.\n"]
    pub fn route_table_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.route_table_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_capacity` after provisioning.\n"]
    pub fn throughput_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_capacity_per_ha_pair` after provisioning.\n"]
    pub fn throughput_capacity_per_ha_pair(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput_capacity_per_ha_pair", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_start_time` after provisioning.\n"]
    pub fn weekly_maintenance_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.weekly_maintenance_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_iops_configuration` after provisioning.\n"]
    pub fn disk_iops_configuration(&self) -> ListRef<FsxOntapFileSystemDiskIopsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk_iops_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapFileSystemTimeoutsElRef {
        FsxOntapFileSystemTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FsxOntapFileSystemEndpointsElInterclusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<SetField<PrimField<String>>>,
}

impl FsxOntapFileSystemEndpointsElInterclusterEl {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ip_addresses = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapFileSystemEndpointsElInterclusterEl {
    type O = BlockAssignable<FsxOntapFileSystemEndpointsElInterclusterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapFileSystemEndpointsElInterclusterEl {}

impl BuildFsxOntapFileSystemEndpointsElInterclusterEl {
    pub fn build(self) -> FsxOntapFileSystemEndpointsElInterclusterEl {
        FsxOntapFileSystemEndpointsElInterclusterEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapFileSystemEndpointsElInterclusterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapFileSystemEndpointsElInterclusterElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapFileSystemEndpointsElInterclusterElRef {
        FsxOntapFileSystemEndpointsElInterclusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapFileSystemEndpointsElInterclusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapFileSystemEndpointsElManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<SetField<PrimField<String>>>,
}

impl FsxOntapFileSystemEndpointsElManagementEl {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ip_addresses = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapFileSystemEndpointsElManagementEl {
    type O = BlockAssignable<FsxOntapFileSystemEndpointsElManagementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapFileSystemEndpointsElManagementEl {}

impl BuildFsxOntapFileSystemEndpointsElManagementEl {
    pub fn build(self) -> FsxOntapFileSystemEndpointsElManagementEl {
        FsxOntapFileSystemEndpointsElManagementEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapFileSystemEndpointsElManagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapFileSystemEndpointsElManagementElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapFileSystemEndpointsElManagementElRef {
        FsxOntapFileSystemEndpointsElManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapFileSystemEndpointsElManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapFileSystemEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    intercluster: Option<ListField<FsxOntapFileSystemEndpointsElInterclusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<ListField<FsxOntapFileSystemEndpointsElManagementEl>>,
}

impl FsxOntapFileSystemEndpointsEl {
    #[doc = "Set the field `intercluster`.\n"]
    pub fn set_intercluster(mut self, v: impl Into<ListField<FsxOntapFileSystemEndpointsElInterclusterEl>>) -> Self {
        self.intercluster = Some(v.into());
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(mut self, v: impl Into<ListField<FsxOntapFileSystemEndpointsElManagementEl>>) -> Self {
        self.management = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapFileSystemEndpointsEl {
    type O = BlockAssignable<FsxOntapFileSystemEndpointsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapFileSystemEndpointsEl {}

impl BuildFsxOntapFileSystemEndpointsEl {
    pub fn build(self) -> FsxOntapFileSystemEndpointsEl {
        FsxOntapFileSystemEndpointsEl {
            intercluster: core::default::Default::default(),
            management: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapFileSystemEndpointsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapFileSystemEndpointsElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapFileSystemEndpointsElRef {
        FsxOntapFileSystemEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapFileSystemEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `intercluster` after provisioning.\n"]
    pub fn intercluster(&self) -> ListRef<FsxOntapFileSystemEndpointsElInterclusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.intercluster", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<FsxOntapFileSystemEndpointsElManagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapFileSystemDiskIopsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}

impl FsxOntapFileSystemDiskIopsConfigurationEl {
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapFileSystemDiskIopsConfigurationEl {
    type O = BlockAssignable<FsxOntapFileSystemDiskIopsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapFileSystemDiskIopsConfigurationEl {}

impl BuildFsxOntapFileSystemDiskIopsConfigurationEl {
    pub fn build(self) -> FsxOntapFileSystemDiskIopsConfigurationEl {
        FsxOntapFileSystemDiskIopsConfigurationEl {
            iops: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapFileSystemDiskIopsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapFileSystemDiskIopsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapFileSystemDiskIopsConfigurationElRef {
        FsxOntapFileSystemDiskIopsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapFileSystemDiskIopsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapFileSystemTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FsxOntapFileSystemTimeoutsEl {
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

impl ToListMappable for FsxOntapFileSystemTimeoutsEl {
    type O = BlockAssignable<FsxOntapFileSystemTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapFileSystemTimeoutsEl {}

impl BuildFsxOntapFileSystemTimeoutsEl {
    pub fn build(self) -> FsxOntapFileSystemTimeoutsEl {
        FsxOntapFileSystemTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapFileSystemTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapFileSystemTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapFileSystemTimeoutsElRef {
        FsxOntapFileSystemTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapFileSystemTimeoutsElRef {
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
struct FsxOntapFileSystemDynamic {
    disk_iops_configuration: Option<DynamicBlock<FsxOntapFileSystemDiskIopsConfigurationEl>>,
}
