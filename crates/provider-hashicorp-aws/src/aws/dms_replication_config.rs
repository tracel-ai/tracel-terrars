use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DmsReplicationConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    replication_config_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_settings: Option<PrimField<String>>,
    replication_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_identifier: Option<PrimField<String>>,
    source_endpoint_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_replication: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supplemental_settings: Option<PrimField<String>>,
    table_mappings: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    target_endpoint_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_config: Option<Vec<DmsReplicationConfigComputeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DmsReplicationConfigTimeoutsEl>,
    dynamic: DmsReplicationConfigDynamic,
}

struct DmsReplicationConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DmsReplicationConfigData>,
}

#[derive(Clone)]
pub struct DmsReplicationConfig(Rc<DmsReplicationConfig_>);

impl DmsReplicationConfig {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `replication_settings`.\n"]
    pub fn set_replication_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().replication_settings = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_identifier`.\n"]
    pub fn set_resource_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `start_replication`.\n"]
    pub fn set_start_replication(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_replication = Some(v.into());
        self
    }

    #[doc = "Set the field `supplemental_settings`.\n"]
    pub fn set_supplemental_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().supplemental_settings = Some(v.into());
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

    #[doc = "Set the field `compute_config`.\n"]
    pub fn set_compute_config(
        self,
        v: impl Into<BlockAssignable<DmsReplicationConfigComputeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DmsReplicationConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_config_identifier` after provisioning.\n"]
    pub fn replication_config_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_config_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_settings` after provisioning.\n"]
    pub fn replication_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_type` after provisioning.\n"]
    pub fn replication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_identifier` after provisioning.\n"]
    pub fn resource_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_endpoint_arn` after provisioning.\n"]
    pub fn source_endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_endpoint_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `start_replication` after provisioning.\n"]
    pub fn start_replication(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_replication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supplemental_settings` after provisioning.\n"]
    pub fn supplemental_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supplemental_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_mappings` after provisioning.\n"]
    pub fn table_mappings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_mappings", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `target_endpoint_arn` after provisioning.\n"]
    pub fn target_endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_endpoint_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_config` after provisioning.\n"]
    pub fn compute_config(&self) -> ListRef<DmsReplicationConfigComputeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsReplicationConfigTimeoutsElRef {
        DmsReplicationConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DmsReplicationConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for DmsReplicationConfig {}

impl ToListMappable for DmsReplicationConfig {
    type O = ListRef<DmsReplicationConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DmsReplicationConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_dms_replication_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDmsReplicationConfig {
    pub tf_id: String,
    #[doc = ""]
    pub replication_config_identifier: PrimField<String>,
    #[doc = ""]
    pub replication_type: PrimField<String>,
    #[doc = ""]
    pub source_endpoint_arn: PrimField<String>,
    #[doc = ""]
    pub table_mappings: PrimField<String>,
    #[doc = ""]
    pub target_endpoint_arn: PrimField<String>,
}

impl BuildDmsReplicationConfig {
    pub fn build(self, stack: &mut Stack) -> DmsReplicationConfig {
        let out = DmsReplicationConfig(Rc::new(DmsReplicationConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DmsReplicationConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                replication_config_identifier: self.replication_config_identifier,
                replication_settings: core::default::Default::default(),
                replication_type: self.replication_type,
                resource_identifier: core::default::Default::default(),
                source_endpoint_arn: self.source_endpoint_arn,
                start_replication: core::default::Default::default(),
                supplemental_settings: core::default::Default::default(),
                table_mappings: self.table_mappings,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target_endpoint_arn: self.target_endpoint_arn,
                compute_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DmsReplicationConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsReplicationConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DmsReplicationConfigRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_config_identifier` after provisioning.\n"]
    pub fn replication_config_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_config_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_settings` after provisioning.\n"]
    pub fn replication_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_type` after provisioning.\n"]
    pub fn replication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_identifier` after provisioning.\n"]
    pub fn resource_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_endpoint_arn` after provisioning.\n"]
    pub fn source_endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_endpoint_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `start_replication` after provisioning.\n"]
    pub fn start_replication(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_replication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supplemental_settings` after provisioning.\n"]
    pub fn supplemental_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supplemental_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_mappings` after provisioning.\n"]
    pub fn table_mappings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_mappings", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `target_endpoint_arn` after provisioning.\n"]
    pub fn target_endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_endpoint_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_config` after provisioning.\n"]
    pub fn compute_config(&self) -> ListRef<DmsReplicationConfigComputeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsReplicationConfigTimeoutsElRef {
        DmsReplicationConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DmsReplicationConfigComputeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_name_servers: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_capacity_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_capacity_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_az: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_maintenance_window: Option<PrimField<String>>,
    replication_subnet_group_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_security_group_ids: Option<SetField<PrimField<String>>>,
}

impl DmsReplicationConfigComputeConfigEl {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }

    #[doc = "Set the field `dns_name_servers`.\n"]
    pub fn set_dns_name_servers(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_name_servers = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `max_capacity_units`.\n"]
    pub fn set_max_capacity_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_capacity_units = Some(v.into());
        self
    }

    #[doc = "Set the field `min_capacity_units`.\n"]
    pub fn set_min_capacity_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_capacity_units = Some(v.into());
        self
    }

    #[doc = "Set the field `multi_az`.\n"]
    pub fn set_multi_az(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.multi_az = Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_maintenance_window`.\n"]
    pub fn set_preferred_maintenance_window(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preferred_maintenance_window = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.vpc_security_group_ids = Some(v.into());
        self
    }
}

