use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecuritylakeDataLakeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    meta_store_manager_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<SecuritylakeDataLakeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecuritylakeDataLakeTimeoutsEl>,
    dynamic: SecuritylakeDataLakeDynamic,
}

struct SecuritylakeDataLake_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecuritylakeDataLakeData>,
}

#[derive(Clone)]
pub struct SecuritylakeDataLake(Rc<SecuritylakeDataLake_>);

impl SecuritylakeDataLake {
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(self, v: impl Into<BlockAssignable<SecuritylakeDataLakeConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SecuritylakeDataLakeTimeoutsEl>) -> Self {
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

    #[doc = "Get a reference to the value of field `meta_store_manager_role_arn` after provisioning.\n"]
    pub fn meta_store_manager_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.meta_store_manager_role_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_arn` after provisioning.\n"]
    pub fn s3_bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<SecuritylakeDataLakeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecuritylakeDataLakeTimeoutsElRef {
        SecuritylakeDataLakeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for SecuritylakeDataLake {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SecuritylakeDataLake { }

impl ToListMappable for SecuritylakeDataLake {
    type O = ListRef<SecuritylakeDataLakeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SecuritylakeDataLake_ {
    fn extract_resource_type(&self) -> String {
        "aws_securitylake_data_lake".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecuritylakeDataLake {
    pub tf_id: String,
    #[doc = ""]
    pub meta_store_manager_role_arn: PrimField<String>,
}

impl BuildSecuritylakeDataLake {
    pub fn build(self, stack: &mut Stack) -> SecuritylakeDataLake {
        let out = SecuritylakeDataLake(Rc::new(SecuritylakeDataLake_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecuritylakeDataLakeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                meta_store_manager_role_arn: self.meta_store_manager_role_arn,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecuritylakeDataLakeRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeDataLakeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SecuritylakeDataLakeRef {
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

    #[doc = "Get a reference to the value of field `meta_store_manager_role_arn` after provisioning.\n"]
    pub fn meta_store_manager_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.meta_store_manager_role_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_arn` after provisioning.\n"]
    pub fn s3_bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<SecuritylakeDataLakeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecuritylakeDataLakeTimeoutsElRef {
        SecuritylakeDataLakeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeDataLakeConfigurationElEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
}

impl SecuritylakeDataLakeConfigurationElEncryptionConfigurationEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for SecuritylakeDataLakeConfigurationElEncryptionConfigurationEl {
    type O = BlockAssignable<SecuritylakeDataLakeConfigurationElEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLakeConfigurationElEncryptionConfigurationEl {}

impl BuildSecuritylakeDataLakeConfigurationElEncryptionConfigurationEl {
    pub fn build(self) -> SecuritylakeDataLakeConfigurationElEncryptionConfigurationEl {
        SecuritylakeDataLakeConfigurationElEncryptionConfigurationEl {
            kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct SecuritylakeDataLakeConfigurationElEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeDataLakeConfigurationElEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SecuritylakeDataLakeConfigurationElEncryptionConfigurationElRef {
        SecuritylakeDataLakeConfigurationElEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLakeConfigurationElEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
}

impl SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationEl {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }
}

impl ToListMappable for SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationEl {
    type O = BlockAssignable<SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationEl {}

impl BuildSecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationEl {
    pub fn build(self) -> SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationEl {
        SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationEl {
            days: core::default::Default::default(),
        }
    }
}

pub struct SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationElRef {
        SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
}

impl SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionEl {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_class = Some(v.into());
        self
    }
}

impl ToListMappable for SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionEl {
    type O = BlockAssignable<SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionEl {}

impl BuildSecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionEl {
    pub fn build(self) -> SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionEl {
        SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionEl {
            days: core::default::Default::default(),
            storage_class: core::default::Default::default(),
        }
    }
}

pub struct SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionElRef {
        SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeDataLakeConfigurationElLifecycleConfigurationElDynamic {
    expiration: Option<DynamicBlock<SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationEl>>,
    transition: Option<DynamicBlock<SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionEl>>,
}

#[derive(Serialize)]
pub struct SecuritylakeDataLakeConfigurationElLifecycleConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration: Option<Vec<SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transition: Option<Vec<SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionEl>>,
    dynamic: SecuritylakeDataLakeConfigurationElLifecycleConfigurationElDynamic,
}

impl SecuritylakeDataLakeConfigurationElLifecycleConfigurationEl {
    #[doc = "Set the field `expiration`.\n"]
    pub fn set_expiration(
        mut self,
        v: impl Into<BlockAssignable<SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expiration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expiration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transition`.\n"]
    pub fn set_transition(
        mut self,
        v: impl Into<BlockAssignable<SecuritylakeDataLakeConfigurationElLifecycleConfigurationElTransitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transition = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SecuritylakeDataLakeConfigurationElLifecycleConfigurationEl {
    type O = BlockAssignable<SecuritylakeDataLakeConfigurationElLifecycleConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLakeConfigurationElLifecycleConfigurationEl {}

impl BuildSecuritylakeDataLakeConfigurationElLifecycleConfigurationEl {
    pub fn build(self) -> SecuritylakeDataLakeConfigurationElLifecycleConfigurationEl {
        SecuritylakeDataLakeConfigurationElLifecycleConfigurationEl {
            expiration: core::default::Default::default(),
            transition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SecuritylakeDataLakeConfigurationElLifecycleConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeDataLakeConfigurationElLifecycleConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SecuritylakeDataLakeConfigurationElLifecycleConfigurationElRef {
        SecuritylakeDataLakeConfigurationElLifecycleConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLakeConfigurationElLifecycleConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> ListRef<SecuritylakeDataLakeConfigurationElLifecycleConfigurationElExpirationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expiration", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeDataLakeConfigurationElReplicationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
}

impl SecuritylakeDataLakeConfigurationElReplicationConfigurationEl {
    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SecuritylakeDataLakeConfigurationElReplicationConfigurationEl {
    type O = BlockAssignable<SecuritylakeDataLakeConfigurationElReplicationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLakeConfigurationElReplicationConfigurationEl {}

impl BuildSecuritylakeDataLakeConfigurationElReplicationConfigurationEl {
    pub fn build(self) -> SecuritylakeDataLakeConfigurationElReplicationConfigurationEl {
        SecuritylakeDataLakeConfigurationElReplicationConfigurationEl {
            regions: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}

pub struct SecuritylakeDataLakeConfigurationElReplicationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeDataLakeConfigurationElReplicationConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SecuritylakeDataLakeConfigurationElReplicationConfigurationElRef {
        SecuritylakeDataLakeConfigurationElReplicationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLakeConfigurationElReplicationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeDataLakeConfigurationElDynamic {
    lifecycle_configuration: Option<DynamicBlock<SecuritylakeDataLakeConfigurationElLifecycleConfigurationEl>>,
    replication_configuration: Option<DynamicBlock<SecuritylakeDataLakeConfigurationElReplicationConfigurationEl>>,
}

#[derive(Serialize)]
pub struct SecuritylakeDataLakeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<ListField<SecuritylakeDataLakeConfigurationElEncryptionConfigurationEl>>,
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_configuration: Option<Vec<SecuritylakeDataLakeConfigurationElLifecycleConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_configuration: Option<Vec<SecuritylakeDataLakeConfigurationElReplicationConfigurationEl>>,
    dynamic: SecuritylakeDataLakeConfigurationElDynamic,
}

impl SecuritylakeDataLakeConfigurationEl {
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v: impl Into<ListField<SecuritylakeDataLakeConfigurationElEncryptionConfigurationEl>>,
    ) -> Self {
        self.encryption_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_configuration`.\n"]
    pub fn set_lifecycle_configuration(
        mut self,
        v: impl Into<BlockAssignable<SecuritylakeDataLakeConfigurationElLifecycleConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lifecycle_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lifecycle_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replication_configuration`.\n"]
    pub fn set_replication_configuration(
        mut self,
        v: impl Into<BlockAssignable<SecuritylakeDataLakeConfigurationElReplicationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.replication_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.replication_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SecuritylakeDataLakeConfigurationEl {
    type O = BlockAssignable<SecuritylakeDataLakeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLakeConfigurationEl {
    #[doc = ""]
    pub region: PrimField<String>,
}

impl BuildSecuritylakeDataLakeConfigurationEl {
    pub fn build(self) -> SecuritylakeDataLakeConfigurationEl {
        SecuritylakeDataLakeConfigurationEl {
            encryption_configuration: core::default::Default::default(),
            region: self.region,
            lifecycle_configuration: core::default::Default::default(),
            replication_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SecuritylakeDataLakeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeDataLakeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SecuritylakeDataLakeConfigurationElRef {
        SecuritylakeDataLakeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLakeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<SecuritylakeDataLakeConfigurationElEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_configuration` after provisioning.\n"]
    pub fn lifecycle_configuration(&self) -> ListRef<SecuritylakeDataLakeConfigurationElLifecycleConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `replication_configuration` after provisioning.\n"]
    pub fn replication_configuration(&self) -> ListRef<SecuritylakeDataLakeConfigurationElReplicationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replication_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeDataLakeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl SecuritylakeDataLakeTimeoutsEl {
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

impl ToListMappable for SecuritylakeDataLakeTimeoutsEl {
    type O = BlockAssignable<SecuritylakeDataLakeTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLakeTimeoutsEl {}

impl BuildSecuritylakeDataLakeTimeoutsEl {
    pub fn build(self) -> SecuritylakeDataLakeTimeoutsEl {
        SecuritylakeDataLakeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct SecuritylakeDataLakeTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeDataLakeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecuritylakeDataLakeTimeoutsElRef {
        SecuritylakeDataLakeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLakeTimeoutsElRef {
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
struct SecuritylakeDataLakeDynamic {
    configuration: Option<DynamicBlock<SecuritylakeDataLakeConfigurationEl>>,
}
