use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct M2EnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apply_changes_during_maintenance_window: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    engine_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_update: Option<PrimField<bool>>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_maintenance_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publicly_accessible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    high_availability_config: Option<Vec<M2EnvironmentHighAvailabilityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_configuration: Option<Vec<M2EnvironmentStorageConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<M2EnvironmentTimeoutsEl>,
    dynamic: M2EnvironmentDynamic,
}
struct M2Environment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<M2EnvironmentData>,
}
#[derive(Clone)]
pub struct M2Environment(Rc<M2Environment_>);
impl M2Environment {
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
    #[doc = "Set the field `apply_changes_during_maintenance_window`.\n"]
    pub fn set_apply_changes_during_maintenance_window(
        self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .apply_changes_during_maintenance_window = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_version = Some(v.into());
        self
    }
    #[doc = "Set the field `force_update`.\n"]
    pub fn set_force_update(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_update = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }
    #[doc = "Set the field `preferred_maintenance_window`.\n"]
    pub fn set_preferred_maintenance_window(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().preferred_maintenance_window = Some(v.into());
        self
    }
    #[doc = "Set the field `publicly_accessible`.\n"]
    pub fn set_publicly_accessible(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_accessible = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().subnet_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `high_availability_config`.\n"]
    pub fn set_high_availability_config(
        self,
        v: impl Into<BlockAssignable<M2EnvironmentHighAvailabilityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().high_availability_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.high_availability_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage_configuration`.\n"]
    pub fn set_storage_configuration(
        self,
        v: impl Into<BlockAssignable<M2EnvironmentStorageConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<M2EnvironmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `apply_changes_during_maintenance_window` after provisioning.\n"]
    pub fn apply_changes_during_maintenance_window(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.apply_changes_during_maintenance_window",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_update` after provisioning.\n"]
    pub fn force_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_update", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `high_availability_config` after provisioning.\n"]
    pub fn high_availability_config(&self) -> ListRef<M2EnvironmentHighAvailabilityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.high_availability_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_configuration` after provisioning.\n"]
    pub fn storage_configuration(&self) -> ListRef<M2EnvironmentStorageConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> M2EnvironmentTimeoutsElRef {
        M2EnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for M2Environment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for M2Environment {}
impl ToListMappable for M2Environment {
    type O = ListRef<M2EnvironmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for M2Environment_ {
    fn extract_resource_type(&self) -> String {
        "aws_m2_environment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildM2Environment {
    pub tf_id: String,
    #[doc = ""]
    pub engine_type: PrimField<String>,
    #[doc = ""]
    pub instance_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildM2Environment {
    pub fn build(self, stack: &mut Stack) -> M2Environment {
        let out = M2Environment(Rc::new(M2Environment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(M2EnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                apply_changes_during_maintenance_window: core::default::Default::default(),
                description: core::default::Default::default(),
                engine_type: self.engine_type,
                engine_version: core::default::Default::default(),
                force_update: core::default::Default::default(),
                instance_type: self.instance_type,
                kms_key_id: core::default::Default::default(),
                name: self.name,
                preferred_maintenance_window: core::default::Default::default(),
                publicly_accessible: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                subnet_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                high_availability_config: core::default::Default::default(),
                storage_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct M2EnvironmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for M2EnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl M2EnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `apply_changes_during_maintenance_window` after provisioning.\n"]
    pub fn apply_changes_during_maintenance_window(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.apply_changes_during_maintenance_window",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_update` after provisioning.\n"]
    pub fn force_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_update", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `high_availability_config` after provisioning.\n"]
    pub fn high_availability_config(&self) -> ListRef<M2EnvironmentHighAvailabilityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.high_availability_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_configuration` after provisioning.\n"]
    pub fn storage_configuration(&self) -> ListRef<M2EnvironmentStorageConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> M2EnvironmentTimeoutsElRef {
        M2EnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct M2EnvironmentHighAvailabilityConfigEl {
    desired_capacity: PrimField<f64>,
}
impl M2EnvironmentHighAvailabilityConfigEl {}
impl ToListMappable for M2EnvironmentHighAvailabilityConfigEl {
    type O = BlockAssignable<M2EnvironmentHighAvailabilityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildM2EnvironmentHighAvailabilityConfigEl {
    #[doc = ""]
    pub desired_capacity: PrimField<f64>,
}
impl BuildM2EnvironmentHighAvailabilityConfigEl {
    pub fn build(self) -> M2EnvironmentHighAvailabilityConfigEl {
        M2EnvironmentHighAvailabilityConfigEl {
            desired_capacity: self.desired_capacity,
        }
    }
}
pub struct M2EnvironmentHighAvailabilityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for M2EnvironmentHighAvailabilityConfigElRef {
    fn new(shared: StackShared, base: String) -> M2EnvironmentHighAvailabilityConfigElRef {
        M2EnvironmentHighAvailabilityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl M2EnvironmentHighAvailabilityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `desired_capacity` after provisioning.\n"]
    pub fn desired_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_capacity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct M2EnvironmentStorageConfigurationElEfsEl {
    file_system_id: PrimField<String>,
    mount_point: PrimField<String>,
}
impl M2EnvironmentStorageConfigurationElEfsEl {}
impl ToListMappable for M2EnvironmentStorageConfigurationElEfsEl {
    type O = BlockAssignable<M2EnvironmentStorageConfigurationElEfsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildM2EnvironmentStorageConfigurationElEfsEl {
    #[doc = ""]
    pub file_system_id: PrimField<String>,
    #[doc = ""]
    pub mount_point: PrimField<String>,
}
impl BuildM2EnvironmentStorageConfigurationElEfsEl {
    pub fn build(self) -> M2EnvironmentStorageConfigurationElEfsEl {
        M2EnvironmentStorageConfigurationElEfsEl {
            file_system_id: self.file_system_id,
            mount_point: self.mount_point,
        }
    }
}
pub struct M2EnvironmentStorageConfigurationElEfsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for M2EnvironmentStorageConfigurationElEfsElRef {
    fn new(shared: StackShared, base: String) -> M2EnvironmentStorageConfigurationElEfsElRef {
        M2EnvironmentStorageConfigurationElEfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl M2EnvironmentStorageConfigurationElEfsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mount_point` after provisioning.\n"]
    pub fn mount_point(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_point", self.base))
    }
}
#[derive(Serialize)]
pub struct M2EnvironmentStorageConfigurationElFsxEl {
    file_system_id: PrimField<String>,
    mount_point: PrimField<String>,
}
impl M2EnvironmentStorageConfigurationElFsxEl {}
impl ToListMappable for M2EnvironmentStorageConfigurationElFsxEl {
    type O = BlockAssignable<M2EnvironmentStorageConfigurationElFsxEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildM2EnvironmentStorageConfigurationElFsxEl {
    #[doc = ""]
    pub file_system_id: PrimField<String>,
    #[doc = ""]
    pub mount_point: PrimField<String>,
}
impl BuildM2EnvironmentStorageConfigurationElFsxEl {
    pub fn build(self) -> M2EnvironmentStorageConfigurationElFsxEl {
        M2EnvironmentStorageConfigurationElFsxEl {
            file_system_id: self.file_system_id,
            mount_point: self.mount_point,
        }
    }
}
pub struct M2EnvironmentStorageConfigurationElFsxElRef {
    shared: StackShared,
    base: String,
}
impl Ref for M2EnvironmentStorageConfigurationElFsxElRef {
    fn new(shared: StackShared, base: String) -> M2EnvironmentStorageConfigurationElFsxElRef {
        M2EnvironmentStorageConfigurationElFsxElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl M2EnvironmentStorageConfigurationElFsxElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mount_point` after provisioning.\n"]
    pub fn mount_point(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_point", self.base))
    }
}
#[derive(Serialize, Default)]
struct M2EnvironmentStorageConfigurationElDynamic {
    efs: Option<DynamicBlock<M2EnvironmentStorageConfigurationElEfsEl>>,
    fsx: Option<DynamicBlock<M2EnvironmentStorageConfigurationElFsxEl>>,
}
#[derive(Serialize)]
pub struct M2EnvironmentStorageConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    efs: Option<Vec<M2EnvironmentStorageConfigurationElEfsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fsx: Option<Vec<M2EnvironmentStorageConfigurationElFsxEl>>,
    dynamic: M2EnvironmentStorageConfigurationElDynamic,
}
impl M2EnvironmentStorageConfigurationEl {
    #[doc = "Set the field `efs`.\n"]
    pub fn set_efs(
        mut self,
        v: impl Into<BlockAssignable<M2EnvironmentStorageConfigurationElEfsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.efs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.efs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fsx`.\n"]
    pub fn set_fsx(
        mut self,
        v: impl Into<BlockAssignable<M2EnvironmentStorageConfigurationElFsxEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fsx = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fsx = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for M2EnvironmentStorageConfigurationEl {
    type O = BlockAssignable<M2EnvironmentStorageConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildM2EnvironmentStorageConfigurationEl {}
impl BuildM2EnvironmentStorageConfigurationEl {
    pub fn build(self) -> M2EnvironmentStorageConfigurationEl {
        M2EnvironmentStorageConfigurationEl {
            efs: core::default::Default::default(),
            fsx: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct M2EnvironmentStorageConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for M2EnvironmentStorageConfigurationElRef {
    fn new(shared: StackShared, base: String) -> M2EnvironmentStorageConfigurationElRef {
        M2EnvironmentStorageConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl M2EnvironmentStorageConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `efs` after provisioning.\n"]
    pub fn efs(&self) -> ListRef<M2EnvironmentStorageConfigurationElEfsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.efs", self.base))
    }
    #[doc = "Get a reference to the value of field `fsx` after provisioning.\n"]
    pub fn fsx(&self) -> ListRef<M2EnvironmentStorageConfigurationElFsxElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fsx", self.base))
    }
}
#[derive(Serialize)]
pub struct M2EnvironmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl M2EnvironmentTimeoutsEl {
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
impl ToListMappable for M2EnvironmentTimeoutsEl {
    type O = BlockAssignable<M2EnvironmentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildM2EnvironmentTimeoutsEl {}
impl BuildM2EnvironmentTimeoutsEl {
    pub fn build(self) -> M2EnvironmentTimeoutsEl {
        M2EnvironmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct M2EnvironmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for M2EnvironmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> M2EnvironmentTimeoutsElRef {
        M2EnvironmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl M2EnvironmentTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct M2EnvironmentDynamic {
    high_availability_config: Option<DynamicBlock<M2EnvironmentHighAvailabilityConfigEl>>,
    storage_configuration: Option<DynamicBlock<M2EnvironmentStorageConfigurationEl>>,
}
