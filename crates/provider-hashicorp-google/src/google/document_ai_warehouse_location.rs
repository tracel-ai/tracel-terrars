use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DocumentAiWarehouseLocationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    access_control_mode: PrimField<String>,
    database_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_creator_default_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    location: PrimField<String>,
    project_number: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DocumentAiWarehouseLocationTimeoutsEl>,
}
struct DocumentAiWarehouseLocation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DocumentAiWarehouseLocationData>,
}
#[derive(Clone)]
pub struct DocumentAiWarehouseLocation(Rc<DocumentAiWarehouseLocation_>);
impl DocumentAiWarehouseLocation {
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
    #[doc = "Set the field `document_creator_default_role`.\nThe default role for the person who create a document. Possible values: [\"DOCUMENT_ADMIN\", \"DOCUMENT_EDITOR\", \"DOCUMENT_VIEWER\"]"]
    pub fn set_document_creator_default_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().document_creator_default_role = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\nThe KMS key used for CMEK encryption. It is required that\nthe kms key is in the same region as the endpoint. The\nsame key will be used for all provisioned resources, if\nencryption is available. If the kmsKey is left empty, no\nencryption will be enforced."]
    pub fn set_kms_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DocumentAiWarehouseLocationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `access_control_mode` after provisioning.\nThe access control mode for accessing the customer data. Possible values: [\"ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_GCI\", \"ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_BYOID\", \"ACL_MODE_UNIVERSAL_ACCESS\"]"]
    pub fn access_control_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_control_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_type` after provisioning.\nThe type of database used to store customer data. Possible values: [\"DB_INFRA_SPANNER\", \"DB_CLOUD_SQL_POSTGRES\"]"]
    pub fn database_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `document_creator_default_role` after provisioning.\nThe default role for the person who create a document. Possible values: [\"DOCUMENT_ADMIN\", \"DOCUMENT_EDITOR\", \"DOCUMENT_VIEWER\"]"]
    pub fn document_creator_default_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_creator_default_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used for CMEK encryption. It is required that\nthe kms key is in the same region as the endpoint. The\nsame key will be used for all provisioned resources, if\nencryption is available. If the kmsKey is left empty, no\nencryption will be enforced."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location in which the instance is to be provisioned. It takes the form projects/{projectNumber}/locations/{location}."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_number` after provisioning.\nThe unique identifier of the project."]
    pub fn project_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocumentAiWarehouseLocationTimeoutsElRef {
        DocumentAiWarehouseLocationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DocumentAiWarehouseLocation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DocumentAiWarehouseLocation {}
impl ToListMappable for DocumentAiWarehouseLocation {
    type O = ListRef<DocumentAiWarehouseLocationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DocumentAiWarehouseLocation_ {
    fn extract_resource_type(&self) -> String {
        "google_document_ai_warehouse_location".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDocumentAiWarehouseLocation {
    pub tf_id: String,
    #[doc = "The access control mode for accessing the customer data. Possible values: [\"ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_GCI\", \"ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_BYOID\", \"ACL_MODE_UNIVERSAL_ACCESS\"]"]
    pub access_control_mode: PrimField<String>,
    #[doc = "The type of database used to store customer data. Possible values: [\"DB_INFRA_SPANNER\", \"DB_CLOUD_SQL_POSTGRES\"]"]
    pub database_type: PrimField<String>,
    #[doc = "The location in which the instance is to be provisioned. It takes the form projects/{projectNumber}/locations/{location}."]
    pub location: PrimField<String>,
    #[doc = "The unique identifier of the project."]
    pub project_number: PrimField<String>,
}
impl BuildDocumentAiWarehouseLocation {
    pub fn build(self, stack: &mut Stack) -> DocumentAiWarehouseLocation {
        let out = DocumentAiWarehouseLocation(Rc::new(DocumentAiWarehouseLocation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DocumentAiWarehouseLocationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_control_mode: self.access_control_mode,
                database_type: self.database_type,
                document_creator_default_role: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key: core::default::Default::default(),
                location: self.location,
                project_number: self.project_number,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DocumentAiWarehouseLocationRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseLocationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DocumentAiWarehouseLocationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_control_mode` after provisioning.\nThe access control mode for accessing the customer data. Possible values: [\"ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_GCI\", \"ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_BYOID\", \"ACL_MODE_UNIVERSAL_ACCESS\"]"]
    pub fn access_control_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_control_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_type` after provisioning.\nThe type of database used to store customer data. Possible values: [\"DB_INFRA_SPANNER\", \"DB_CLOUD_SQL_POSTGRES\"]"]
    pub fn database_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `document_creator_default_role` after provisioning.\nThe default role for the person who create a document. Possible values: [\"DOCUMENT_ADMIN\", \"DOCUMENT_EDITOR\", \"DOCUMENT_VIEWER\"]"]
    pub fn document_creator_default_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_creator_default_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used for CMEK encryption. It is required that\nthe kms key is in the same region as the endpoint. The\nsame key will be used for all provisioned resources, if\nencryption is available. If the kmsKey is left empty, no\nencryption will be enforced."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location in which the instance is to be provisioned. It takes the form projects/{projectNumber}/locations/{location}."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_number` after provisioning.\nThe unique identifier of the project."]
    pub fn project_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocumentAiWarehouseLocationTimeoutsElRef {
        DocumentAiWarehouseLocationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseLocationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl DocumentAiWarehouseLocationTimeoutsEl {
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
impl ToListMappable for DocumentAiWarehouseLocationTimeoutsEl {
    type O = BlockAssignable<DocumentAiWarehouseLocationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseLocationTimeoutsEl {}
impl BuildDocumentAiWarehouseLocationTimeoutsEl {
    pub fn build(self) -> DocumentAiWarehouseLocationTimeoutsEl {
        DocumentAiWarehouseLocationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct DocumentAiWarehouseLocationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseLocationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DocumentAiWarehouseLocationTimeoutsElRef {
        DocumentAiWarehouseLocationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseLocationTimeoutsElRef {
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
