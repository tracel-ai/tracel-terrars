use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudIdentityGroupLookupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_key: Option<Vec<DataCloudIdentityGroupLookupGroupKeyEl>>,
    dynamic: DataCloudIdentityGroupLookupDynamic,
}
struct DataCloudIdentityGroupLookup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudIdentityGroupLookupData>,
}
#[derive(Clone)]
pub struct DataCloudIdentityGroupLookup(Rc<DataCloudIdentityGroupLookup_>);
impl DataCloudIdentityGroupLookup {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `group_key`.\n"]
    pub fn set_group_key(
        self,
        v: impl Into<BlockAssignable<DataCloudIdentityGroupLookupGroupKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().group_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.group_key = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe [resource name](https://cloud.google.com/apis/design/resource_names) of the looked-up Group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_key` after provisioning.\n"]
    pub fn group_key(&self) -> ListRef<DataCloudIdentityGroupLookupGroupKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_key", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudIdentityGroupLookup {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudIdentityGroupLookup {}
impl ToListMappable for DataCloudIdentityGroupLookup {
    type O = ListRef<DataCloudIdentityGroupLookupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudIdentityGroupLookup_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_identity_group_lookup".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudIdentityGroupLookup {
    pub tf_id: String,
}
impl BuildDataCloudIdentityGroupLookup {
    pub fn build(self, stack: &mut Stack) -> DataCloudIdentityGroupLookup {
        let out = DataCloudIdentityGroupLookup(Rc::new(DataCloudIdentityGroupLookup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudIdentityGroupLookupData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                group_key: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudIdentityGroupLookupRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupLookupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudIdentityGroupLookupRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe [resource name](https://cloud.google.com/apis/design/resource_names) of the looked-up Group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_key` after provisioning.\n"]
    pub fn group_key(&self) -> ListRef<DataCloudIdentityGroupLookupGroupKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_key", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityGroupLookupGroupKeyEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
}
impl DataCloudIdentityGroupLookupGroupKeyEl {
    #[doc = "Set the field `namespace`.\nThe namespace in which the entity exists. If not specified, the EntityKey represents a Google-managed entity such as a Google user or a Google Group.\nIf specified, the EntityKey represents an external-identity-mapped group. The namespace must correspond to an identity source created in Admin Console and must be in the form of identitysources/{identity_source}."]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityGroupLookupGroupKeyEl {
    type O = BlockAssignable<DataCloudIdentityGroupLookupGroupKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityGroupLookupGroupKeyEl {
    #[doc = "The ID of the entity. For Google-managed entities, the id should be the email address of an existing group or user.\nFor external-identity-mapped entities, the id must be a string conforming to the Identity Source's requirements.\nMust be unique within a namespace."]
    pub id: PrimField<String>,
}
impl BuildDataCloudIdentityGroupLookupGroupKeyEl {
    pub fn build(self) -> DataCloudIdentityGroupLookupGroupKeyEl {
        DataCloudIdentityGroupLookupGroupKeyEl {
            id: self.id,
            namespace: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityGroupLookupGroupKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupLookupGroupKeyElRef {
    fn new(shared: StackShared, base: String) -> DataCloudIdentityGroupLookupGroupKeyElRef {
        DataCloudIdentityGroupLookupGroupKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityGroupLookupGroupKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the entity. For Google-managed entities, the id should be the email address of an existing group or user.\nFor external-identity-mapped entities, the id must be a string conforming to the Identity Source's requirements.\nMust be unique within a namespace."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe namespace in which the entity exists. If not specified, the EntityKey represents a Google-managed entity such as a Google user or a Google Group.\nIf specified, the EntityKey represents an external-identity-mapped group. The namespace must correspond to an identity source created in Admin Console and must be in the form of identitysources/{identity_source}."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataCloudIdentityGroupLookupDynamic {
    group_key: Option<DynamicBlock<DataCloudIdentityGroupLookupGroupKeyEl>>,
}
