use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct FinspaceKxClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_id: Option<PrimField<String>>,
    az_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command_line_arguments: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    environment_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initialization_script: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    release_label: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_scaling_configuration: Option<Vec<FinspaceKxClusterAutoScalingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_storage_configurations: Option<Vec<FinspaceKxClusterCacheStorageConfigurationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_configuration: Option<Vec<FinspaceKxClusterCapacityConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<Vec<FinspaceKxClusterCodeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<Vec<FinspaceKxClusterDatabaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    savedown_storage_configuration: Option<Vec<FinspaceKxClusterSavedownStorageConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_group_configuration: Option<Vec<FinspaceKxClusterScalingGroupConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tickerplant_log_configuration: Option<Vec<FinspaceKxClusterTickerplantLogConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FinspaceKxClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_configuration: Option<Vec<FinspaceKxClusterVpcConfigurationEl>>,
    dynamic: FinspaceKxClusterDynamic,
}

struct FinspaceKxCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FinspaceKxClusterData>,
}

#[derive(Clone)]
pub struct FinspaceKxCluster(Rc<FinspaceKxCluster_>);

impl FinspaceKxCluster {
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

    #[doc = "Set the field `availability_zone_id`.\n"]
    pub fn set_availability_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_zone_id = Some(v.into());
        self
    }

    #[doc = "Set the field `command_line_arguments`.\n"]
    pub fn set_command_line_arguments(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().command_line_arguments = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role`.\n"]
    pub fn set_execution_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().execution_role = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `initialization_script`.\n"]
    pub fn set_initialization_script(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().initialization_script = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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

    #[doc = "Set the field `auto_scaling_configuration`.\n"]
    pub fn set_auto_scaling_configuration(
        self,
        v: impl Into<BlockAssignable<FinspaceKxClusterAutoScalingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_scaling_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_scaling_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `cache_storage_configurations`.\n"]
    pub fn set_cache_storage_configurations(
        self,
        v: impl Into<BlockAssignable<FinspaceKxClusterCacheStorageConfigurationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_storage_configurations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .cache_storage_configurations = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `capacity_configuration`.\n"]
    pub fn set_capacity_configuration(
        self,
        v: impl Into<BlockAssignable<FinspaceKxClusterCapacityConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `code`.\n"]
    pub fn set_code(self, v: impl Into<BlockAssignable<FinspaceKxClusterCodeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().code = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.code = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `database`.\n"]
    pub fn set_database(self, v: impl Into<BlockAssignable<FinspaceKxClusterDatabaseEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().database = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.database = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `savedown_storage_configuration`.\n"]
    pub fn set_savedown_storage_configuration(
        self,
        v: impl Into<BlockAssignable<FinspaceKxClusterSavedownStorageConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().savedown_storage_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .savedown_storage_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `scaling_group_configuration`.\n"]
    pub fn set_scaling_group_configuration(
        self,
        v: impl Into<BlockAssignable<FinspaceKxClusterScalingGroupConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_group_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_group_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tickerplant_log_configuration`.\n"]
    pub fn set_tickerplant_log_configuration(
        self,
        v: impl Into<BlockAssignable<FinspaceKxClusterTickerplantLogConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tickerplant_log_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .tickerplant_log_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FinspaceKxClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_configuration`.\n"]
    pub fn set_vpc_configuration(
        self,
        v: impl Into<BlockAssignable<FinspaceKxClusterVpcConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `az_mode` after provisioning.\n"]
    pub fn az_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.az_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `command_line_arguments` after provisioning.\n"]
    pub fn command_line_arguments(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.command_line_arguments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initialization_script` after provisioning.\n"]
    pub fn initialization_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initialization_script", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_modified_timestamp` after provisioning.\n"]
    pub fn last_modified_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.release_label", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_scaling_configuration` after provisioning.\n"]
    pub fn auto_scaling_configuration(
        &self,
    ) -> ListRef<FinspaceKxClusterAutoScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cache_storage_configurations` after provisioning.\n"]
    pub fn cache_storage_configurations(
        &self,
    ) -> ListRef<FinspaceKxClusterCacheStorageConfigurationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_storage_configurations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_configuration` after provisioning.\n"]
    pub fn capacity_configuration(&self) -> ListRef<FinspaceKxClusterCapacityConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> ListRef<FinspaceKxClusterCodeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<FinspaceKxClusterDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `savedown_storage_configuration` after provisioning.\n"]
    pub fn savedown_storage_configuration(
        &self,
    ) -> ListRef<FinspaceKxClusterSavedownStorageConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.savedown_storage_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_group_configuration` after provisioning.\n"]
    pub fn scaling_group_configuration(
        &self,
    ) -> ListRef<FinspaceKxClusterScalingGroupConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_group_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tickerplant_log_configuration` after provisioning.\n"]
    pub fn tickerplant_log_configuration(
        &self,
    ) -> ListRef<FinspaceKxClusterTickerplantLogConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tickerplant_log_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FinspaceKxClusterTimeoutsElRef {
        FinspaceKxClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> ListRef<FinspaceKxClusterVpcConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_configuration", self.extract_ref()),
        )
    }
}

