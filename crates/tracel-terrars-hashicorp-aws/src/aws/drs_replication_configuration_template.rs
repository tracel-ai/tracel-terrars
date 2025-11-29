use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DrsReplicationConfigurationTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    associate_default_security_group: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_replicate_new_disks: Option<PrimField<bool>>,
    bandwidth_throttling: PrimField<f64>,
    create_public_ip: PrimField<bool>,
    data_plane_routing: PrimField<String>,
    default_large_staging_disk_type: PrimField<String>,
    ebs_encryption: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_encryption_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    replication_server_instance_type: PrimField<String>,
    replication_servers_security_groups_ids: ListField<PrimField<String>>,
    staging_area_subnet_id: PrimField<String>,
    staging_area_tags: RecField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    use_dedicated_replication_server: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pit_policy: Option<Vec<DrsReplicationConfigurationTemplatePitPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DrsReplicationConfigurationTemplateTimeoutsEl>,
    dynamic: DrsReplicationConfigurationTemplateDynamic,
}

struct DrsReplicationConfigurationTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DrsReplicationConfigurationTemplateData>,
}

#[derive(Clone)]
pub struct DrsReplicationConfigurationTemplate(Rc<DrsReplicationConfigurationTemplate_>);

impl DrsReplicationConfigurationTemplate {
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

