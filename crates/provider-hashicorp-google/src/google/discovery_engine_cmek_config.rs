use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineCmekConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cmek_config_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    kms_key: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_region_keys: Option<Vec<DiscoveryEngineCmekConfigSingleRegionKeysEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineCmekConfigTimeoutsEl>,
    dynamic: DiscoveryEngineCmekConfigDynamic,
}
struct DiscoveryEngineCmekConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineCmekConfigData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineCmekConfig(Rc<DiscoveryEngineCmekConfig_>);
impl DiscoveryEngineCmekConfig {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
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
    #[doc = "Set the field `set_default`.\nSet the following CmekConfig as the default to be used for child resources\nif one is not specified. The default value is true."]
    pub fn set_set_default(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().set_default = Some(v.into());
        self
    }
    #[doc = "Set the field `single_region_keys`.\n"]
    pub fn set_single_region_keys(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineCmekConfigSingleRegionKeysEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().single_region_keys = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.single_region_keys = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineCmekConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cmek_config_id` after provisioning.\nThe unique id of the cmek config."]
    pub fn cmek_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cmek_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_default` after provisioning.\nThe default CmekConfig for the Customer."]
    pub fn is_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_default", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nKMS key resource name which will be used to encrypt resources\n'projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}'."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_version` after provisioning.\nKMS key version resource name which will be used to encrypt resources\n'<kms_key>/cryptoKeyVersions/{keyVersion}'."]
    pub fn kms_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_rotation_timestamp_micros` after provisioning.\nThe timestamp of the last key rotation."]
    pub fn last_rotation_timestamp_micros(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_rotation_timestamp_micros", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the CMEK config should reside. The value can\nonly be one of \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the cmek config. Values are of the format\n'projects/{project}/locations/{location}/cmekConfigs/{cmek_config_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notebooklm_state` after provisioning.\nWhether the NotebookLM Corpus is ready to be used."]
    pub fn notebooklm_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebooklm_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `set_default` after provisioning.\nSet the following CmekConfig as the default to be used for child resources\nif one is not specified. The default value is true."]
    pub fn set_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.set_default", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the CmekConfig."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `single_region_keys` after provisioning.\n"]
    pub fn single_region_keys(&self) -> ListRef<DiscoveryEngineCmekConfigSingleRegionKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.single_region_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineCmekConfigTimeoutsElRef {
        DiscoveryEngineCmekConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineCmekConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineCmekConfig {}
impl ToListMappable for DiscoveryEngineCmekConfig {
    type O = ListRef<DiscoveryEngineCmekConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineCmekConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_cmek_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineCmekConfig {
    pub tf_id: String,
    #[doc = "The unique id of the cmek config."]
    pub cmek_config_id: PrimField<String>,
    #[doc = "KMS key resource name which will be used to encrypt resources\n'projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}'."]
    pub kms_key: PrimField<String>,
    #[doc = "The geographic location where the CMEK config should reside. The value can\nonly be one of \"us\" and \"eu\"."]
    pub location: PrimField<String>,
}
impl BuildDiscoveryEngineCmekConfig {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineCmekConfig {
        let out = DiscoveryEngineCmekConfig(Rc::new(DiscoveryEngineCmekConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineCmekConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cmek_config_id: self.cmek_config_id,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key: self.kms_key,
                location: self.location,
                project: core::default::Default::default(),
                set_default: core::default::Default::default(),
                single_region_keys: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineCmekConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineCmekConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineCmekConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cmek_config_id` after provisioning.\nThe unique id of the cmek config."]
    pub fn cmek_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cmek_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_default` after provisioning.\nThe default CmekConfig for the Customer."]
    pub fn is_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_default", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nKMS key resource name which will be used to encrypt resources\n'projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}'."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_version` after provisioning.\nKMS key version resource name which will be used to encrypt resources\n'<kms_key>/cryptoKeyVersions/{keyVersion}'."]
    pub fn kms_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_rotation_timestamp_micros` after provisioning.\nThe timestamp of the last key rotation."]
    pub fn last_rotation_timestamp_micros(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_rotation_timestamp_micros", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the CMEK config should reside. The value can\nonly be one of \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the cmek config. Values are of the format\n'projects/{project}/locations/{location}/cmekConfigs/{cmek_config_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notebooklm_state` after provisioning.\nWhether the NotebookLM Corpus is ready to be used."]
    pub fn notebooklm_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebooklm_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `set_default` after provisioning.\nSet the following CmekConfig as the default to be used for child resources\nif one is not specified. The default value is true."]
    pub fn set_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.set_default", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the CmekConfig."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `single_region_keys` after provisioning.\n"]
    pub fn single_region_keys(&self) -> ListRef<DiscoveryEngineCmekConfigSingleRegionKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.single_region_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineCmekConfigTimeoutsElRef {
        DiscoveryEngineCmekConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineCmekConfigSingleRegionKeysEl {
    kms_key: PrimField<String>,
}
impl DiscoveryEngineCmekConfigSingleRegionKeysEl {}
impl ToListMappable for DiscoveryEngineCmekConfigSingleRegionKeysEl {
    type O = BlockAssignable<DiscoveryEngineCmekConfigSingleRegionKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineCmekConfigSingleRegionKeysEl {
    #[doc = "Single-regional kms key resource name which will be used to encrypt\nresources\n'projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}'."]
    pub kms_key: PrimField<String>,
}
impl BuildDiscoveryEngineCmekConfigSingleRegionKeysEl {
    pub fn build(self) -> DiscoveryEngineCmekConfigSingleRegionKeysEl {
        DiscoveryEngineCmekConfigSingleRegionKeysEl {
            kms_key: self.kms_key,
        }
    }
}
pub struct DiscoveryEngineCmekConfigSingleRegionKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineCmekConfigSingleRegionKeysElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineCmekConfigSingleRegionKeysElRef {
        DiscoveryEngineCmekConfigSingleRegionKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineCmekConfigSingleRegionKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nSingle-regional kms key resource name which will be used to encrypt\nresources\n'projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}'."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineCmekConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineCmekConfigTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineCmekConfigTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineCmekConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineCmekConfigTimeoutsEl {}
impl BuildDiscoveryEngineCmekConfigTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineCmekConfigTimeoutsEl {
        DiscoveryEngineCmekConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineCmekConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineCmekConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineCmekConfigTimeoutsElRef {
        DiscoveryEngineCmekConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineCmekConfigTimeoutsElRef {
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
struct DiscoveryEngineCmekConfigDynamic {
    single_region_keys: Option<DynamicBlock<DiscoveryEngineCmekConfigSingleRegionKeysEl>>,
}
