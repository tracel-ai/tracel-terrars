use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VectorSearchCollectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    collection_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_spec: Option<Vec<VectorSearchCollectionEncryptionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VectorSearchCollectionTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vector_schema: Option<Vec<VectorSearchCollectionVectorSchemaEl>>,
    dynamic: VectorSearchCollectionDynamic,
}
struct VectorSearchCollection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VectorSearchCollectionData>,
}
#[derive(Clone)]
pub struct VectorSearchCollection(Rc<VectorSearchCollection_>);
impl VectorSearchCollection {
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
    #[doc = "Set the field `data_schema`.\nJSON Schema for data.\nField names must contain only alphanumeric characters,\nunderscores, and hyphens."]
    pub fn set_data_schema(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_schema = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nUser-specified description of the collection"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser-specified display name of the collection"]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl Into<BlockAssignable<VectorSearchCollectionEncryptionSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VectorSearchCollectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vector_schema`.\n"]
    pub fn set_vector_schema(
        self,
        v: impl Into<BlockAssignable<VectorSearchCollectionVectorSchemaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vector_schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vector_schema = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nID of the Collection to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create time stamp"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_schema` after provisioning.\nJSON Schema for data.\nField names must contain only alphanumeric characters,\nunderscores, and hyphens."]
    pub fn data_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-specified description of the collection"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-specified display name of the collection"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update time stamp"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VectorSearchCollectionEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VectorSearchCollectionTimeoutsElRef {
        VectorSearchCollectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VectorSearchCollection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VectorSearchCollection {}
impl ToListMappable for VectorSearchCollection {
    type O = ListRef<VectorSearchCollectionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VectorSearchCollection_ {
    fn extract_resource_type(&self) -> String {
        "google_vector_search_collection".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVectorSearchCollection {
    pub tf_id: String,
    #[doc = "ID of the Collection to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub collection_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildVectorSearchCollection {
    pub fn build(self, stack: &mut Stack) -> VectorSearchCollection {
        let out = VectorSearchCollection(Rc::new(VectorSearchCollection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VectorSearchCollectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                collection_id: self.collection_id,
                data_schema: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vector_schema: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VectorSearchCollectionRef {
    shared: StackShared,
    base: String,
}
impl Ref for VectorSearchCollectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VectorSearchCollectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nID of the Collection to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create time stamp"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_schema` after provisioning.\nJSON Schema for data.\nField names must contain only alphanumeric characters,\nunderscores, and hyphens."]
    pub fn data_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-specified description of the collection"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-specified display name of the collection"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update time stamp"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VectorSearchCollectionEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VectorSearchCollectionTimeoutsElRef {
        VectorSearchCollectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VectorSearchCollectionEncryptionSpecEl {
    crypto_key_name: PrimField<String>,
}
impl VectorSearchCollectionEncryptionSpecEl {}
impl ToListMappable for VectorSearchCollectionEncryptionSpecEl {
    type O = BlockAssignable<VectorSearchCollectionEncryptionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVectorSearchCollectionEncryptionSpecEl {
    #[doc = "Resource name of the Cloud KMS key used to protect the resource.\n\nThe Cloud KMS key must be in the same region as the resource. It must have\nthe format\n'projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}'."]
    pub crypto_key_name: PrimField<String>,
}
impl BuildVectorSearchCollectionEncryptionSpecEl {
    pub fn build(self) -> VectorSearchCollectionEncryptionSpecEl {
        VectorSearchCollectionEncryptionSpecEl {
            crypto_key_name: self.crypto_key_name,
        }
    }
}
pub struct VectorSearchCollectionEncryptionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VectorSearchCollectionEncryptionSpecElRef {
    fn new(shared: StackShared, base: String) -> VectorSearchCollectionEncryptionSpecElRef {
        VectorSearchCollectionEncryptionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VectorSearchCollectionEncryptionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `crypto_key_name` after provisioning.\nResource name of the Cloud KMS key used to protect the resource.\n\nThe Cloud KMS key must be in the same region as the resource. It must have\nthe format\n'projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}'."]
    pub fn crypto_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crypto_key_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VectorSearchCollectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VectorSearchCollectionTimeoutsEl {
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
impl ToListMappable for VectorSearchCollectionTimeoutsEl {
    type O = BlockAssignable<VectorSearchCollectionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVectorSearchCollectionTimeoutsEl {}
impl BuildVectorSearchCollectionTimeoutsEl {
    pub fn build(self) -> VectorSearchCollectionTimeoutsEl {
        VectorSearchCollectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VectorSearchCollectionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VectorSearchCollectionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VectorSearchCollectionTimeoutsElRef {
        VectorSearchCollectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VectorSearchCollectionTimeoutsElRef {
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
pub struct VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigEl {
    model_id: PrimField<String>,
    task_type: PrimField<String>,
    text_template: PrimField<String>,
}
impl VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigEl {}
impl ToListMappable for VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigEl {
    type O =
        BlockAssignable<VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigEl {
    #[doc = "Required: ID of the embedding model to use. See\nhttps://cloud.google.com/vertex-ai/generative-ai/docs/learn/models#embeddings-models\nfor the list of supported models."]
    pub model_id: PrimField<String>,
    #[doc = "Possible values:\nRETRIEVAL_QUERY\nRETRIEVAL_DOCUMENT\nSEMANTIC_SIMILARITY\nCLASSIFICATION\nCLUSTERING\nQUESTION_ANSWERING\nFACT_VERIFICATION\nCODE_RETRIEVAL_QUERY"]
    pub task_type: PrimField<String>,
    #[doc = "Required: Text template for the input to the model. The template must\ncontain one or more references to fields in the DataObject, e.g.:\n\"Movie Title: {title} ---- Movie Plot: {plot}\"."]
    pub text_template: PrimField<String>,
}
impl BuildVectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigEl {
    pub fn build(self) -> VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigEl {
        VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigEl {
            model_id: self.model_id,
            task_type: self.task_type,
            text_template: self.text_template,
        }
    }
}
pub struct VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigElRef {
        VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model_id` after provisioning.\nRequired: ID of the embedding model to use. See\nhttps://cloud.google.com/vertex-ai/generative-ai/docs/learn/models#embeddings-models\nfor the list of supported models."]
    pub fn model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }
    #[doc = "Get a reference to the value of field `task_type` after provisioning.\nPossible values:\nRETRIEVAL_QUERY\nRETRIEVAL_DOCUMENT\nSEMANTIC_SIMILARITY\nCLASSIFICATION\nCLUSTERING\nQUESTION_ANSWERING\nFACT_VERIFICATION\nCODE_RETRIEVAL_QUERY"]
    pub fn task_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_type", self.base))
    }
    #[doc = "Get a reference to the value of field `text_template` after provisioning.\nRequired: Text template for the input to the model. The template must\ncontain one or more references to fields in the DataObject, e.g.:\n\"Movie Title: {title} ---- Movie Plot: {plot}\"."]
    pub fn text_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.text_template", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VectorSearchCollectionVectorSchemaElDenseVectorElDynamic {
    vertex_embedding_config: Option<
        DynamicBlock<VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct VectorSearchCollectionVectorSchemaElDenseVectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertex_embedding_config:
        Option<Vec<VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigEl>>,
    dynamic: VectorSearchCollectionVectorSchemaElDenseVectorElDynamic,
}
impl VectorSearchCollectionVectorSchemaElDenseVectorEl {
    #[doc = "Set the field `dimensions`.\nDimensionality of the vector field."]
    pub fn set_dimensions(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.dimensions = Some(v.into());
        self
    }
    #[doc = "Set the field `vertex_embedding_config`.\n"]
    pub fn set_vertex_embedding_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vertex_embedding_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vertex_embedding_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VectorSearchCollectionVectorSchemaElDenseVectorEl {
    type O = BlockAssignable<VectorSearchCollectionVectorSchemaElDenseVectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVectorSearchCollectionVectorSchemaElDenseVectorEl {}
impl BuildVectorSearchCollectionVectorSchemaElDenseVectorEl {
    pub fn build(self) -> VectorSearchCollectionVectorSchemaElDenseVectorEl {
        VectorSearchCollectionVectorSchemaElDenseVectorEl {
            dimensions: core::default::Default::default(),
            vertex_embedding_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VectorSearchCollectionVectorSchemaElDenseVectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VectorSearchCollectionVectorSchemaElDenseVectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VectorSearchCollectionVectorSchemaElDenseVectorElRef {
        VectorSearchCollectionVectorSchemaElDenseVectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VectorSearchCollectionVectorSchemaElDenseVectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\nDimensionality of the vector field."]
    pub fn dimensions(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }
    #[doc = "Get a reference to the value of field `vertex_embedding_config` after provisioning.\n"]
    pub fn vertex_embedding_config(
        &self,
    ) -> ListRef<VectorSearchCollectionVectorSchemaElDenseVectorElVertexEmbeddingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vertex_embedding_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VectorSearchCollectionVectorSchemaElSparseVectorEl {}
impl VectorSearchCollectionVectorSchemaElSparseVectorEl {}
impl ToListMappable for VectorSearchCollectionVectorSchemaElSparseVectorEl {
    type O = BlockAssignable<VectorSearchCollectionVectorSchemaElSparseVectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVectorSearchCollectionVectorSchemaElSparseVectorEl {}
impl BuildVectorSearchCollectionVectorSchemaElSparseVectorEl {
    pub fn build(self) -> VectorSearchCollectionVectorSchemaElSparseVectorEl {
        VectorSearchCollectionVectorSchemaElSparseVectorEl {}
    }
}
pub struct VectorSearchCollectionVectorSchemaElSparseVectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VectorSearchCollectionVectorSchemaElSparseVectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VectorSearchCollectionVectorSchemaElSparseVectorElRef {
        VectorSearchCollectionVectorSchemaElSparseVectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VectorSearchCollectionVectorSchemaElSparseVectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct VectorSearchCollectionVectorSchemaElDynamic {
    dense_vector: Option<DynamicBlock<VectorSearchCollectionVectorSchemaElDenseVectorEl>>,
    sparse_vector: Option<DynamicBlock<VectorSearchCollectionVectorSchemaElSparseVectorEl>>,
}
#[derive(Serialize)]
pub struct VectorSearchCollectionVectorSchemaEl {
    field_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dense_vector: Option<Vec<VectorSearchCollectionVectorSchemaElDenseVectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sparse_vector: Option<Vec<VectorSearchCollectionVectorSchemaElSparseVectorEl>>,
    dynamic: VectorSearchCollectionVectorSchemaElDynamic,
}
impl VectorSearchCollectionVectorSchemaEl {
    #[doc = "Set the field `dense_vector`.\n"]
    pub fn set_dense_vector(
        mut self,
        v: impl Into<BlockAssignable<VectorSearchCollectionVectorSchemaElDenseVectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dense_vector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dense_vector = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sparse_vector`.\n"]
    pub fn set_sparse_vector(
        mut self,
        v: impl Into<BlockAssignable<VectorSearchCollectionVectorSchemaElSparseVectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sparse_vector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sparse_vector = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VectorSearchCollectionVectorSchemaEl {
    type O = BlockAssignable<VectorSearchCollectionVectorSchemaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVectorSearchCollectionVectorSchemaEl {
    #[doc = ""]
    pub field_name: PrimField<String>,
}
impl BuildVectorSearchCollectionVectorSchemaEl {
    pub fn build(self) -> VectorSearchCollectionVectorSchemaEl {
        VectorSearchCollectionVectorSchemaEl {
            field_name: self.field_name,
            dense_vector: core::default::Default::default(),
            sparse_vector: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VectorSearchCollectionVectorSchemaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VectorSearchCollectionVectorSchemaElRef {
    fn new(shared: StackShared, base: String) -> VectorSearchCollectionVectorSchemaElRef {
        VectorSearchCollectionVectorSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VectorSearchCollectionVectorSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field_name` after provisioning.\n"]
    pub fn field_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }
    #[doc = "Get a reference to the value of field `dense_vector` after provisioning.\n"]
    pub fn dense_vector(&self) -> ListRef<VectorSearchCollectionVectorSchemaElDenseVectorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dense_vector", self.base))
    }
    #[doc = "Get a reference to the value of field `sparse_vector` after provisioning.\n"]
    pub fn sparse_vector(&self) -> ListRef<VectorSearchCollectionVectorSchemaElSparseVectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sparse_vector", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VectorSearchCollectionDynamic {
    encryption_spec: Option<DynamicBlock<VectorSearchCollectionEncryptionSpecEl>>,
    vector_schema: Option<DynamicBlock<VectorSearchCollectionVectorSchemaEl>>,
}