    #[doc = "Set the field `auto_replicate_new_disks`.\n"]
    pub fn set_auto_replicate_new_disks(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_replicate_new_disks = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_encryption_key_arn`.\n"]
    pub fn set_ebs_encryption_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ebs_encryption_key_arn = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `pit_policy`.\n"]
    pub fn set_pit_policy(self, v: impl Into<BlockAssignable<DrsReplicationConfigurationTemplatePitPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pit_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pit_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DrsReplicationConfigurationTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associate_default_security_group` after provisioning.\n"]
    pub fn associate_default_security_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.associate_default_security_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_replicate_new_disks` after provisioning.\n"]
    pub fn auto_replicate_new_disks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_replicate_new_disks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth_throttling` after provisioning.\n"]
    pub fn bandwidth_throttling(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bandwidth_throttling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_public_ip` after provisioning.\n"]
    pub fn create_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_public_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_plane_routing` after provisioning.\n"]
    pub fn data_plane_routing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_plane_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_large_staging_disk_type` after provisioning.\n"]
    pub fn default_large_staging_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_large_staging_disk_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_encryption` after provisioning.\n"]
    pub fn ebs_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_encryption_key_arn` after provisioning.\n"]
    pub fn ebs_encryption_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_server_instance_type` after provisioning.\n"]
    pub fn replication_server_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replication_server_instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_servers_security_groups_ids` after provisioning.\n"]
    pub fn replication_servers_security_groups_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_servers_security_groups_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `staging_area_subnet_id` after provisioning.\n"]
    pub fn staging_area_subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.staging_area_subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `staging_area_tags` after provisioning.\n"]
    pub fn staging_area_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.staging_area_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_dedicated_replication_server` after provisioning.\n"]
    pub fn use_dedicated_replication_server(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_dedicated_replication_server", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pit_policy` after provisioning.\n"]
    pub fn pit_policy(&self) -> ListRef<DrsReplicationConfigurationTemplatePitPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pit_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DrsReplicationConfigurationTemplateTimeoutsElRef {
        DrsReplicationConfigurationTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DrsReplicationConfigurationTemplate {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DrsReplicationConfigurationTemplate { }

impl ToListMappable for DrsReplicationConfigurationTemplate {
    type O = ListRef<DrsReplicationConfigurationTemplateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DrsReplicationConfigurationTemplate_ {
    fn extract_resource_type(&self) -> String {
        "aws_drs_replication_configuration_template".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDrsReplicationConfigurationTemplate {
    pub tf_id: String,
    #[doc = ""]
    pub associate_default_security_group: PrimField<bool>,
    #[doc = ""]
    pub bandwidth_throttling: PrimField<f64>,
    #[doc = ""]
    pub create_public_ip: PrimField<bool>,
    #[doc = ""]
    pub data_plane_routing: PrimField<String>,
    #[doc = ""]
    pub default_large_staging_disk_type: PrimField<String>,
    #[doc = ""]
    pub ebs_encryption: PrimField<String>,
    #[doc = ""]
    pub replication_server_instance_type: PrimField<String>,
    #[doc = ""]
    pub replication_servers_security_groups_ids: ListField<PrimField<String>>,
    #[doc = ""]
    pub staging_area_subnet_id: PrimField<String>,
    #[doc = ""]
    pub staging_area_tags: RecField<PrimField<String>>,
    #[doc = ""]
    pub use_dedicated_replication_server: PrimField<bool>,
}

impl BuildDrsReplicationConfigurationTemplate {
    pub fn build(self, stack: &mut Stack) -> DrsReplicationConfigurationTemplate {
        let out = DrsReplicationConfigurationTemplate(Rc::new(DrsReplicationConfigurationTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DrsReplicationConfigurationTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                associate_default_security_group: self.associate_default_security_group,
                auto_replicate_new_disks: core::default::Default::default(),
                bandwidth_throttling: self.bandwidth_throttling,
                create_public_ip: self.create_public_ip,
                data_plane_routing: self.data_plane_routing,
                default_large_staging_disk_type: self.default_large_staging_disk_type,
                ebs_encryption: self.ebs_encryption,
                ebs_encryption_key_arn: core::default::Default::default(),
                region: core::default::Default::default(),
                replication_server_instance_type: self.replication_server_instance_type,
                replication_servers_security_groups_ids: self.replication_servers_security_groups_ids,
                staging_area_subnet_id: self.staging_area_subnet_id,
                staging_area_tags: self.staging_area_tags,
                tags: core::default::Default::default(),
                use_dedicated_replication_server: self.use_dedicated_replication_server,
                pit_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DrsReplicationConfigurationTemplateRef {
    shared: StackShared,
    base: String,
}

impl Ref for DrsReplicationConfigurationTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DrsReplicationConfigurationTemplateRef {
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

    #[doc = "Get a reference to the value of field `associate_default_security_group` after provisioning.\n"]
    pub fn associate_default_security_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.associate_default_security_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_replicate_new_disks` after provisioning.\n"]
    pub fn auto_replicate_new_disks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_replicate_new_disks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth_throttling` after provisioning.\n"]
    pub fn bandwidth_throttling(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bandwidth_throttling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_public_ip` after provisioning.\n"]
    pub fn create_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_public_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_plane_routing` after provisioning.\n"]
    pub fn data_plane_routing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_plane_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_large_staging_disk_type` after provisioning.\n"]
    pub fn default_large_staging_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_large_staging_disk_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_encryption` after provisioning.\n"]
    pub fn ebs_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_encryption_key_arn` after provisioning.\n"]
    pub fn ebs_encryption_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_server_instance_type` after provisioning.\n"]
    pub fn replication_server_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replication_server_instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_servers_security_groups_ids` after provisioning.\n"]
    pub fn replication_servers_security_groups_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_servers_security_groups_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `staging_area_subnet_id` after provisioning.\n"]
    pub fn staging_area_subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.staging_area_subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `staging_area_tags` after provisioning.\n"]
    pub fn staging_area_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.staging_area_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_dedicated_replication_server` after provisioning.\n"]
    pub fn use_dedicated_replication_server(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_dedicated_replication_server", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pit_policy` after provisioning.\n"]
    pub fn pit_policy(&self) -> ListRef<DrsReplicationConfigurationTemplatePitPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pit_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DrsReplicationConfigurationTemplateTimeoutsElRef {
        DrsReplicationConfigurationTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DrsReplicationConfigurationTemplatePitPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    interval: PrimField<f64>,
    retention_duration: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_id: Option<PrimField<f64>>,
    units: PrimField<String>,
}

impl DrsReplicationConfigurationTemplatePitPolicyEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_id`.\n"]
    pub fn set_rule_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rule_id = Some(v.into());
        self
    }
}

impl ToListMappable for DrsReplicationConfigurationTemplatePitPolicyEl {
    type O = BlockAssignable<DrsReplicationConfigurationTemplatePitPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDrsReplicationConfigurationTemplatePitPolicyEl {
    #[doc = ""]
    pub interval: PrimField<f64>,
    #[doc = ""]
    pub retention_duration: PrimField<f64>,
    #[doc = ""]
    pub units: PrimField<String>,
}

impl BuildDrsReplicationConfigurationTemplatePitPolicyEl {
    pub fn build(self) -> DrsReplicationConfigurationTemplatePitPolicyEl {
        DrsReplicationConfigurationTemplatePitPolicyEl {
            enabled: core::default::Default::default(),
            interval: self.interval,
            retention_duration: self.retention_duration,
            rule_id: core::default::Default::default(),
            units: self.units,
        }
    }
}

pub struct DrsReplicationConfigurationTemplatePitPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DrsReplicationConfigurationTemplatePitPolicyElRef {
    fn new(shared: StackShared, base: String) -> DrsReplicationConfigurationTemplatePitPolicyElRef {
        DrsReplicationConfigurationTemplatePitPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DrsReplicationConfigurationTemplatePitPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_duration` after provisioning.\n"]
    pub fn retention_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.base))
    }

    #[doc = "Get a reference to the value of field `units` after provisioning.\n"]
    pub fn units(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}

#[derive(Serialize)]
pub struct DrsReplicationConfigurationTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DrsReplicationConfigurationTemplateTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for DrsReplicationConfigurationTemplateTimeoutsEl {
    type O = BlockAssignable<DrsReplicationConfigurationTemplateTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDrsReplicationConfigurationTemplateTimeoutsEl {}

impl BuildDrsReplicationConfigurationTemplateTimeoutsEl {
    pub fn build(self) -> DrsReplicationConfigurationTemplateTimeoutsEl {
        DrsReplicationConfigurationTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DrsReplicationConfigurationTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DrsReplicationConfigurationTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DrsReplicationConfigurationTemplateTimeoutsElRef {
        DrsReplicationConfigurationTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DrsReplicationConfigurationTemplateTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct DrsReplicationConfigurationTemplateDynamic {
    pit_policy: Option<DynamicBlock<DrsReplicationConfigurationTemplatePitPolicyEl>>,
}
