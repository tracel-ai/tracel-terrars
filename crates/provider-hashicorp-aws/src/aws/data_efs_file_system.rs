use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataEfsFileSystemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataEfsFileSystem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEfsFileSystemData>,
}
#[derive(Clone)]
pub struct DataEfsFileSystem(Rc<DataEfsFileSystem_>);
impl DataEfsFileSystem {
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
    #[doc = "Set the field `creation_token`.\n"]
    pub fn set_creation_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().creation_token = Some(v.into());
        self
    }
    #[doc = "Set the field `file_system_id`.\n"]
    pub fn set_file_system_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().file_system_id = Some(v.into());
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
    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `lifecycle_policy` after provisioning.\n"]
    pub fn lifecycle_policy(&self) -> ListRef<DataEfsFileSystemLifecyclePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_mode` after provisioning.\n"]
    pub fn performance_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protection` after provisioning.\n"]
    pub fn protection(&self) -> ListRef<DataEfsFileSystemProtectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protection", self.extract_ref()),
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
    pub fn size_in_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
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
    #[doc = "Get a reference to the value of field `throughput_mode` after provisioning.\n"]
    pub fn throughput_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throughput_mode", self.extract_ref()),
        )
    }
}
impl Referable for DataEfsFileSystem {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataEfsFileSystem {}
impl ToListMappable for DataEfsFileSystem {
    type O = ListRef<DataEfsFileSystemRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataEfsFileSystem_ {
    fn extract_datasource_type(&self) -> String {
        "aws_efs_file_system".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataEfsFileSystem {
    pub tf_id: String,
}
impl BuildDataEfsFileSystem {
    pub fn build(self, stack: &mut Stack) -> DataEfsFileSystem {
        let out = DataEfsFileSystem(Rc::new(DataEfsFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEfsFileSystemData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                creation_token: core::default::Default::default(),
                file_system_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataEfsFileSystemRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEfsFileSystemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataEfsFileSystemRef {
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
    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `lifecycle_policy` after provisioning.\n"]
    pub fn lifecycle_policy(&self) -> ListRef<DataEfsFileSystemLifecyclePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_mode` after provisioning.\n"]
    pub fn performance_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protection` after provisioning.\n"]
    pub fn protection(&self) -> ListRef<DataEfsFileSystemProtectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protection", self.extract_ref()),
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
    pub fn size_in_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
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
    #[doc = "Get a reference to the value of field `throughput_mode` after provisioning.\n"]
    pub fn throughput_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throughput_mode", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataEfsFileSystemLifecyclePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    transition_to_archive: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transition_to_ia: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transition_to_primary_storage_class: Option<PrimField<String>>,
}
impl DataEfsFileSystemLifecyclePolicyEl {
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
impl ToListMappable for DataEfsFileSystemLifecyclePolicyEl {
    type O = BlockAssignable<DataEfsFileSystemLifecyclePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEfsFileSystemLifecyclePolicyEl {}
impl BuildDataEfsFileSystemLifecyclePolicyEl {
    pub fn build(self) -> DataEfsFileSystemLifecyclePolicyEl {
        DataEfsFileSystemLifecyclePolicyEl {
            transition_to_archive: core::default::Default::default(),
            transition_to_ia: core::default::Default::default(),
            transition_to_primary_storage_class: core::default::Default::default(),
        }
    }
}
pub struct DataEfsFileSystemLifecyclePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEfsFileSystemLifecyclePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataEfsFileSystemLifecyclePolicyElRef {
        DataEfsFileSystemLifecyclePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEfsFileSystemLifecyclePolicyElRef {
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
pub struct DataEfsFileSystemProtectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_overwrite: Option<PrimField<String>>,
}
impl DataEfsFileSystemProtectionEl {
    #[doc = "Set the field `replication_overwrite`.\n"]
    pub fn set_replication_overwrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replication_overwrite = Some(v.into());
        self
    }
}
impl ToListMappable for DataEfsFileSystemProtectionEl {
    type O = BlockAssignable<DataEfsFileSystemProtectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEfsFileSystemProtectionEl {}
impl BuildDataEfsFileSystemProtectionEl {
    pub fn build(self) -> DataEfsFileSystemProtectionEl {
        DataEfsFileSystemProtectionEl {
            replication_overwrite: core::default::Default::default(),
        }
    }
}
pub struct DataEfsFileSystemProtectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEfsFileSystemProtectionElRef {
    fn new(shared: StackShared, base: String) -> DataEfsFileSystemProtectionElRef {
        DataEfsFileSystemProtectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEfsFileSystemProtectionElRef {
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
