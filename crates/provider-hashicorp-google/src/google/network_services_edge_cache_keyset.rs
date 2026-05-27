use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesEdgeCacheKeysetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<Vec<NetworkServicesEdgeCacheKeysetPublicKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesEdgeCacheKeysetTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_shared_keys: Option<Vec<NetworkServicesEdgeCacheKeysetValidationSharedKeysEl>>,
    dynamic: NetworkServicesEdgeCacheKeysetDynamic,
}
struct NetworkServicesEdgeCacheKeyset_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesEdgeCacheKeysetData>,
}
#[derive(Clone)]
pub struct NetworkServicesEdgeCacheKeyset(Rc<NetworkServicesEdgeCacheKeyset_>);
impl NetworkServicesEdgeCacheKeyset {
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
    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(
        self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheKeysetPublicKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().public_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.public_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesEdgeCacheKeysetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `validation_shared_keys`.\n"]
    pub fn set_validation_shared_keys(
        self,
        v: impl Into<BlockAssignable<NetworkServicesEdgeCacheKeysetValidationSharedKeysEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validation_shared_keys = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validation_shared_keys = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> ListRef<NetworkServicesEdgeCacheKeysetPublicKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheKeysetTimeoutsElRef {
        NetworkServicesEdgeCacheKeysetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validation_shared_keys` after provisioning.\n"]
    pub fn validation_shared_keys(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheKeysetValidationSharedKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_shared_keys", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesEdgeCacheKeyset {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesEdgeCacheKeyset {}
impl ToListMappable for NetworkServicesEdgeCacheKeyset {
    type O = ListRef<NetworkServicesEdgeCacheKeysetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesEdgeCacheKeyset_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_edge_cache_keyset".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesEdgeCacheKeyset {
    pub tf_id: String,
    #[doc = "Name of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheKeyset {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesEdgeCacheKeyset {
        let out = NetworkServicesEdgeCacheKeyset(Rc::new(NetworkServicesEdgeCacheKeyset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesEdgeCacheKeysetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                public_key: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                validation_shared_keys: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesEdgeCacheKeysetRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheKeysetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesEdgeCacheKeysetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> ListRef<NetworkServicesEdgeCacheKeysetPublicKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheKeysetTimeoutsElRef {
        NetworkServicesEdgeCacheKeysetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validation_shared_keys` after provisioning.\n"]
    pub fn validation_shared_keys(
        &self,
    ) -> ListRef<NetworkServicesEdgeCacheKeysetValidationSharedKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_shared_keys", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheKeysetPublicKeyEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl NetworkServicesEdgeCacheKeysetPublicKeyEl {
    #[doc = "Set the field `managed`.\nSet to true to have the CDN automatically manage this public key value."]
    pub fn set_managed(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.managed = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe base64-encoded value of the Ed25519 public key. The base64 encoding can be padded (44 bytes) or unpadded (43 bytes).\nRepresentations or encodings of the public key other than this will be rejected with an error."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesEdgeCacheKeysetPublicKeyEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheKeysetPublicKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheKeysetPublicKeyEl {
    #[doc = "The ID of the public key. The ID must be 1-63 characters long, and comply with RFC1035.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]*\nwhich means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit."]
    pub id: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheKeysetPublicKeyEl {
    pub fn build(self) -> NetworkServicesEdgeCacheKeysetPublicKeyEl {
        NetworkServicesEdgeCacheKeysetPublicKeyEl {
            id: self.id,
            managed: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheKeysetPublicKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheKeysetPublicKeyElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesEdgeCacheKeysetPublicKeyElRef {
        NetworkServicesEdgeCacheKeysetPublicKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheKeysetPublicKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the public key. The ID must be 1-63 characters long, and comply with RFC1035.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]*\nwhich means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `managed` after provisioning.\nSet to true to have the CDN automatically manage this public key value."]
    pub fn managed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe base64-encoded value of the Ed25519 public key. The base64 encoding can be padded (44 bytes) or unpadded (43 bytes).\nRepresentations or encodings of the public key other than this will be rejected with an error."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheKeysetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesEdgeCacheKeysetTimeoutsEl {
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
impl ToListMappable for NetworkServicesEdgeCacheKeysetTimeoutsEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheKeysetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheKeysetTimeoutsEl {}
impl BuildNetworkServicesEdgeCacheKeysetTimeoutsEl {
    pub fn build(self) -> NetworkServicesEdgeCacheKeysetTimeoutsEl {
        NetworkServicesEdgeCacheKeysetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesEdgeCacheKeysetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheKeysetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesEdgeCacheKeysetTimeoutsElRef {
        NetworkServicesEdgeCacheKeysetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheKeysetTimeoutsElRef {
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
#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheKeysetValidationSharedKeysEl {
    secret_version: PrimField<String>,
}
impl NetworkServicesEdgeCacheKeysetValidationSharedKeysEl {}
impl ToListMappable for NetworkServicesEdgeCacheKeysetValidationSharedKeysEl {
    type O = BlockAssignable<NetworkServicesEdgeCacheKeysetValidationSharedKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesEdgeCacheKeysetValidationSharedKeysEl {
    #[doc = "The name of the secret version in Secret Manager.\n\nThe resource name of the secret version must be in the format 'projects/*/secrets/*/versions/*' where the '*' values are replaced by the secrets themselves.\nThe secrets must be at least 16 bytes large.  The recommended secret size depends on the signature algorithm you are using.\n* If you are using HMAC-SHA1, we suggest 20-byte secrets.\n* If you are using HMAC-SHA256, we suggest 32-byte secrets.\nSee RFC 2104, Section 3 for more details on these recommendations."]
    pub secret_version: PrimField<String>,
}
impl BuildNetworkServicesEdgeCacheKeysetValidationSharedKeysEl {
    pub fn build(self) -> NetworkServicesEdgeCacheKeysetValidationSharedKeysEl {
        NetworkServicesEdgeCacheKeysetValidationSharedKeysEl {
            secret_version: self.secret_version,
        }
    }
}
pub struct NetworkServicesEdgeCacheKeysetValidationSharedKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesEdgeCacheKeysetValidationSharedKeysElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesEdgeCacheKeysetValidationSharedKeysElRef {
        NetworkServicesEdgeCacheKeysetValidationSharedKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesEdgeCacheKeysetValidationSharedKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_version` after provisioning.\nThe name of the secret version in Secret Manager.\n\nThe resource name of the secret version must be in the format 'projects/*/secrets/*/versions/*' where the '*' values are replaced by the secrets themselves.\nThe secrets must be at least 16 bytes large.  The recommended secret size depends on the signature algorithm you are using.\n* If you are using HMAC-SHA1, we suggest 20-byte secrets.\n* If you are using HMAC-SHA256, we suggest 32-byte secrets.\nSee RFC 2104, Section 3 for more details on these recommendations."]
    pub fn secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheKeysetDynamic {
    public_key: Option<DynamicBlock<NetworkServicesEdgeCacheKeysetPublicKeyEl>>,
    validation_shared_keys:
        Option<DynamicBlock<NetworkServicesEdgeCacheKeysetValidationSharedKeysEl>>,
}
