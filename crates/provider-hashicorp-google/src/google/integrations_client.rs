use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IntegrationsClientData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_sample_integrations: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_kms_config: Option<Vec<IntegrationsClientCloudKmsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IntegrationsClientTimeoutsEl>,
    dynamic: IntegrationsClientDynamic,
}
struct IntegrationsClient_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IntegrationsClientData>,
}
#[derive(Clone)]
pub struct IntegrationsClient(Rc<IntegrationsClient_>);
impl IntegrationsClient {
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
    #[doc = "Set the field `create_sample_integrations`.\nIndicates if sample integrations should be created along with provisioning."]
    pub fn set_create_sample_integrations(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_sample_integrations = Some(v.into());
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
    #[doc = "Set the field `run_as_service_account`.\nUser input run-as service account, if empty, will bring up a new default service account."]
    pub fn set_run_as_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().run_as_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_kms_config`.\n"]
    pub fn set_cloud_kms_config(
        self,
        v: impl Into<BlockAssignable<IntegrationsClientCloudKmsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_kms_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_kms_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IntegrationsClientTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_sample_integrations` after provisioning.\nIndicates if sample integrations should be created along with provisioning."]
    pub fn create_sample_integrations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_sample_integrations", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation in which client needs to be provisioned."]
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
    #[doc = "Get a reference to the value of field `run_as_service_account` after provisioning.\nUser input run-as service account, if empty, will bring up a new default service account."]
    pub fn run_as_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_as_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_kms_config` after provisioning.\n"]
    pub fn cloud_kms_config(&self) -> ListRef<IntegrationsClientCloudKmsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_kms_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationsClientTimeoutsElRef {
        IntegrationsClientTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IntegrationsClient {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IntegrationsClient {}
impl ToListMappable for IntegrationsClient {
    type O = ListRef<IntegrationsClientRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IntegrationsClient_ {
    fn extract_resource_type(&self) -> String {
        "google_integrations_client".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIntegrationsClient {
    pub tf_id: String,
    #[doc = "Location in which client needs to be provisioned."]
    pub location: PrimField<String>,
}
impl BuildIntegrationsClient {
    pub fn build(self, stack: &mut Stack) -> IntegrationsClient {
        let out = IntegrationsClient(Rc::new(IntegrationsClient_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IntegrationsClientData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                create_sample_integrations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                run_as_service_account: core::default::Default::default(),
                cloud_kms_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IntegrationsClientRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsClientRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IntegrationsClientRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_sample_integrations` after provisioning.\nIndicates if sample integrations should be created along with provisioning."]
    pub fn create_sample_integrations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_sample_integrations", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation in which client needs to be provisioned."]
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
    #[doc = "Get a reference to the value of field `run_as_service_account` after provisioning.\nUser input run-as service account, if empty, will bring up a new default service account."]
    pub fn run_as_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_as_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_kms_config` after provisioning.\n"]
    pub fn cloud_kms_config(&self) -> ListRef<IntegrationsClientCloudKmsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_kms_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationsClientTimeoutsElRef {
        IntegrationsClientTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IntegrationsClientCloudKmsConfigEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_version: Option<PrimField<String>>,
    kms_location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_project_id: Option<PrimField<String>>,
    kms_ring: PrimField<String>,
}
impl IntegrationsClientCloudKmsConfigEl {
    #[doc = "Set the field `key_version`.\nEach version of a key contains key material used for encryption or signing.\nA key's version is represented by an integer, starting at 1. To decrypt data\nor verify a signature, you must use the same key version that was used to\nencrypt or sign the data."]
    pub fn set_key_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_version = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_project_id`.\nThe Google Cloud project id of the project where the kms key stored. If empty,\nthe kms key is stored at the same project as customer's project and ecrypted\nwith CMEK, otherwise, the kms key is stored in the tenant project and\nencrypted with GMEK."]
    pub fn set_kms_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_project_id = Some(v.into());
        self
    }
}
impl ToListMappable for IntegrationsClientCloudKmsConfigEl {
    type O = BlockAssignable<IntegrationsClientCloudKmsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsClientCloudKmsConfigEl {
    #[doc = "A Cloud KMS key is a named object containing one or more key versions, along\nwith metadata for the key. A key exists on exactly one key ring tied to a\nspecific location."]
    pub key: PrimField<String>,
    #[doc = "Location name of the key ring, e.g. \"us-west1\"."]
    pub kms_location: PrimField<String>,
    #[doc = "A key ring organizes keys in a specific Google Cloud location and allows you to\nmanage access control on groups of keys. A key ring's name does not need to be\nunique across a Google Cloud project, but must be unique within a given location."]
    pub kms_ring: PrimField<String>,
}
impl BuildIntegrationsClientCloudKmsConfigEl {
    pub fn build(self) -> IntegrationsClientCloudKmsConfigEl {
        IntegrationsClientCloudKmsConfigEl {
            key: self.key,
            key_version: core::default::Default::default(),
            kms_location: self.kms_location,
            kms_project_id: core::default::Default::default(),
            kms_ring: self.kms_ring,
        }
    }
}
pub struct IntegrationsClientCloudKmsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsClientCloudKmsConfigElRef {
    fn new(shared: StackShared, base: String) -> IntegrationsClientCloudKmsConfigElRef {
        IntegrationsClientCloudKmsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsClientCloudKmsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nA Cloud KMS key is a named object containing one or more key versions, along\nwith metadata for the key. A key exists on exactly one key ring tied to a\nspecific location."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `key_version` after provisioning.\nEach version of a key contains key material used for encryption or signing.\nA key's version is represented by an integer, starting at 1. To decrypt data\nor verify a signature, you must use the same key version that was used to\nencrypt or sign the data."]
    pub fn key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_version", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_location` after provisioning.\nLocation name of the key ring, e.g. \"us-west1\"."]
    pub fn kms_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_location", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_project_id` after provisioning.\nThe Google Cloud project id of the project where the kms key stored. If empty,\nthe kms key is stored at the same project as customer's project and ecrypted\nwith CMEK, otherwise, the kms key is stored in the tenant project and\nencrypted with GMEK."]
    pub fn kms_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_project_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_ring` after provisioning.\nA key ring organizes keys in a specific Google Cloud location and allows you to\nmanage access control on groups of keys. A key ring's name does not need to be\nunique across a Google Cloud project, but must be unique within a given location."]
    pub fn kms_ring(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_ring", self.base))
    }
}
#[derive(Serialize)]
pub struct IntegrationsClientTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl IntegrationsClientTimeoutsEl {
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
}
impl ToListMappable for IntegrationsClientTimeoutsEl {
    type O = BlockAssignable<IntegrationsClientTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIntegrationsClientTimeoutsEl {}
impl BuildIntegrationsClientTimeoutsEl {
    pub fn build(self) -> IntegrationsClientTimeoutsEl {
        IntegrationsClientTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct IntegrationsClientTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IntegrationsClientTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IntegrationsClientTimeoutsElRef {
        IntegrationsClientTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IntegrationsClientTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct IntegrationsClientDynamic {
    cloud_kms_config: Option<DynamicBlock<IntegrationsClientCloudKmsConfigEl>>,
}
