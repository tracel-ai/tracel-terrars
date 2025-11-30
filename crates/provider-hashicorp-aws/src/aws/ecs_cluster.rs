use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct EcsClusterData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<EcsClusterConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_connect_defaults: Option<Vec<EcsClusterServiceConnectDefaultsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    setting: Option<Vec<EcsClusterSettingEl>>,
    dynamic: EcsClusterDynamic,
}

struct EcsCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcsClusterData>,
}

#[derive(Clone)]
pub struct EcsCluster(Rc<EcsCluster_>);

impl EcsCluster {
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl Into<BlockAssignable<EcsClusterConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `service_connect_defaults`.\n"]
    pub fn set_service_connect_defaults(
        self,
        v: impl Into<BlockAssignable<EcsClusterServiceConnectDefaultsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_connect_defaults = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_connect_defaults = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `setting`.\n"]
    pub fn set_setting(self, v: impl Into<BlockAssignable<EcsClusterSettingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.setting = Some(d);
            }
        }
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

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<EcsClusterConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_connect_defaults` after provisioning.\n"]
    pub fn service_connect_defaults(&self) -> ListRef<EcsClusterServiceConnectDefaultsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_connect_defaults", self.extract_ref()),
        )
    }
}

impl Referable for EcsCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for EcsCluster {}

impl ToListMappable for EcsCluster {
    type O = ListRef<EcsClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EcsCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecs_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsCluster {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildEcsCluster {
    pub fn build(self, stack: &mut Stack) -> EcsCluster {
        let out = EcsCluster(Rc::new(EcsCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcsClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                configuration: core::default::Default::default(),
                service_connect_defaults: core::default::Default::default(),
                setting: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl EcsClusterRef {
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

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<EcsClusterConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_connect_defaults` after provisioning.\n"]
    pub fn service_connect_defaults(&self) -> ListRef<EcsClusterServiceConnectDefaultsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_connect_defaults", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_watch_encryption_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_watch_log_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket_encryption_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key_prefix: Option<PrimField<String>>,
}

impl EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
    #[doc = "Set the field `cloud_watch_encryption_enabled`.\n"]
    pub fn set_cloud_watch_encryption_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cloud_watch_encryption_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_watch_log_group_name`.\n"]
    pub fn set_cloud_watch_log_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_watch_log_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket_encryption_enabled`.\n"]
    pub fn set_s3_bucket_encryption_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.s3_bucket_encryption_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket_name`.\n"]
    pub fn set_s3_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_bucket_name = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_key_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
    type O =
        BlockAssignable<EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {}

impl BuildEcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
    pub fn build(self) -> EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
        EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl {
            cloud_watch_encryption_enabled: core::default::Default::default(),
            cloud_watch_log_group_name: core::default::Default::default(),
            s3_bucket_encryption_enabled: core::default::Default::default(),
            s3_bucket_name: core::default::Default::default(),
            s3_key_prefix: core::default::Default::default(),
        }
    }
}

pub struct EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef {
        EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_encryption_enabled` after provisioning.\n"]
    pub fn cloud_watch_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_watch_encryption_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_name` after provisioning.\n"]
    pub fn cloud_watch_log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_watch_log_group_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_bucket_encryption_enabled` after provisioning.\n"]
    pub fn s3_bucket_encryption_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_encryption_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_key_prefix", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct EcsClusterConfigurationElExecuteCommandConfigurationElDynamic {
    log_configuration: Option<
        DynamicBlock<EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct EcsClusterConfigurationElExecuteCommandConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_configuration:
        Option<Vec<EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl>>,
    dynamic: EcsClusterConfigurationElExecuteCommandConfigurationElDynamic,
}

