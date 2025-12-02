use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StoragegatewayFileSystemAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_destination_arn: Option<PrimField<String>>,
    gateway_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location_arn: PrimField<String>,
    password: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    username: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_attributes: Option<Vec<StoragegatewayFileSystemAssociationCacheAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StoragegatewayFileSystemAssociationTimeoutsEl>,
    dynamic: StoragegatewayFileSystemAssociationDynamic,
}
struct StoragegatewayFileSystemAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StoragegatewayFileSystemAssociationData>,
}
#[derive(Clone)]
pub struct StoragegatewayFileSystemAssociation(Rc<StoragegatewayFileSystemAssociation_>);
impl StoragegatewayFileSystemAssociation {
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
    #[doc = "Set the field `audit_destination_arn`.\n"]
    pub fn set_audit_destination_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().audit_destination_arn = Some(v.into());
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
    #[doc = "Set the field `cache_attributes`.\n"]
    pub fn set_cache_attributes(
        self,
        v: impl Into<BlockAssignable<StoragegatewayFileSystemAssociationCacheAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_attributes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StoragegatewayFileSystemAssociationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `audit_destination_arn` after provisioning.\n"]
    pub fn audit_destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audit_destination_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location_arn` after provisioning.\n"]
    pub fn location_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cache_attributes` after provisioning.\n"]
    pub fn cache_attributes(
        &self,
    ) -> ListRef<StoragegatewayFileSystemAssociationCacheAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewayFileSystemAssociationTimeoutsElRef {
        StoragegatewayFileSystemAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StoragegatewayFileSystemAssociation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StoragegatewayFileSystemAssociation {}
impl ToListMappable for StoragegatewayFileSystemAssociation {
    type O = ListRef<StoragegatewayFileSystemAssociationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StoragegatewayFileSystemAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_storagegateway_file_system_association".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStoragegatewayFileSystemAssociation {
    pub tf_id: String,
    #[doc = ""]
    pub gateway_arn: PrimField<String>,
    #[doc = ""]
    pub location_arn: PrimField<String>,
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildStoragegatewayFileSystemAssociation {
    pub fn build(self, stack: &mut Stack) -> StoragegatewayFileSystemAssociation {
        let out =
            StoragegatewayFileSystemAssociation(Rc::new(StoragegatewayFileSystemAssociation_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(StoragegatewayFileSystemAssociationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    audit_destination_arn: core::default::Default::default(),
                    gateway_arn: self.gateway_arn,
                    id: core::default::Default::default(),
                    location_arn: self.location_arn,
                    password: self.password,
                    region: core::default::Default::default(),
                    tags: core::default::Default::default(),
                    tags_all: core::default::Default::default(),
                    username: self.username,
                    cache_attributes: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StoragegatewayFileSystemAssociationRef {
    shared: StackShared,
    base: String,
}
impl Ref for StoragegatewayFileSystemAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StoragegatewayFileSystemAssociationRef {
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
    #[doc = "Get a reference to the value of field `audit_destination_arn` after provisioning.\n"]
    pub fn audit_destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audit_destination_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location_arn` after provisioning.\n"]
    pub fn location_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cache_attributes` after provisioning.\n"]
    pub fn cache_attributes(
        &self,
    ) -> ListRef<StoragegatewayFileSystemAssociationCacheAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewayFileSystemAssociationTimeoutsElRef {
        StoragegatewayFileSystemAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StoragegatewayFileSystemAssociationCacheAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_stale_timeout_in_seconds: Option<PrimField<f64>>,
}
impl StoragegatewayFileSystemAssociationCacheAttributesEl {
    #[doc = "Set the field `cache_stale_timeout_in_seconds`.\n"]
    pub fn set_cache_stale_timeout_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cache_stale_timeout_in_seconds = Some(v.into());
        self
    }
}
impl ToListMappable for StoragegatewayFileSystemAssociationCacheAttributesEl {
    type O = BlockAssignable<StoragegatewayFileSystemAssociationCacheAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStoragegatewayFileSystemAssociationCacheAttributesEl {}
impl BuildStoragegatewayFileSystemAssociationCacheAttributesEl {
    pub fn build(self) -> StoragegatewayFileSystemAssociationCacheAttributesEl {
        StoragegatewayFileSystemAssociationCacheAttributesEl {
            cache_stale_timeout_in_seconds: core::default::Default::default(),
        }
    }
}
pub struct StoragegatewayFileSystemAssociationCacheAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StoragegatewayFileSystemAssociationCacheAttributesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> StoragegatewayFileSystemAssociationCacheAttributesElRef {
        StoragegatewayFileSystemAssociationCacheAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StoragegatewayFileSystemAssociationCacheAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cache_stale_timeout_in_seconds` after provisioning.\n"]
    pub fn cache_stale_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cache_stale_timeout_in_seconds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct StoragegatewayFileSystemAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StoragegatewayFileSystemAssociationTimeoutsEl {
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
impl ToListMappable for StoragegatewayFileSystemAssociationTimeoutsEl {
    type O = BlockAssignable<StoragegatewayFileSystemAssociationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStoragegatewayFileSystemAssociationTimeoutsEl {}
impl BuildStoragegatewayFileSystemAssociationTimeoutsEl {
    pub fn build(self) -> StoragegatewayFileSystemAssociationTimeoutsEl {
        StoragegatewayFileSystemAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StoragegatewayFileSystemAssociationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StoragegatewayFileSystemAssociationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StoragegatewayFileSystemAssociationTimeoutsElRef {
        StoragegatewayFileSystemAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StoragegatewayFileSystemAssociationTimeoutsElRef {
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
struct StoragegatewayFileSystemAssociationDynamic {
    cache_attributes: Option<DynamicBlock<StoragegatewayFileSystemAssociationCacheAttributesEl>>,
}