impl Referable for FinspaceKxCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for FinspaceKxCluster {}

impl ToListMappable for FinspaceKxCluster {
    type O = ListRef<FinspaceKxClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FinspaceKxCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_finspace_kx_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFinspaceKxCluster {
    pub tf_id: String,
    #[doc = ""]
    pub az_mode: PrimField<String>,
    #[doc = ""]
    pub environment_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub release_label: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildFinspaceKxCluster {
    pub fn build(self, stack: &mut Stack) -> FinspaceKxCluster {
        let out = FinspaceKxCluster(Rc::new(FinspaceKxCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FinspaceKxClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                availability_zone_id: core::default::Default::default(),
                az_mode: self.az_mode,
                command_line_arguments: core::default::Default::default(),
                description: core::default::Default::default(),
                environment_id: self.environment_id,
                execution_role: core::default::Default::default(),
                id: core::default::Default::default(),
                initialization_script: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                release_label: self.release_label,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                auto_scaling_configuration: core::default::Default::default(),
                cache_storage_configurations: core::default::Default::default(),
                capacity_configuration: core::default::Default::default(),
                code: core::default::Default::default(),
                database: core::default::Default::default(),
                savedown_storage_configuration: core::default::Default::default(),
                scaling_group_configuration: core::default::Default::default(),
                tickerplant_log_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FinspaceKxClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl FinspaceKxClusterRef {
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

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `az_mode` after provisioning.\n"]
    pub fn az_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.az_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `command_line_arguments` after provisioning.\n"]
    pub fn command_line_arguments(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.command_line_arguments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initialization_script` after provisioning.\n"]
    pub fn initialization_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initialization_script", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_modified_timestamp` after provisioning.\n"]
    pub fn last_modified_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.release_label", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_scaling_configuration` after provisioning.\n"]
    pub fn auto_scaling_configuration(
        &self,
    ) -> ListRef<FinspaceKxClusterAutoScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_scaling_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cache_storage_configurations` after provisioning.\n"]
    pub fn cache_storage_configurations(
        &self,
    ) -> ListRef<FinspaceKxClusterCacheStorageConfigurationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_storage_configurations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_configuration` after provisioning.\n"]
    pub fn capacity_configuration(&self) -> ListRef<FinspaceKxClusterCapacityConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> ListRef<FinspaceKxClusterCodeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<FinspaceKxClusterDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `savedown_storage_configuration` after provisioning.\n"]
    pub fn savedown_storage_configuration(
        &self,
    ) -> ListRef<FinspaceKxClusterSavedownStorageConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.savedown_storage_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_group_configuration` after provisioning.\n"]
    pub fn scaling_group_configuration(
        &self,
    ) -> ListRef<FinspaceKxClusterScalingGroupConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_group_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tickerplant_log_configuration` after provisioning.\n"]
    pub fn tickerplant_log_configuration(
        &self,
    ) -> ListRef<FinspaceKxClusterTickerplantLogConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tickerplant_log_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FinspaceKxClusterTimeoutsElRef {
        FinspaceKxClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> ListRef<FinspaceKxClusterVpcConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct FinspaceKxClusterAutoScalingConfigurationEl {
    auto_scaling_metric: PrimField<String>,
    max_node_count: PrimField<f64>,
    metric_target: PrimField<f64>,
    min_node_count: PrimField<f64>,
    scale_in_cooldown_seconds: PrimField<f64>,
    scale_out_cooldown_seconds: PrimField<f64>,
}

impl FinspaceKxClusterAutoScalingConfigurationEl {}

impl ToListMappable for FinspaceKxClusterAutoScalingConfigurationEl {
    type O = BlockAssignable<FinspaceKxClusterAutoScalingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxClusterAutoScalingConfigurationEl {
    #[doc = ""]
    pub auto_scaling_metric: PrimField<String>,
    #[doc = ""]
    pub max_node_count: PrimField<f64>,
    #[doc = ""]
    pub metric_target: PrimField<f64>,
    #[doc = ""]
    pub min_node_count: PrimField<f64>,
    #[doc = ""]
    pub scale_in_cooldown_seconds: PrimField<f64>,
    #[doc = ""]
    pub scale_out_cooldown_seconds: PrimField<f64>,
}

impl BuildFinspaceKxClusterAutoScalingConfigurationEl {
    pub fn build(self) -> FinspaceKxClusterAutoScalingConfigurationEl {
        FinspaceKxClusterAutoScalingConfigurationEl {
            auto_scaling_metric: self.auto_scaling_metric,
            max_node_count: self.max_node_count,
            metric_target: self.metric_target,
            min_node_count: self.min_node_count,
            scale_in_cooldown_seconds: self.scale_in_cooldown_seconds,
            scale_out_cooldown_seconds: self.scale_out_cooldown_seconds,
        }
    }
}

pub struct FinspaceKxClusterAutoScalingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterAutoScalingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxClusterAutoScalingConfigurationElRef {
        FinspaceKxClusterAutoScalingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxClusterAutoScalingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_scaling_metric` after provisioning.\n"]
    pub fn auto_scaling_metric(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_scaling_metric", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_node_count` after provisioning.\n"]
    pub fn max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_node_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `metric_target` after provisioning.\n"]
    pub fn metric_target(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_target", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `min_node_count` after provisioning.\n"]
    pub fn min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `scale_in_cooldown_seconds` after provisioning.\n"]
    pub fn scale_in_cooldown_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_in_cooldown_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `scale_out_cooldown_seconds` after provisioning.\n"]
    pub fn scale_out_cooldown_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_out_cooldown_seconds", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FinspaceKxClusterCacheStorageConfigurationsEl {
    size: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl FinspaceKxClusterCacheStorageConfigurationsEl {}

impl ToListMappable for FinspaceKxClusterCacheStorageConfigurationsEl {
    type O = BlockAssignable<FinspaceKxClusterCacheStorageConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxClusterCacheStorageConfigurationsEl {
    #[doc = ""]
    pub size: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildFinspaceKxClusterCacheStorageConfigurationsEl {
    pub fn build(self) -> FinspaceKxClusterCacheStorageConfigurationsEl {
        FinspaceKxClusterCacheStorageConfigurationsEl {
            size: self.size,
            type_: self.type_,
        }
    }
}

pub struct FinspaceKxClusterCacheStorageConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterCacheStorageConfigurationsElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxClusterCacheStorageConfigurationsElRef {
        FinspaceKxClusterCacheStorageConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxClusterCacheStorageConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxClusterCapacityConfigurationEl {
    node_count: PrimField<f64>,
    node_type: PrimField<String>,
}

impl FinspaceKxClusterCapacityConfigurationEl {}

impl ToListMappable for FinspaceKxClusterCapacityConfigurationEl {
    type O = BlockAssignable<FinspaceKxClusterCapacityConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxClusterCapacityConfigurationEl {
    #[doc = ""]
    pub node_count: PrimField<f64>,
    #[doc = ""]
    pub node_type: PrimField<String>,
}

impl BuildFinspaceKxClusterCapacityConfigurationEl {
    pub fn build(self) -> FinspaceKxClusterCapacityConfigurationEl {
        FinspaceKxClusterCapacityConfigurationEl {
            node_count: self.node_count,
            node_type: self.node_type,
        }
    }
}

pub struct FinspaceKxClusterCapacityConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterCapacityConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxClusterCapacityConfigurationElRef {
        FinspaceKxClusterCapacityConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxClusterCapacityConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxClusterCodeEl {
    s3_bucket: PrimField<String>,
    s3_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_object_version: Option<PrimField<String>>,
}

impl FinspaceKxClusterCodeEl {
    #[doc = "Set the field `s3_object_version`.\n"]
    pub fn set_s3_object_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_object_version = Some(v.into());
        self
    }
}

impl ToListMappable for FinspaceKxClusterCodeEl {
    type O = BlockAssignable<FinspaceKxClusterCodeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxClusterCodeEl {
    #[doc = ""]
    pub s3_bucket: PrimField<String>,
    #[doc = ""]
    pub s3_key: PrimField<String>,
}

impl BuildFinspaceKxClusterCodeEl {
    pub fn build(self) -> FinspaceKxClusterCodeEl {
        FinspaceKxClusterCodeEl {
            s3_bucket: self.s3_bucket,
            s3_key: self.s3_key,
            s3_object_version: core::default::Default::default(),
        }
    }
}

pub struct FinspaceKxClusterCodeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterCodeElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxClusterCodeElRef {
        FinspaceKxClusterCodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxClusterCodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_key", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_object_version` after provisioning.\n"]
    pub fn s3_object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_object_version", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FinspaceKxClusterDatabaseElCacheConfigurationsEl {
    cache_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_paths: Option<SetField<PrimField<String>>>,
}

impl FinspaceKxClusterDatabaseElCacheConfigurationsEl {
    #[doc = "Set the field `db_paths`.\n"]
    pub fn set_db_paths(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.db_paths = Some(v.into());
        self
    }
}

impl ToListMappable for FinspaceKxClusterDatabaseElCacheConfigurationsEl {
    type O = BlockAssignable<FinspaceKxClusterDatabaseElCacheConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxClusterDatabaseElCacheConfigurationsEl {
    #[doc = ""]
    pub cache_type: PrimField<String>,
}

impl BuildFinspaceKxClusterDatabaseElCacheConfigurationsEl {
    pub fn build(self) -> FinspaceKxClusterDatabaseElCacheConfigurationsEl {
        FinspaceKxClusterDatabaseElCacheConfigurationsEl {
            cache_type: self.cache_type,
            db_paths: core::default::Default::default(),
        }
    }
}

pub struct FinspaceKxClusterDatabaseElCacheConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterDatabaseElCacheConfigurationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FinspaceKxClusterDatabaseElCacheConfigurationsElRef {
        FinspaceKxClusterDatabaseElCacheConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxClusterDatabaseElCacheConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_type` after provisioning.\n"]
    pub fn cache_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_type", self.base))
    }

    #[doc = "Get a reference to the value of field `db_paths` after provisioning.\n"]
    pub fn db_paths(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.db_paths", self.base))
    }
}

#[derive(Serialize, Default)]
struct FinspaceKxClusterDatabaseElDynamic {
    cache_configurations: Option<DynamicBlock<FinspaceKxClusterDatabaseElCacheConfigurationsEl>>,
}

#[derive(Serialize)]
pub struct FinspaceKxClusterDatabaseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    changeset_id: Option<PrimField<String>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataview_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_configurations: Option<Vec<FinspaceKxClusterDatabaseElCacheConfigurationsEl>>,
    dynamic: FinspaceKxClusterDatabaseElDynamic,
}

impl FinspaceKxClusterDatabaseEl {
    #[doc = "Set the field `changeset_id`.\n"]
    pub fn set_changeset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.changeset_id = Some(v.into());
        self
    }

    #[doc = "Set the field `dataview_name`.\n"]
    pub fn set_dataview_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataview_name = Some(v.into());
        self
    }

    #[doc = "Set the field `cache_configurations`.\n"]
    pub fn set_cache_configurations(
        mut self,
        v: impl Into<BlockAssignable<FinspaceKxClusterDatabaseElCacheConfigurationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_configurations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_configurations = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for FinspaceKxClusterDatabaseEl {
    type O = BlockAssignable<FinspaceKxClusterDatabaseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxClusterDatabaseEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
}

impl BuildFinspaceKxClusterDatabaseEl {
    pub fn build(self) -> FinspaceKxClusterDatabaseEl {
        FinspaceKxClusterDatabaseEl {
            changeset_id: core::default::Default::default(),
            database_name: self.database_name,
            dataview_name: core::default::Default::default(),
            cache_configurations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FinspaceKxClusterDatabaseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterDatabaseElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxClusterDatabaseElRef {
        FinspaceKxClusterDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxClusterDatabaseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `changeset_id` after provisioning.\n"]
    pub fn changeset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.changeset_id", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dataview_name` after provisioning.\n"]
    pub fn dataview_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataview_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cache_configurations` after provisioning.\n"]
    pub fn cache_configurations(
        &self,
    ) -> ListRef<FinspaceKxClusterDatabaseElCacheConfigurationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_configurations", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FinspaceKxClusterSavedownStorageConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_name: Option<PrimField<String>>,
}

impl FinspaceKxClusterSavedownStorageConfigurationEl {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_name`.\n"]
    pub fn set_volume_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_name = Some(v.into());
        self
    }
}

impl ToListMappable for FinspaceKxClusterSavedownStorageConfigurationEl {
    type O = BlockAssignable<FinspaceKxClusterSavedownStorageConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxClusterSavedownStorageConfigurationEl {}

impl BuildFinspaceKxClusterSavedownStorageConfigurationEl {
    pub fn build(self) -> FinspaceKxClusterSavedownStorageConfigurationEl {
        FinspaceKxClusterSavedownStorageConfigurationEl {
            size: core::default::Default::default(),
            type_: core::default::Default::default(),
            volume_name: core::default::Default::default(),
        }
    }
}

pub struct FinspaceKxClusterSavedownStorageConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterSavedownStorageConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FinspaceKxClusterSavedownStorageConfigurationElRef {
        FinspaceKxClusterSavedownStorageConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxClusterSavedownStorageConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_name` after provisioning.\n"]
    pub fn volume_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_name", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxClusterScalingGroupConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_limit: Option<PrimField<f64>>,
    memory_reservation: PrimField<f64>,
    node_count: PrimField<f64>,
    scaling_group_name: PrimField<String>,
}

impl FinspaceKxClusterScalingGroupConfigurationEl {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }

    #[doc = "Set the field `memory_limit`.\n"]
    pub fn set_memory_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_limit = Some(v.into());
        self
    }
}

impl ToListMappable for FinspaceKxClusterScalingGroupConfigurationEl {
    type O = BlockAssignable<FinspaceKxClusterScalingGroupConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxClusterScalingGroupConfigurationEl {
    #[doc = ""]
    pub memory_reservation: PrimField<f64>,
    #[doc = ""]
    pub node_count: PrimField<f64>,
    #[doc = ""]
    pub scaling_group_name: PrimField<String>,
}

impl BuildFinspaceKxClusterScalingGroupConfigurationEl {
    pub fn build(self) -> FinspaceKxClusterScalingGroupConfigurationEl {
        FinspaceKxClusterScalingGroupConfigurationEl {
            cpu: core::default::Default::default(),
            memory_limit: core::default::Default::default(),
            memory_reservation: self.memory_reservation,
            node_count: self.node_count,
            scaling_group_name: self.scaling_group_name,
        }
    }
}

pub struct FinspaceKxClusterScalingGroupConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterScalingGroupConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxClusterScalingGroupConfigurationElRef {
        FinspaceKxClusterScalingGroupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxClusterScalingGroupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_limit` after provisioning.\n"]
    pub fn memory_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_reservation` after provisioning.\n"]
    pub fn memory_reservation(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_reservation", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_group_name` after provisioning.\n"]
    pub fn scaling_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scaling_group_name", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FinspaceKxClusterTickerplantLogConfigurationEl {
    tickerplant_log_volumes: SetField<PrimField<String>>,
}

impl FinspaceKxClusterTickerplantLogConfigurationEl {}

impl ToListMappable for FinspaceKxClusterTickerplantLogConfigurationEl {
    type O = BlockAssignable<FinspaceKxClusterTickerplantLogConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxClusterTickerplantLogConfigurationEl {
    #[doc = ""]
    pub tickerplant_log_volumes: SetField<PrimField<String>>,
}

impl BuildFinspaceKxClusterTickerplantLogConfigurationEl {
    pub fn build(self) -> FinspaceKxClusterTickerplantLogConfigurationEl {
        FinspaceKxClusterTickerplantLogConfigurationEl {
            tickerplant_log_volumes: self.tickerplant_log_volumes,
        }
    }
}

pub struct FinspaceKxClusterTickerplantLogConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterTickerplantLogConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxClusterTickerplantLogConfigurationElRef {
        FinspaceKxClusterTickerplantLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxClusterTickerplantLogConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tickerplant_log_volumes` after provisioning.\n"]
    pub fn tickerplant_log_volumes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.tickerplant_log_volumes", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FinspaceKxClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FinspaceKxClusterTimeoutsEl {
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

impl ToListMappable for FinspaceKxClusterTimeoutsEl {
    type O = BlockAssignable<FinspaceKxClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxClusterTimeoutsEl {}

impl BuildFinspaceKxClusterTimeoutsEl {
    pub fn build(self) -> FinspaceKxClusterTimeoutsEl {
        FinspaceKxClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FinspaceKxClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxClusterTimeoutsElRef {
        FinspaceKxClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxClusterTimeoutsElRef {
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

#[derive(Serialize)]
pub struct FinspaceKxClusterVpcConfigurationEl {
    ip_address_type: PrimField<String>,
    security_group_ids: SetField<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
    vpc_id: PrimField<String>,
}

impl FinspaceKxClusterVpcConfigurationEl {}

impl ToListMappable for FinspaceKxClusterVpcConfigurationEl {
    type O = BlockAssignable<FinspaceKxClusterVpcConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxClusterVpcConfigurationEl {
    #[doc = ""]
    pub ip_address_type: PrimField<String>,
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}

impl BuildFinspaceKxClusterVpcConfigurationEl {
    pub fn build(self) -> FinspaceKxClusterVpcConfigurationEl {
        FinspaceKxClusterVpcConfigurationEl {
            ip_address_type: self.ip_address_type,
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
            vpc_id: self.vpc_id,
        }
    }
}

pub struct FinspaceKxClusterVpcConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxClusterVpcConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxClusterVpcConfigurationElRef {
        FinspaceKxClusterVpcConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxClusterVpcConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct FinspaceKxClusterDynamic {
    auto_scaling_configuration: Option<DynamicBlock<FinspaceKxClusterAutoScalingConfigurationEl>>,
    cache_storage_configurations:
        Option<DynamicBlock<FinspaceKxClusterCacheStorageConfigurationsEl>>,
    capacity_configuration: Option<DynamicBlock<FinspaceKxClusterCapacityConfigurationEl>>,
    code: Option<DynamicBlock<FinspaceKxClusterCodeEl>>,
    database: Option<DynamicBlock<FinspaceKxClusterDatabaseEl>>,
    savedown_storage_configuration:
        Option<DynamicBlock<FinspaceKxClusterSavedownStorageConfigurationEl>>,
    scaling_group_configuration: Option<DynamicBlock<FinspaceKxClusterScalingGroupConfigurationEl>>,
    tickerplant_log_configuration:
        Option<DynamicBlock<FinspaceKxClusterTickerplantLogConfigurationEl>>,
    vpc_configuration: Option<DynamicBlock<FinspaceKxClusterVpcConfigurationEl>>,
}
