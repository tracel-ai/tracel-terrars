use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamWorkforcePoolProviderKeyData {
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
    id: Option<PrimField<String>>,
    key_id: PrimField<String>,
    location: PrimField<String>,
    provider_id: PrimField<String>,
    #[serde(rename = "use")]
    use_: PrimField<String>,
    workforce_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_data: Option<Vec<IamWorkforcePoolProviderKeyKeyDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamWorkforcePoolProviderKeyTimeoutsEl>,
    dynamic: IamWorkforcePoolProviderKeyDynamic,
}
struct IamWorkforcePoolProviderKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamWorkforcePoolProviderKeyData>,
}
#[derive(Clone)]
pub struct IamWorkforcePoolProviderKey(Rc<IamWorkforcePoolProviderKey_>);
impl IamWorkforcePoolProviderKey {
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
    #[doc = "Set the field `key_data`.\n"]
    pub fn set_key_data(
        self,
        v: impl Into<BlockAssignable<IamWorkforcePoolProviderKeyKeyDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().key_data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.key_data = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IamWorkforcePoolProviderKeyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nThe time after which the key will be permanently deleted and cannot be recovered.\nNote that the key may get purged before this time if the total limit of keys per provider is exceeded."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_id` after provisioning.\nThe ID to use for the key, which becomes the final component of the resource name. This value must be 4-32 characters, and may contain the characters [a-z0-9-]."]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the key.\nFormat: 'locations/{location}/workforcePools/{workforcePoolId}/providers/{providerId}/keys/{keyId}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provider_id` after provisioning.\nThe ID of the provider."]
    pub fn provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the key."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `use_` after provisioning.\nThe purpose of the key. Possible values: [\"ENCRYPTION\"]"]
    pub fn use_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.use", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe ID of the workforce pool."]
    pub fn workforce_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workforce_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_data` after provisioning.\n"]
    pub fn key_data(&self) -> ListRef<IamWorkforcePoolProviderKeyKeyDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolProviderKeyTimeoutsElRef {
        IamWorkforcePoolProviderKeyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamWorkforcePoolProviderKey {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamWorkforcePoolProviderKey {}
impl ToListMappable for IamWorkforcePoolProviderKey {
    type O = ListRef<IamWorkforcePoolProviderKeyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamWorkforcePoolProviderKey_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_workforce_pool_provider_key".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamWorkforcePoolProviderKey {
    pub tf_id: String,
    #[doc = "The ID to use for the key, which becomes the final component of the resource name. This value must be 4-32 characters, and may contain the characters [a-z0-9-]."]
    pub key_id: PrimField<String>,
    #[doc = "The location for the resource."]
    pub location: PrimField<String>,
    #[doc = "The ID of the provider."]
    pub provider_id: PrimField<String>,
    #[doc = "The purpose of the key. Possible values: [\"ENCRYPTION\"]"]
    pub use_: PrimField<String>,
    #[doc = "The ID of the workforce pool."]
    pub workforce_pool_id: PrimField<String>,
}
impl BuildIamWorkforcePoolProviderKey {
    pub fn build(self, stack: &mut Stack) -> IamWorkforcePoolProviderKey {
        let out = IamWorkforcePoolProviderKey(Rc::new(IamWorkforcePoolProviderKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamWorkforcePoolProviderKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                key_id: self.key_id,
                location: self.location,
                provider_id: self.provider_id,
                use_: self.use_,
                workforce_pool_id: self.workforce_pool_id,
                key_data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IamWorkforcePoolProviderKeyRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkforcePoolProviderKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamWorkforcePoolProviderKeyRef {
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
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nThe time after which the key will be permanently deleted and cannot be recovered.\nNote that the key may get purged before this time if the total limit of keys per provider is exceeded."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_id` after provisioning.\nThe ID to use for the key, which becomes the final component of the resource name. This value must be 4-32 characters, and may contain the characters [a-z0-9-]."]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the key.\nFormat: 'locations/{location}/workforcePools/{workforcePoolId}/providers/{providerId}/keys/{keyId}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provider_id` after provisioning.\nThe ID of the provider."]
    pub fn provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the key."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `use_` after provisioning.\nThe purpose of the key. Possible values: [\"ENCRYPTION\"]"]
    pub fn use_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.use", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe ID of the workforce pool."]
    pub fn workforce_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workforce_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `key_data` after provisioning.\n"]
    pub fn key_data(&self) -> ListRef<IamWorkforcePoolProviderKeyKeyDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolProviderKeyTimeoutsElRef {
        IamWorkforcePoolProviderKeyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkforcePoolProviderKeyKeyDataEl {
    key_spec: PrimField<String>,
}
impl IamWorkforcePoolProviderKeyKeyDataEl {}
impl ToListMappable for IamWorkforcePoolProviderKeyKeyDataEl {
    type O = BlockAssignable<IamWorkforcePoolProviderKeyKeyDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkforcePoolProviderKeyKeyDataEl {
    #[doc = "The specifications for the key. Possible values: [\"RSA_2048\", \"RSA_3072\", \"RSA_4096\"]"]
    pub key_spec: PrimField<String>,
}
impl BuildIamWorkforcePoolProviderKeyKeyDataEl {
    pub fn build(self) -> IamWorkforcePoolProviderKeyKeyDataEl {
        IamWorkforcePoolProviderKeyKeyDataEl {
            key_spec: self.key_spec,
        }
    }
}
pub struct IamWorkforcePoolProviderKeyKeyDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkforcePoolProviderKeyKeyDataElRef {
    fn new(shared: StackShared, base: String) -> IamWorkforcePoolProviderKeyKeyDataElRef {
        IamWorkforcePoolProviderKeyKeyDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkforcePoolProviderKeyKeyDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\nThe format of the key."]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe key data. The format of the key is represented by the format field."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `key_spec` after provisioning.\nThe specifications for the key. Possible values: [\"RSA_2048\", \"RSA_3072\", \"RSA_4096\"]"]
    pub fn key_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_spec", self.base))
    }
    #[doc = "Get a reference to the value of field `not_after_time` after provisioning.\nLatest timestamp when this key is valid. Attempts to use this key after this time will fail.\nOnly present if the key data represents a X.509 certificate.\n\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn not_after_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.not_after_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `not_before_time` after provisioning.\nEarliest timestamp when this key is valid. Attempts to use this key before this time will fail.\nOnly present if the key data represents a X.509 certificate.\n\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn not_before_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.not_before_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkforcePoolProviderKeyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl IamWorkforcePoolProviderKeyTimeoutsEl {
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
impl ToListMappable for IamWorkforcePoolProviderKeyTimeoutsEl {
    type O = BlockAssignable<IamWorkforcePoolProviderKeyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkforcePoolProviderKeyTimeoutsEl {}
impl BuildIamWorkforcePoolProviderKeyTimeoutsEl {
    pub fn build(self) -> IamWorkforcePoolProviderKeyTimeoutsEl {
        IamWorkforcePoolProviderKeyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct IamWorkforcePoolProviderKeyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkforcePoolProviderKeyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamWorkforcePoolProviderKeyTimeoutsElRef {
        IamWorkforcePoolProviderKeyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkforcePoolProviderKeyTimeoutsElRef {
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
struct IamWorkforcePoolProviderKeyDynamic {
    key_data: Option<DynamicBlock<IamWorkforcePoolProviderKeyKeyDataEl>>,
}
