use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSecretManagerRegionalSecretsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataSecretManagerRegionalSecrets_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSecretManagerRegionalSecretsData>,
}
#[derive(Clone)]
pub struct DataSecretManagerRegionalSecrets(Rc<DataSecretManagerRegionalSecrets_>);
impl DataSecretManagerRegionalSecrets {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `filter`.\nFilter string, adhering to the rules in List-operation filtering (https://cloud.google.com/secret-manager/docs/filtering).\nList only secrets matching the filter. If filter is empty, all regional secrets are listed from the specified location."]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter string, adhering to the rules in List-operation filtering (https://cloud.google.com/secret-manager/docs/filtering).\nList only secrets matching the filter. If filter is empty, all regional secrets are listed from the specified location."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> ListRef<DataSecretManagerRegionalSecretsSecretsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secrets", self.extract_ref()),
        )
    }
}
impl Referable for DataSecretManagerRegionalSecrets {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSecretManagerRegionalSecrets {}
impl ToListMappable for DataSecretManagerRegionalSecrets {
    type O = ListRef<DataSecretManagerRegionalSecretsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSecretManagerRegionalSecrets_ {
    fn extract_datasource_type(&self) -> String {
        "google_secret_manager_regional_secrets".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSecretManagerRegionalSecrets {
    pub tf_id: String,
    #[doc = ""]
    pub location: PrimField<String>,
}
impl BuildDataSecretManagerRegionalSecrets {
    pub fn build(self, stack: &mut Stack) -> DataSecretManagerRegionalSecrets {
        let out = DataSecretManagerRegionalSecrets(Rc::new(DataSecretManagerRegionalSecrets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSecretManagerRegionalSecretsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSecretManagerRegionalSecretsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerRegionalSecretsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSecretManagerRegionalSecretsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter string, adhering to the rules in List-operation filtering (https://cloud.google.com/secret-manager/docs/filtering).\nList only secrets matching the filter. If filter is empty, all regional secrets are listed from the specified location."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> ListRef<DataSecretManagerRegionalSecretsSecretsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secrets", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionEl {
    type O = BlockAssignable<DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionEl {}
impl BuildDataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionEl {
    pub fn build(self) -> DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionEl {
        DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionElRef {
        DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerRegionalSecretsSecretsElRotationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    next_rotation_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_period: Option<PrimField<String>>,
}
impl DataSecretManagerRegionalSecretsSecretsElRotationEl {
    #[doc = "Set the field `next_rotation_time`.\n"]
    pub fn set_next_rotation_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_rotation_time = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_period`.\n"]
    pub fn set_rotation_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rotation_period = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerRegionalSecretsSecretsElRotationEl {
    type O = BlockAssignable<DataSecretManagerRegionalSecretsSecretsElRotationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerRegionalSecretsSecretsElRotationEl {}
impl BuildDataSecretManagerRegionalSecretsSecretsElRotationEl {
    pub fn build(self) -> DataSecretManagerRegionalSecretsSecretsElRotationEl {
        DataSecretManagerRegionalSecretsSecretsElRotationEl {
            next_rotation_time: core::default::Default::default(),
            rotation_period: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerRegionalSecretsSecretsElRotationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerRegionalSecretsSecretsElRotationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerRegionalSecretsSecretsElRotationElRef {
        DataSecretManagerRegionalSecretsSecretsElRotationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerRegionalSecretsSecretsElRotationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `next_rotation_time` after provisioning.\n"]
    pub fn next_rotation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_rotation_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rotation_period` after provisioning.\n"]
    pub fn rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_period", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerRegionalSecretsSecretsElTopicsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataSecretManagerRegionalSecretsSecretsElTopicsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerRegionalSecretsSecretsElTopicsEl {
    type O = BlockAssignable<DataSecretManagerRegionalSecretsSecretsElTopicsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerRegionalSecretsSecretsElTopicsEl {}
impl BuildDataSecretManagerRegionalSecretsSecretsElTopicsEl {
    pub fn build(self) -> DataSecretManagerRegionalSecretsSecretsElTopicsEl {
        DataSecretManagerRegionalSecretsSecretsElTopicsEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerRegionalSecretsSecretsElTopicsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerRegionalSecretsSecretsElTopicsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerRegionalSecretsSecretsElTopicsElRef {
        DataSecretManagerRegionalSecretsSecretsElTopicsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerRegionalSecretsSecretsElTopicsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerRegionalSecretsSecretsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_encryption:
        Option<ListField<DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<ListField<DataSecretManagerRegionalSecretsSecretsElRotationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terraform_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topics: Option<ListField<DataSecretManagerRegionalSecretsSecretsElTopicsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_aliases: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_destroy_ttl: Option<PrimField<String>>,
}
impl DataSecretManagerRegionalSecretsSecretsEl {
    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v: impl Into<ListField<DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionEl>>,
    ) -> Self {
        self.customer_managed_encryption = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_annotations`.\n"]
    pub fn set_effective_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.effective_annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_labels`.\n"]
    pub fn set_effective_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.effective_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expire_time = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation`.\n"]
    pub fn set_rotation(
        mut self,
        v: impl Into<ListField<DataSecretManagerRegionalSecretsSecretsElRotationEl>>,
    ) -> Self {
        self.rotation = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_id`.\n"]
    pub fn set_secret_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_id = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.terraform_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `topics`.\n"]
    pub fn set_topics(
        mut self,
        v: impl Into<ListField<DataSecretManagerRegionalSecretsSecretsElTopicsEl>>,
    ) -> Self {
        self.topics = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `version_aliases`.\n"]
    pub fn set_version_aliases(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.version_aliases = Some(v.into());
        self
    }
    #[doc = "Set the field `version_destroy_ttl`.\n"]
    pub fn set_version_destroy_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_destroy_ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerRegionalSecretsSecretsEl {
    type O = BlockAssignable<DataSecretManagerRegionalSecretsSecretsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerRegionalSecretsSecretsEl {}
impl BuildDataSecretManagerRegionalSecretsSecretsEl {
    pub fn build(self) -> DataSecretManagerRegionalSecretsSecretsEl {
        DataSecretManagerRegionalSecretsSecretsEl {
            annotations: core::default::Default::default(),
            create_time: core::default::Default::default(),
            customer_managed_encryption: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            deletion_protection: core::default::Default::default(),
            effective_annotations: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            expire_time: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
            rotation: core::default::Default::default(),
            secret_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
            topics: core::default::Default::default(),
            ttl: core::default::Default::default(),
            version_aliases: core::default::Default::default(),
            version_destroy_ttl: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerRegionalSecretsSecretsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerRegionalSecretsSecretsElRef {
    fn new(shared: StackShared, base: String) -> DataSecretManagerRegionalSecretsSecretsElRef {
        DataSecretManagerRegionalSecretsSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerRegionalSecretsSecretsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<DataSecretManagerRegionalSecretsSecretsElCustomerManagedEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\n"]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `rotation` after provisioning.\n"]
    pub fn rotation(&self) -> ListRef<DataSecretManagerRegionalSecretsSecretsElRotationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rotation", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> ListRef<DataSecretManagerRegionalSecretsSecretsElTopicsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.topics", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `version_aliases` after provisioning.\n"]
    pub fn version_aliases(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.version_aliases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version_destroy_ttl` after provisioning.\n"]
    pub fn version_destroy_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_destroy_ttl", self.base),
        )
    }
}