impl EcsClusterConfigurationElExecuteCommandConfigurationEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging = Some(v.into());
        self
    }

    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for EcsClusterConfigurationElExecuteCommandConfigurationEl {
    type O = BlockAssignable<EcsClusterConfigurationElExecuteCommandConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterConfigurationElExecuteCommandConfigurationEl {}

impl BuildEcsClusterConfigurationElExecuteCommandConfigurationEl {
    pub fn build(self) -> EcsClusterConfigurationElExecuteCommandConfigurationEl {
        EcsClusterConfigurationElExecuteCommandConfigurationEl {
            kms_key_id: core::default::Default::default(),
            logging: core::default::Default::default(),
            log_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EcsClusterConfigurationElExecuteCommandConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsClusterConfigurationElExecuteCommandConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcsClusterConfigurationElExecuteCommandConfigurationElRef {
        EcsClusterConfigurationElExecuteCommandConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterConfigurationElExecuteCommandConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(
        &self,
    ) -> ListRef<EcsClusterConfigurationElExecuteCommandConfigurationElLogConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct EcsClusterConfigurationElManagedStorageConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fargate_ephemeral_storage_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
}

impl EcsClusterConfigurationElManagedStorageConfigurationEl {
    #[doc = "Set the field `fargate_ephemeral_storage_kms_key_id`.\n"]
    pub fn set_fargate_ephemeral_storage_kms_key_id(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.fargate_ephemeral_storage_kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for EcsClusterConfigurationElManagedStorageConfigurationEl {
    type O = BlockAssignable<EcsClusterConfigurationElManagedStorageConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterConfigurationElManagedStorageConfigurationEl {}

impl BuildEcsClusterConfigurationElManagedStorageConfigurationEl {
    pub fn build(self) -> EcsClusterConfigurationElManagedStorageConfigurationEl {
        EcsClusterConfigurationElManagedStorageConfigurationEl {
            fargate_ephemeral_storage_kms_key_id: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct EcsClusterConfigurationElManagedStorageConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsClusterConfigurationElManagedStorageConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EcsClusterConfigurationElManagedStorageConfigurationElRef {
        EcsClusterConfigurationElManagedStorageConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterConfigurationElManagedStorageConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fargate_ephemeral_storage_kms_key_id` after provisioning.\n"]
    pub fn fargate_ephemeral_storage_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fargate_ephemeral_storage_kms_key_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsClusterConfigurationElDynamic {
    execute_command_configuration:
        Option<DynamicBlock<EcsClusterConfigurationElExecuteCommandConfigurationEl>>,
    managed_storage_configuration:
        Option<DynamicBlock<EcsClusterConfigurationElManagedStorageConfigurationEl>>,
}

#[derive(Serialize)]
pub struct EcsClusterConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execute_command_configuration:
        Option<Vec<EcsClusterConfigurationElExecuteCommandConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_storage_configuration:
        Option<Vec<EcsClusterConfigurationElManagedStorageConfigurationEl>>,
    dynamic: EcsClusterConfigurationElDynamic,
}

impl EcsClusterConfigurationEl {
    #[doc = "Set the field `execute_command_configuration`.\n"]
    pub fn set_execute_command_configuration(
        mut self,
        v: impl Into<BlockAssignable<EcsClusterConfigurationElExecuteCommandConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.execute_command_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.execute_command_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `managed_storage_configuration`.\n"]
    pub fn set_managed_storage_configuration(
        mut self,
        v: impl Into<BlockAssignable<EcsClusterConfigurationElManagedStorageConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.managed_storage_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_storage_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for EcsClusterConfigurationEl {
    type O = BlockAssignable<EcsClusterConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterConfigurationEl {}

impl BuildEcsClusterConfigurationEl {
    pub fn build(self) -> EcsClusterConfigurationEl {
        EcsClusterConfigurationEl {
            execute_command_configuration: core::default::Default::default(),
            managed_storage_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EcsClusterConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsClusterConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EcsClusterConfigurationElRef {
        EcsClusterConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execute_command_configuration` after provisioning.\n"]
    pub fn execute_command_configuration(
        &self,
    ) -> ListRef<EcsClusterConfigurationElExecuteCommandConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execute_command_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `managed_storage_configuration` after provisioning.\n"]
    pub fn managed_storage_configuration(
        &self,
    ) -> ListRef<EcsClusterConfigurationElManagedStorageConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_storage_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct EcsClusterServiceConnectDefaultsEl {
    namespace: PrimField<String>,
}

impl EcsClusterServiceConnectDefaultsEl {}

impl ToListMappable for EcsClusterServiceConnectDefaultsEl {
    type O = BlockAssignable<EcsClusterServiceConnectDefaultsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterServiceConnectDefaultsEl {
    #[doc = ""]
    pub namespace: PrimField<String>,
}

impl BuildEcsClusterServiceConnectDefaultsEl {
    pub fn build(self) -> EcsClusterServiceConnectDefaultsEl {
        EcsClusterServiceConnectDefaultsEl {
            namespace: self.namespace,
        }
    }
}

pub struct EcsClusterServiceConnectDefaultsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsClusterServiceConnectDefaultsElRef {
    fn new(shared: StackShared, base: String) -> EcsClusterServiceConnectDefaultsElRef {
        EcsClusterServiceConnectDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterServiceConnectDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsClusterSettingEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl EcsClusterSettingEl {}

impl ToListMappable for EcsClusterSettingEl {
    type O = BlockAssignable<EcsClusterSettingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterSettingEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildEcsClusterSettingEl {
    pub fn build(self) -> EcsClusterSettingEl {
        EcsClusterSettingEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct EcsClusterSettingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsClusterSettingElRef {
    fn new(shared: StackShared, base: String) -> EcsClusterSettingElRef {
        EcsClusterSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsClusterDynamic {
    configuration: Option<DynamicBlock<EcsClusterConfigurationEl>>,
    service_connect_defaults: Option<DynamicBlock<EcsClusterServiceConnectDefaultsEl>>,
    setting: Option<DynamicBlock<EcsClusterSettingEl>>,
}
