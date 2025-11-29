use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DsqlClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_encryption_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_region_properties: Option<Vec<DsqlClusterMultiRegionPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DsqlClusterTimeoutsEl>,
    dynamic: DsqlClusterDynamic,
}

struct DsqlCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DsqlClusterData>,
}

#[derive(Clone)]
pub struct DsqlCluster(Rc<DsqlCluster_>);

impl DsqlCluster {
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

    #[doc = "Set the field `deletion_protection_enabled`.\n"]
    pub fn set_deletion_protection_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_encryption_key`.\n"]
    pub fn set_kms_encryption_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_encryption_key = Some(v.into());
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

    #[doc = "Set the field `multi_region_properties`.\n"]
    pub fn set_multi_region_properties(self, v: impl Into<BlockAssignable<DsqlClusterMultiRegionPropertiesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multi_region_properties = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multi_region_properties = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DsqlClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_details` after provisioning.\n"]
    pub fn encryption_details(&self) -> ListRef<DsqlClusterEncryptionDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_encryption_key` after provisioning.\n"]
    pub fn kms_encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_encryption_key", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vpc_endpoint_service_name` after provisioning.\n"]
    pub fn vpc_endpoint_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region_properties` after provisioning.\n"]
    pub fn multi_region_properties(&self) -> ListRef<DsqlClusterMultiRegionPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.multi_region_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DsqlClusterTimeoutsElRef {
        DsqlClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DsqlCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DsqlCluster { }

impl ToListMappable for DsqlCluster {
    type O = ListRef<DsqlClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DsqlCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_dsql_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDsqlCluster {
    pub tf_id: String,
}

impl BuildDsqlCluster {
    pub fn build(self, stack: &mut Stack) -> DsqlCluster {
        let out = DsqlCluster(Rc::new(DsqlCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DsqlClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_protection_enabled: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                kms_encryption_key: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                multi_region_properties: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DsqlClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DsqlClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DsqlClusterRef {
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

    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_details` after provisioning.\n"]
    pub fn encryption_details(&self) -> ListRef<DsqlClusterEncryptionDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_encryption_key` after provisioning.\n"]
    pub fn kms_encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_encryption_key", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vpc_endpoint_service_name` after provisioning.\n"]
    pub fn vpc_endpoint_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region_properties` after provisioning.\n"]
    pub fn multi_region_properties(&self) -> ListRef<DsqlClusterMultiRegionPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.multi_region_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DsqlClusterTimeoutsElRef {
        DsqlClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DsqlClusterEncryptionDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_type: Option<PrimField<String>>,
}

impl DsqlClusterEncryptionDetailsEl {
    #[doc = "Set the field `encryption_status`.\n"]
    pub fn set_encryption_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_status = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_type = Some(v.into());
        self
    }
}

impl ToListMappable for DsqlClusterEncryptionDetailsEl {
    type O = BlockAssignable<DsqlClusterEncryptionDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDsqlClusterEncryptionDetailsEl {}

impl BuildDsqlClusterEncryptionDetailsEl {
    pub fn build(self) -> DsqlClusterEncryptionDetailsEl {
        DsqlClusterEncryptionDetailsEl {
            encryption_status: core::default::Default::default(),
            encryption_type: core::default::Default::default(),
        }
    }
}

pub struct DsqlClusterEncryptionDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DsqlClusterEncryptionDetailsElRef {
    fn new(shared: StackShared, base: String) -> DsqlClusterEncryptionDetailsElRef {
        DsqlClusterEncryptionDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DsqlClusterEncryptionDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_status` after provisioning.\n"]
    pub fn encryption_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_status", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DsqlClusterMultiRegionPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    clusters: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    witness_region: Option<PrimField<String>>,
}

impl DsqlClusterMultiRegionPropertiesEl {
    #[doc = "Set the field `clusters`.\n"]
    pub fn set_clusters(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.clusters = Some(v.into());
        self
    }

    #[doc = "Set the field `witness_region`.\n"]
    pub fn set_witness_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.witness_region = Some(v.into());
        self
    }
}

impl ToListMappable for DsqlClusterMultiRegionPropertiesEl {
    type O = BlockAssignable<DsqlClusterMultiRegionPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDsqlClusterMultiRegionPropertiesEl {}

impl BuildDsqlClusterMultiRegionPropertiesEl {
    pub fn build(self) -> DsqlClusterMultiRegionPropertiesEl {
        DsqlClusterMultiRegionPropertiesEl {
            clusters: core::default::Default::default(),
            witness_region: core::default::Default::default(),
        }
    }
}

pub struct DsqlClusterMultiRegionPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DsqlClusterMultiRegionPropertiesElRef {
    fn new(shared: StackShared, base: String) -> DsqlClusterMultiRegionPropertiesElRef {
        DsqlClusterMultiRegionPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DsqlClusterMultiRegionPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `clusters` after provisioning.\n"]
    pub fn clusters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.clusters", self.base))
    }

    #[doc = "Get a reference to the value of field `witness_region` after provisioning.\n"]
    pub fn witness_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.witness_region", self.base))
    }
}

#[derive(Serialize)]
pub struct DsqlClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DsqlClusterTimeoutsEl {
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

impl ToListMappable for DsqlClusterTimeoutsEl {
    type O = BlockAssignable<DsqlClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDsqlClusterTimeoutsEl {}

impl BuildDsqlClusterTimeoutsEl {
    pub fn build(self) -> DsqlClusterTimeoutsEl {
        DsqlClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DsqlClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DsqlClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DsqlClusterTimeoutsElRef {
        DsqlClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DsqlClusterTimeoutsElRef {
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
struct DsqlClusterDynamic {
    multi_region_properties: Option<DynamicBlock<DsqlClusterMultiRegionPropertiesEl>>,
}