impl ToListMappable for DmsReplicationConfigComputeConfigEl {
    type O = BlockAssignable<DmsReplicationConfigComputeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsReplicationConfigComputeConfigEl {
    #[doc = ""]
    pub replication_subnet_group_id: PrimField<String>,
}

impl BuildDmsReplicationConfigComputeConfigEl {
    pub fn build(self) -> DmsReplicationConfigComputeConfigEl {
        DmsReplicationConfigComputeConfigEl {
            availability_zone: core::default::Default::default(),
            dns_name_servers: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            max_capacity_units: core::default::Default::default(),
            min_capacity_units: core::default::Default::default(),
            multi_az: core::default::Default::default(),
            preferred_maintenance_window: core::default::Default::default(),
            replication_subnet_group_id: self.replication_subnet_group_id,
            vpc_security_group_ids: core::default::Default::default(),
        }
    }
}

pub struct DmsReplicationConfigComputeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsReplicationConfigComputeConfigElRef {
    fn new(shared: StackShared, base: String) -> DmsReplicationConfigComputeConfigElRef {
        DmsReplicationConfigComputeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsReplicationConfigComputeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dns_name_servers` after provisioning.\n"]
    pub fn dns_name_servers(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name_servers", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `max_capacity_units` after provisioning.\n"]
    pub fn max_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_capacity_units", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `min_capacity_units` after provisioning.\n"]
    pub fn min_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_capacity_units", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_az", self.base))
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_maintenance_window", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `replication_subnet_group_id` after provisioning.\n"]
    pub fn replication_subnet_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_subnet_group_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DmsReplicationConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DmsReplicationConfigTimeoutsEl {
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

impl ToListMappable for DmsReplicationConfigTimeoutsEl {
    type O = BlockAssignable<DmsReplicationConfigTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsReplicationConfigTimeoutsEl {}

impl BuildDmsReplicationConfigTimeoutsEl {
    pub fn build(self) -> DmsReplicationConfigTimeoutsEl {
        DmsReplicationConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DmsReplicationConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsReplicationConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DmsReplicationConfigTimeoutsElRef {
        DmsReplicationConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsReplicationConfigTimeoutsElRef {
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
struct DmsReplicationConfigDynamic {
    compute_config: Option<DynamicBlock<DmsReplicationConfigComputeConfigEl>>,
}
