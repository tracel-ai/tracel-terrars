use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3controlStorageLensConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    config_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_lens_configuration: Option<Vec<S3controlStorageLensConfigurationStorageLensConfigurationEl>>,
    dynamic: S3controlStorageLensConfigurationDynamic,
}

struct S3controlStorageLensConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3controlStorageLensConfigurationData>,
}

#[derive(Clone)]
pub struct S3controlStorageLensConfiguration(Rc<S3controlStorageLensConfiguration_>);

impl S3controlStorageLensConfiguration {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
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

    #[doc = "Set the field `storage_lens_configuration`.\n"]
    pub fn set_storage_lens_configuration(
        self,
        v: impl Into<BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_lens_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_lens_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_lens_configuration` after provisioning.\n"]
    pub fn storage_lens_configuration(&self) -> ListRef<S3controlStorageLensConfigurationStorageLensConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_lens_configuration", self.extract_ref()))
    }
}

impl Referable for S3controlStorageLensConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for S3controlStorageLensConfiguration { }

impl ToListMappable for S3controlStorageLensConfiguration {
    type O = ListRef<S3controlStorageLensConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3controlStorageLensConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3control_storage_lens_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlStorageLensConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub config_id: PrimField<String>,
}

impl BuildS3controlStorageLensConfiguration {
    pub fn build(self, stack: &mut Stack) -> S3controlStorageLensConfiguration {
        let out = S3controlStorageLensConfiguration(Rc::new(S3controlStorageLensConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3controlStorageLensConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                config_id: self.config_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                storage_lens_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlStorageLensConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl S3controlStorageLensConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_lens_configuration` after provisioning.\n"]
    pub fn storage_lens_configuration(&self) -> ListRef<S3controlStorageLensConfigurationStorageLensConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_lens_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsEl {
    type O =
        BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsEl {
    pub fn build(self) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_depth: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_storage_bytes_percentage: Option<PrimField<f64>>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaEl {
    #[doc = "Set the field `delimiter`.\n"]
    pub fn set_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delimiter = Some(v.into());
        self
    }

    #[doc = "Set the field `max_depth`.\n"]
    pub fn set_max_depth(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_depth = Some(v.into());
        self
    }

    #[doc = "Set the field `min_storage_bytes_percentage`.\n"]
    pub fn set_min_storage_bytes_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_storage_bytes_percentage = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaEl {
            delimiter: core::default::Default::default(),
            max_depth: core::default::Default::default(),
            min_storage_bytes_percentage: core::default::Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `max_depth` after provisioning.\n"]
    pub fn max_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `min_storage_bytes_percentage` after provisioning.\n"]
    pub fn min_storage_bytes_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_storage_bytes_percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElDynamic {
    selection_criteria: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selection_criteria: Option<
        Vec<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaEl,
        >,
    >,
    dynamic: S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElDynamic,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `selection_criteria`.\n"]
    pub fn set_selection_criteria(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.selection_criteria = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.selection_criteria = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsEl {
            enabled: core::default::Default::default(),
            selection_criteria: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `selection_criteria` after provisioning.\n"]
    pub fn selection_criteria(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElSelectionCriteriaElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.selection_criteria", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElDynamic {
    storage_metrics: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_metrics: Option<
        Vec<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsEl,
        >,
    >,
    dynamic: S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElDynamic,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelEl {
    #[doc = "Set the field `storage_metrics`.\n"]
    pub fn set_storage_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_metrics = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelEl {
            storage_metrics: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `storage_metrics` after provisioning.\n"]
    pub fn storage_metrics(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElStorageMetricsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.storage_metrics", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDynamic {
    activity_metrics: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsEl,
        >,
    >,
    advanced_cost_optimization_metrics: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsEl,
        >,
    >,
    advanced_data_protection_metrics: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsEl,
        >,
    >,
    detailed_status_code_metrics: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsEl,
        >,
    >,
    prefix_level: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_metrics: Option<
        Vec<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_cost_optimization_metrics: Option<
        Vec<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_data_protection_metrics: Option<
        Vec<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    detailed_status_code_metrics: Option<
        Vec<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_level: Option<
        Vec<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelEl>,
    >,
    dynamic: S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDynamic,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelEl {
    #[doc = "Set the field `activity_metrics`.\n"]
    pub fn set_activity_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.activity_metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.activity_metrics = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_cost_optimization_metrics`.\n"]
    pub fn set_advanced_cost_optimization_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_cost_optimization_metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_cost_optimization_metrics = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_data_protection_metrics`.\n"]
    pub fn set_advanced_data_protection_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_data_protection_metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_data_protection_metrics = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `detailed_status_code_metrics`.\n"]
    pub fn set_detailed_status_code_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.detailed_status_code_metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.detailed_status_code_metrics = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prefix_level`.\n"]
    pub fn set_prefix_level(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prefix_level = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix_level = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelEl {
    type O = BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelEl {
    pub fn build(self) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelEl {
            activity_metrics: core::default::Default::default(),
            advanced_cost_optimization_metrics: core::default::Default::default(),
            advanced_data_protection_metrics: core::default::Default::default(),
            detailed_status_code_metrics: core::default::Default::default(),
            prefix_level: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `activity_metrics` after provisioning.\n"]
    pub fn activity_metrics(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElActivityMetricsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.activity_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_cost_optimization_metrics` after provisioning.\n"]
    pub fn advanced_cost_optimization_metrics(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedCostOptimizationMetricsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.advanced_cost_optimization_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_data_protection_metrics` after provisioning.\n"]
    pub fn advanced_data_protection_metrics(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElAdvancedDataProtectionMetricsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.advanced_data_protection_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `detailed_status_code_metrics` after provisioning.\n"]
    pub fn detailed_status_code_metrics(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElDetailedStatusCodeMetricsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.detailed_status_code_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_level` after provisioning.\n"]
    pub fn prefix_level(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElPrefixLevelElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.prefix_level", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDynamic {
    activity_metrics: Option<
        DynamicBlock<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsEl>,
    >,
    advanced_cost_optimization_metrics: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsEl,
        >,
    >,
    advanced_data_protection_metrics: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsEl,
        >,
    >,
    bucket_level: Option<
        DynamicBlock<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelEl>,
    >,
    detailed_status_code_metrics: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_metrics: Option<
        Vec<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_cost_optimization_metrics: Option<
        Vec<
            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_data_protection_metrics: Option<
        Vec<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_level: Option<Vec<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detailed_status_code_metrics: Option<
        Vec<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsEl>,
    >,
    dynamic: S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDynamic,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelEl {
    #[doc = "Set the field `activity_metrics`.\n"]
    pub fn set_activity_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.activity_metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.activity_metrics = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_cost_optimization_metrics`.\n"]
    pub fn set_advanced_cost_optimization_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_cost_optimization_metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_cost_optimization_metrics = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_data_protection_metrics`.\n"]
    pub fn set_advanced_data_protection_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_data_protection_metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_data_protection_metrics = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bucket_level`.\n"]
    pub fn set_bucket_level(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bucket_level = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bucket_level = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `detailed_status_code_metrics`.\n"]
    pub fn set_detailed_status_code_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.detailed_status_code_metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.detailed_status_code_metrics = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelEl {
    type O = BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelEl {
    pub fn build(self) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelEl {
            activity_metrics: core::default::Default::default(),
            advanced_cost_optimization_metrics: core::default::Default::default(),
            advanced_data_protection_metrics: core::default::Default::default(),
            bucket_level: core::default::Default::default(),
            detailed_status_code_metrics: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `activity_metrics` after provisioning.\n"]
    pub fn activity_metrics(
        &self,
    ) -> ListRef<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElActivityMetricsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.activity_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_cost_optimization_metrics` after provisioning.\n"]
    pub fn advanced_cost_optimization_metrics(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedCostOptimizationMetricsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.advanced_cost_optimization_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_data_protection_metrics` after provisioning.\n"]
    pub fn advanced_data_protection_metrics(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElAdvancedDataProtectionMetricsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.advanced_data_protection_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_level` after provisioning.\n"]
    pub fn bucket_level(
        &self,
    ) -> ListRef<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElBucketLevelElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bucket_level", self.base))
    }

    #[doc = "Get a reference to the value of field `detailed_status_code_metrics` after provisioning.\n"]
    pub fn detailed_status_code_metrics(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElDetailedStatusCodeMetricsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.detailed_status_code_metrics", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgEl {
    arn: PrimField<String>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgEl { }

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgEl {
    type O = BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgEl {
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgEl {
    pub fn build(self) -> S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgEl { arn: self.arn }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsEl {
    enabled: PrimField<bool>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsEl { }

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsEl {
    type O =
        BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsEl {
    pub fn build(self) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsEl {
            enabled: self.enabled,
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsEl {
    key_id: PrimField<String>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsEl {

}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsEl {
    #[doc = ""]
    pub key_id: PrimField<String>,
}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsEl {
            key_id: self.key_id,
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3El {}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3El {

}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3El {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3El,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3El {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3El {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3El {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3El {}
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3ElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElDynamic {
    sse_kms: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsEl,
        >,
    >,
    sse_s3: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3El,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_kms: Option<
        Vec<
            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_s3: Option<
        Vec<
            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3El,
        >,
    >,
    dynamic: S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElDynamic,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionEl {
    #[doc = "Set the field `sse_kms`.\n"]
    pub fn set_sse_kms(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sse_kms = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_kms = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sse_s3`.\n"]
    pub fn set_sse_s3(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3El,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sse_s3 = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_s3 = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionEl {
            sse_kms: core::default::Default::default(),
            sse_s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sse_kms` after provisioning.\n"]
    pub fn sse_kms(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseKmsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.sse_kms", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_s3` after provisioning.\n"]
    pub fn sse_s3(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElSseS3ElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.sse_s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElDynamic {
    encryption: Option<
        DynamicBlock<
            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationEl {
    account_id: PrimField<String>,
    arn: PrimField<String>,
    format: PrimField<String>,
    output_schema_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption: Option<
        Vec<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionEl>,
    >,
    dynamic: S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElDynamic,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption`.\n"]
    pub fn set_encryption(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationEl {
    type O =
        BlockAssignable<
            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationEl {
    #[doc = ""]
    pub account_id: PrimField<String>,
    #[doc = ""]
    pub arn: PrimField<String>,
    #[doc = ""]
    pub format: PrimField<String>,
    #[doc = ""]
    pub output_schema_version: PrimField<String>,
}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationEl {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationEl {
            account_id: self.account_id,
            arn: self.arn,
            format: self.format,
            output_schema_version: self.output_schema_version,
            prefix: core::default::Default::default(),
            encryption: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `output_schema_version` after provisioning.\n"]
    pub fn output_schema_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_schema_version", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(
        &self,
    ) -> ListRef<
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElEncryptionElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.encryption", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElDynamic {
    cloud_watch_metrics: Option<
        DynamicBlock<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsEl>,
    >,
    s3_bucket_destination: Option<
        DynamicBlock<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationEl>,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_watch_metrics: Option<
        Vec<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket_destination: Option<
        Vec<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationEl>,
    >,
    dynamic: S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElDynamic,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportEl {
    #[doc = "Set the field `cloud_watch_metrics`.\n"]
    pub fn set_cloud_watch_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_watch_metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_watch_metrics = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_bucket_destination`.\n"]
    pub fn set_s3_bucket_destination(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_bucket_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_bucket_destination = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportEl {
    type O = BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElDataExportEl {
    pub fn build(self) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportEl {
            cloud_watch_metrics: core::default::Default::default(),
            s3_bucket_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_metrics` after provisioning.\n"]
    pub fn cloud_watch_metrics(
        &self,
    ) -> ListRef<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElCloudWatchMetricsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_watch_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_destination` after provisioning.\n"]
    pub fn s3_bucket_destination(
        &self,
    ) -> ListRef<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElS3BucketDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_bucket_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElExcludeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    buckets: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<SetField<PrimField<String>>>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElExcludeEl {
    #[doc = "Set the field `buckets`.\n"]
    pub fn set_buckets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.buckets = Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElExcludeEl {
    type O = BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElExcludeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElExcludeEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElExcludeEl {
    pub fn build(self) -> S3controlStorageLensConfigurationStorageLensConfigurationElExcludeEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElExcludeEl {
            buckets: core::default::Default::default(),
            regions: core::default::Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElExcludeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElExcludeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElExcludeElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElExcludeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElExcludeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationElIncludeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    buckets: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<SetField<PrimField<String>>>,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElIncludeEl {
    #[doc = "Set the field `buckets`.\n"]
    pub fn set_buckets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.buckets = Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationElIncludeEl {
    type O = BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElIncludeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationElIncludeEl {}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationElIncludeEl {
    pub fn build(self) -> S3controlStorageLensConfigurationStorageLensConfigurationElIncludeEl {
        S3controlStorageLensConfigurationStorageLensConfigurationElIncludeEl {
            buckets: core::default::Default::default(),
            regions: core::default::Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElIncludeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElIncludeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlStorageLensConfigurationStorageLensConfigurationElIncludeElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElIncludeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElIncludeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfigurationStorageLensConfigurationElDynamic {
    account_level: Option<DynamicBlock<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelEl>>,
    aws_org: Option<DynamicBlock<S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgEl>>,
    data_export: Option<DynamicBlock<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportEl>>,
    exclude: Option<DynamicBlock<S3controlStorageLensConfigurationStorageLensConfigurationElExcludeEl>>,
    include: Option<DynamicBlock<S3controlStorageLensConfigurationStorageLensConfigurationElIncludeEl>>,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfigurationStorageLensConfigurationEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_level: Option<Vec<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_org: Option<Vec<S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_export: Option<Vec<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<Vec<S3controlStorageLensConfigurationStorageLensConfigurationElExcludeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<Vec<S3controlStorageLensConfigurationStorageLensConfigurationElIncludeEl>>,
    dynamic: S3controlStorageLensConfigurationStorageLensConfigurationElDynamic,
}

impl S3controlStorageLensConfigurationStorageLensConfigurationEl {
    #[doc = "Set the field `account_level`.\n"]
    pub fn set_account_level(
        mut self,
        v: impl Into<BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.account_level = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.account_level = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_org`.\n"]
    pub fn set_aws_org(
        mut self,
        v: impl Into<BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_org = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_org = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_export`.\n"]
    pub fn set_data_export(
        mut self,
        v: impl Into<BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_export = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_export = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude`.\n"]
    pub fn set_exclude(
        mut self,
        v: impl Into<BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElExcludeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclude = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include`.\n"]
    pub fn set_include(
        mut self,
        v: impl Into<BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationElIncludeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.include = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.include = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3controlStorageLensConfigurationStorageLensConfigurationEl {
    type O = BlockAssignable<S3controlStorageLensConfigurationStorageLensConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfigurationStorageLensConfigurationEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildS3controlStorageLensConfigurationStorageLensConfigurationEl {
    pub fn build(self) -> S3controlStorageLensConfigurationStorageLensConfigurationEl {
        S3controlStorageLensConfigurationStorageLensConfigurationEl {
            enabled: self.enabled,
            account_level: core::default::Default::default(),
            aws_org: core::default::Default::default(),
            data_export: core::default::Default::default(),
            exclude: core::default::Default::default(),
            include: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlStorageLensConfigurationStorageLensConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlStorageLensConfigurationStorageLensConfigurationElRef {
    fn new(shared: StackShared, base: String) -> S3controlStorageLensConfigurationStorageLensConfigurationElRef {
        S3controlStorageLensConfigurationStorageLensConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfigurationStorageLensConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `account_level` after provisioning.\n"]
    pub fn account_level(&self) -> ListRef<S3controlStorageLensConfigurationStorageLensConfigurationElAccountLevelElRef> {
        ListRef::new(self.shared().clone(), format!("{}.account_level", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_org` after provisioning.\n"]
    pub fn aws_org(&self) -> ListRef<S3controlStorageLensConfigurationStorageLensConfigurationElAwsOrgElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws_org", self.base))
    }

    #[doc = "Get a reference to the value of field `data_export` after provisioning.\n"]
    pub fn data_export(&self) -> ListRef<S3controlStorageLensConfigurationStorageLensConfigurationElDataExportElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_export", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> ListRef<S3controlStorageLensConfigurationStorageLensConfigurationElExcludeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> ListRef<S3controlStorageLensConfigurationStorageLensConfigurationElIncludeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfigurationDynamic {
    storage_lens_configuration: Option<DynamicBlock<S3controlStorageLensConfigurationStorageLensConfigurationEl>>,
}
