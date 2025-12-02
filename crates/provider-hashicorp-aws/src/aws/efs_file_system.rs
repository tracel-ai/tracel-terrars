use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EfsFileSystemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_throughput_in_mibps: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_policy: Option<Vec<EfsFileSystemLifecyclePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protection: Option<Vec<EfsFileSystemProtectionEl>>,
    dynamic: EfsFileSystemDynamic,
}
struct EfsFileSystem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EfsFileSystemData>,
}
#[derive(Clone)]
pub struct EfsFileSystem(Rc<EfsFileSystem_>);
impl EfsFileSystem {
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
    #[doc = "Set the field `availability_zone_name`.\n"]
    pub fn set_availability_zone_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_zone_name = Some(v.into());
        self
    }
    #[doc = "Set the field `creation_token`.\n"]
    pub fn set_creation_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().creation_token = Some(v.into());
        self
    }
    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().encrypted = Some(v.into());
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
    #[doc = "Set the field `performance_mode`.\n"]
    pub fn set_performance_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().performance_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_throughput_in_mibps`.\n"]
    pub fn set_provisioned_throughput_in_mibps(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().provisioned_throughput_in_mibps = Some(v.into());
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
    #[doc = "Set the field `throughput_mode`.\n"]
    pub fn set_throughput_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().throughput_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `lifecycle_policy`.\n"]
    pub fn set_lifecycle_policy(
        self,
        v: impl Into<BlockAssignable<EfsFileSystemLifecyclePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lifecycle_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lifecycle_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `protection`.\n"]
    pub fn set_protection(self, v: impl Into<BlockAssignable<EfsFileSystemProtectionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().protection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.protection = Some(d);
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
    #[doc = "Get a reference to the value of field `availability_zone_name` after provisioning.\n"]
    pub fn availability_zone_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_token` after provisioning.\n"]
    pub fn creation_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `number_of_mount_targets` after provisioning.\n"]
    pub fn number_of_mount_targets(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_mount_targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_mode` after provisioning.\n"]
    pub fn performance_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput_in_mibps` after provisioning.\n"]
    pub fn provisioned_throughput_in_mibps(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput_in_mibps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size_in_bytes` after provisioning.\n"]
    pub fn size_in_bytes(&self) -> ListRef<EfsFileSystemSizeInBytesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.size_in_bytes", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `throughput_mode` after provisioning.\n"]
    pub fn throughput_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throughput_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_policy` after provisioning.\n"]
    pub fn lifecycle_policy(&self) -> ListRef<EfsFileSystemLifecyclePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protection` after provisioning.\n"]
    pub fn protection(&self) -> ListRef<EfsFileSystemProtectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protection", self.extract_ref()),
        )
    }
}
impl Referable for EfsFileSystem {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EfsFileSystem {}
impl ToListMappable for EfsFileSystem {
    type O = ListRef<EfsFileSystemRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EfsFileSystem_ {
    fn extract_resource_type(&self) -> String {
        "aws_efs_file_system".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEfsFileSystem {
    pub tf_id: String,
}
impl BuildEfsFileSystem {
    pub fn build(self, stack: &mut Stack) -> EfsFileSystem {
        let out = EfsFileSystem(Rc::new(EfsFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EfsFileSystemData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                availability_zone_name: core::default::Default::default(),
                creation_token: core::default::Default::default(),
                encrypted: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                performance_mode: core::default::Default::default(),
                provisioned_throughput_in_mibps: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                throughput_mode: core::default::Default::default(),
                lifecycle_policy: core::default::Default::default(),
                protection: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EfsFileSystemRef {
    shared: StackShared,
    base: String,
}
impl Ref for EfsFileSystemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EfsFileSystemRef {
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
    #[doc = "Get a reference to the value of field `availability_zone_name` after provisioning.\n"]
    pub fn availability_zone_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_token` after provisioning.\n"]
    pub fn creation_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `number_of_mount_targets` after provisioning.\n"]
    pub fn number_of_mount_targets(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_mount_targets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_mode` after provisioning.\n"]
    pub fn performance_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput_in_mibps` after provisioning.\n"]
    pub fn provisioned_throughput_in_mibps(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput_in_mibps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size_in_bytes` after provisioning.\n"]
    pub fn size_in_bytes(&self) -> ListRef<EfsFileSystemSizeInBytesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.size_in_bytes", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `throughput_mode` after provisioning.\n"]
    pub fn throughput_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throughput_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_policy` after provisioning.\n"]
    pub fn lifecycle_policy(&self) -> ListRef<EfsFileSystemLifecyclePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protection` after provisioning.\n"]
    pub fn protection(&self) -> ListRef<EfsFileSystemProtectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protection", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EfsFileSystemSizeInBytesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_in_ia: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_in_standard: Option<PrimField<f64>>,
}
impl EfsFileSystemSizeInBytesEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
    #[doc = "Set the field `value_in_ia`.\n"]
    pub fn set_value_in_ia(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value_in_ia = Some(v.into());
        self
    }
    #[doc = "Set the field `value_in_standard`.\n"]
    pub fn set_value_in_standard(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value_in_standard = Some(v.into());
        self
    }
}
impl ToListMappable for EfsFileSystemSizeInBytesEl {
    type O = BlockAssignable<EfsFileSystemSizeInBytesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEfsFileSystemSizeInBytesEl {}
impl BuildEfsFileSystemSizeInBytesEl {
    pub fn build(self) -> EfsFileSystemSizeInBytesEl {
        EfsFileSystemSizeInBytesEl {
            value: core::default::Default::default(),
            value_in_ia: core::default::Default::default(),
            value_in_standard: core::default::Default::default(),
        }
    }
}
pub struct EfsFileSystemSizeInBytesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EfsFileSystemSizeInBytesElRef {
    fn new(shared: StackShared, base: String) -> EfsFileSystemSizeInBytesElRef {
        EfsFileSystemSizeInBytesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EfsFileSystemSizeInBytesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
    #[doc = "Get a reference to the value of field `value_in_ia` after provisioning.\n"]
    pub fn value_in_ia(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_in_ia", self.base))
    }
    #[doc = "Get a reference to the value of field `value_in_standard` after provisioning.\n"]
    pub fn value_in_standard(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.value_in_standard", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EfsFileSystemLifecyclePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    transition_to_archive: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transition_to_ia: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transition_to_primary_storage_class: Option<PrimField<String>>,
}
impl EfsFileSystemLifecyclePolicyEl {
    #[doc = "Set the field `transition_to_archive`.\n"]
    pub fn set_transition_to_archive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transition_to_archive = Some(v.into());
        self
    }
    #[doc = "Set the field `transition_to_ia`.\n"]
    pub fn set_transition_to_ia(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transition_to_ia = Some(v.into());
        self
    }
    #[doc = "Set the field `transition_to_primary_storage_class`.\n"]
    pub fn set_transition_to_primary_storage_class(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.transition_to_primary_storage_class = Some(v.into());
        self
    }
}
impl ToListMappable for EfsFileSystemLifecyclePolicyEl {
    type O = BlockAssignable<EfsFileSystemLifecyclePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEfsFileSystemLifecyclePolicyEl {}
impl BuildEfsFileSystemLifecyclePolicyEl {
    pub fn build(self) -> EfsFileSystemLifecyclePolicyEl {
        EfsFileSystemLifecyclePolicyEl {
            transition_to_archive: core::default::Default::default(),
            transition_to_ia: core::default::Default::default(),
            transition_to_primary_storage_class: core::default::Default::default(),
        }
    }
}
pub struct EfsFileSystemLifecyclePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EfsFileSystemLifecyclePolicyElRef {
    fn new(shared: StackShared, base: String) -> EfsFileSystemLifecyclePolicyElRef {
        EfsFileSystemLifecyclePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EfsFileSystemLifecyclePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `transition_to_archive` after provisioning.\n"]
    pub fn transition_to_archive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transition_to_archive", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transition_to_ia` after provisioning.\n"]
    pub fn transition_to_ia(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transition_to_ia", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transition_to_primary_storage_class` after provisioning.\n"]
    pub fn transition_to_primary_storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transition_to_primary_storage_class", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct EfsFileSystemProtectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_overwrite: Option<PrimField<String>>,
}
impl EfsFileSystemProtectionEl {
    #[doc = "Set the field `replication_overwrite`.\n"]
    pub fn set_replication_overwrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replication_overwrite = Some(v.into());
        self
    }
}
impl ToListMappable for EfsFileSystemProtectionEl {
    type O = BlockAssignable<EfsFileSystemProtectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEfsFileSystemProtectionEl {}
impl BuildEfsFileSystemProtectionEl {
    pub fn build(self) -> EfsFileSystemProtectionEl {
        EfsFileSystemProtectionEl {
            replication_overwrite: core::default::Default::default(),
        }
    }
}
pub struct EfsFileSystemProtectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EfsFileSystemProtectionElRef {
    fn new(shared: StackShared, base: String) -> EfsFileSystemProtectionElRef {
        EfsFileSystemProtectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EfsFileSystemProtectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replication_overwrite` after provisioning.\n"]
    pub fn replication_overwrite(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_overwrite", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct EfsFileSystemDynamic {
    lifecycle_policy: Option<DynamicBlock<EfsFileSystemLifecyclePolicyEl>>,
    protection: Option<DynamicBlock<EfsFileSystemProtectionEl>>,
}
