use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKmsKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grant_tokens: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataKmsKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsKeyData>,
}

#[derive(Clone)]
pub struct DataKmsKey(Rc<DataKmsKey_>);

impl DataKmsKey {
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

    #[doc = "Set the field `grant_tokens`.\n"]
    pub fn set_grant_tokens(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().grant_tokens = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_hsm_cluster_id` after provisioning.\n"]
    pub fn cloud_hsm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_hsm_cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_key_store_id` after provisioning.\n"]
    pub fn custom_key_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_key_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_master_key_spec` after provisioning.\n"]
    pub fn customer_master_key_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_master_key_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_date` after provisioning.\n"]
    pub fn deletion_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_model` after provisioning.\n"]
    pub fn expiration_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_tokens` after provisioning.\n"]
    pub fn grant_tokens(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.grant_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_manager` after provisioning.\n"]
    pub fn key_manager(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_manager", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_spec` after provisioning.\n"]
    pub fn key_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_state` after provisioning.\n"]
    pub fn key_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_usage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region` after provisioning.\n"]
    pub fn multi_region(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region_configuration` after provisioning.\n"]
    pub fn multi_region_configuration(&self) -> ListRef<DataKmsKeyMultiRegionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.multi_region_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_deletion_window_in_days` after provisioning.\n"]
    pub fn pending_deletion_window_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pending_deletion_window_in_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_to` after provisioning.\n"]
    pub fn valid_to(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_key_configuration` after provisioning.\n"]
    pub fn xks_key_configuration(&self) -> ListRef<DataKmsKeyXksKeyConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.xks_key_configuration", self.extract_ref()))
    }
}

impl Referable for DataKmsKey {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataKmsKey { }

impl ToListMappable for DataKmsKey {
    type O = ListRef<DataKmsKeyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataKmsKey_ {
    fn extract_datasource_type(&self) -> String {
        "aws_kms_key".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKmsKey {
    pub tf_id: String,
    #[doc = ""]
    pub key_id: PrimField<String>,
}

impl BuildDataKmsKey {
    pub fn build(self, stack: &mut Stack) -> DataKmsKey {
        let out = DataKmsKey(Rc::new(DataKmsKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                grant_tokens: core::default::Default::default(),
                id: core::default::Default::default(),
                key_id: self.key_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKmsKeyRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKmsKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataKmsKeyRef {
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

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_hsm_cluster_id` after provisioning.\n"]
    pub fn cloud_hsm_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_hsm_cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_key_store_id` after provisioning.\n"]
    pub fn custom_key_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_key_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_master_key_spec` after provisioning.\n"]
    pub fn customer_master_key_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_master_key_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_date` after provisioning.\n"]
    pub fn deletion_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_model` after provisioning.\n"]
    pub fn expiration_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_tokens` after provisioning.\n"]
    pub fn grant_tokens(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.grant_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_manager` after provisioning.\n"]
    pub fn key_manager(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_manager", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_spec` after provisioning.\n"]
    pub fn key_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_state` after provisioning.\n"]
    pub fn key_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_usage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region` after provisioning.\n"]
    pub fn multi_region(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region_configuration` after provisioning.\n"]
    pub fn multi_region_configuration(&self) -> ListRef<DataKmsKeyMultiRegionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.multi_region_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_deletion_window_in_days` after provisioning.\n"]
    pub fn pending_deletion_window_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pending_deletion_window_in_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_to` after provisioning.\n"]
    pub fn valid_to(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_key_configuration` after provisioning.\n"]
    pub fn xks_key_configuration(&self) -> ListRef<DataKmsKeyXksKeyConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.xks_key_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

impl DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}

impl ToListMappable for DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
    type O = BlockAssignable<DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {}

impl BuildDataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
    pub fn build(self) -> DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
        DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
            arn: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef {
    fn new(shared: StackShared, base: String) -> DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef {
        DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

impl DataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}

impl ToListMappable for DataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
    type O = BlockAssignable<DataKmsKeyMultiRegionConfigurationElReplicaKeysEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKmsKeyMultiRegionConfigurationElReplicaKeysEl {}

impl BuildDataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
    pub fn build(self) -> DataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
        DataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
            arn: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef {
    fn new(shared: StackShared, base: String) -> DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef {
        DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKmsKeyMultiRegionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_region_key_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_key: Option<ListField<DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_keys: Option<ListField<DataKmsKeyMultiRegionConfigurationElReplicaKeysEl>>,
}

impl DataKmsKeyMultiRegionConfigurationEl {
    #[doc = "Set the field `multi_region_key_type`.\n"]
    pub fn set_multi_region_key_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.multi_region_key_type = Some(v.into());
        self
    }

    #[doc = "Set the field `primary_key`.\n"]
    pub fn set_primary_key(mut self, v: impl Into<ListField<DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl>>) -> Self {
        self.primary_key = Some(v.into());
        self
    }

    #[doc = "Set the field `replica_keys`.\n"]
    pub fn set_replica_keys(
        mut self,
        v: impl Into<ListField<DataKmsKeyMultiRegionConfigurationElReplicaKeysEl>>,
    ) -> Self {
        self.replica_keys = Some(v.into());
        self
    }
}

impl ToListMappable for DataKmsKeyMultiRegionConfigurationEl {
    type O = BlockAssignable<DataKmsKeyMultiRegionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKmsKeyMultiRegionConfigurationEl {}

impl BuildDataKmsKeyMultiRegionConfigurationEl {
    pub fn build(self) -> DataKmsKeyMultiRegionConfigurationEl {
        DataKmsKeyMultiRegionConfigurationEl {
            multi_region_key_type: core::default::Default::default(),
            primary_key: core::default::Default::default(),
            replica_keys: core::default::Default::default(),
        }
    }
}

pub struct DataKmsKeyMultiRegionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKmsKeyMultiRegionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataKmsKeyMultiRegionConfigurationElRef {
        DataKmsKeyMultiRegionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKmsKeyMultiRegionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `multi_region_key_type` after provisioning.\n"]
    pub fn multi_region_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_region_key_type", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_key` after provisioning.\n"]
    pub fn primary_key(&self) -> ListRef<DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_keys` after provisioning.\n"]
    pub fn replica_keys(&self) -> ListRef<DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replica_keys", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKmsKeyXksKeyConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

impl DataKmsKeyXksKeyConfigurationEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
}

impl ToListMappable for DataKmsKeyXksKeyConfigurationEl {
    type O = BlockAssignable<DataKmsKeyXksKeyConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKmsKeyXksKeyConfigurationEl {}

impl BuildDataKmsKeyXksKeyConfigurationEl {
    pub fn build(self) -> DataKmsKeyXksKeyConfigurationEl {
        DataKmsKeyXksKeyConfigurationEl { id: core::default::Default::default() }
    }
}

pub struct DataKmsKeyXksKeyConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKmsKeyXksKeyConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataKmsKeyXksKeyConfigurationElRef {
        DataKmsKeyXksKeyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKmsKeyXksKeyConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
