use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataEbsSnapshotIdsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owners: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restorable_by_user_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEbsSnapshotIdsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEbsSnapshotIdsTimeoutsEl>,
    dynamic: DataEbsSnapshotIdsDynamic,
}
struct DataEbsSnapshotIds_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEbsSnapshotIdsData>,
}
#[derive(Clone)]
pub struct DataEbsSnapshotIds(Rc<DataEbsSnapshotIds_>);
impl DataEbsSnapshotIds {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `owners`.\n"]
    pub fn set_owners(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().owners = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `restorable_by_user_ids`.\n"]
    pub fn set_restorable_by_user_ids(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().restorable_by_user_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataEbsSnapshotIdsFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataEbsSnapshotIdsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `owners` after provisioning.\n"]
    pub fn owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.owners", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restorable_by_user_ids` after provisioning.\n"]
    pub fn restorable_by_user_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restorable_by_user_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEbsSnapshotIdsTimeoutsElRef {
        DataEbsSnapshotIdsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataEbsSnapshotIds {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataEbsSnapshotIds {}
impl ToListMappable for DataEbsSnapshotIds {
    type O = ListRef<DataEbsSnapshotIdsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataEbsSnapshotIds_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ebs_snapshot_ids".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataEbsSnapshotIds {
    pub tf_id: String,
}
impl BuildDataEbsSnapshotIds {
    pub fn build(self, stack: &mut Stack) -> DataEbsSnapshotIds {
        let out = DataEbsSnapshotIds(Rc::new(DataEbsSnapshotIds_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEbsSnapshotIdsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                owners: core::default::Default::default(),
                region: core::default::Default::default(),
                restorable_by_user_ids: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataEbsSnapshotIdsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEbsSnapshotIdsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataEbsSnapshotIdsRef {
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
    #[doc = "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `owners` after provisioning.\n"]
    pub fn owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.owners", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restorable_by_user_ids` after provisioning.\n"]
    pub fn restorable_by_user_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restorable_by_user_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEbsSnapshotIdsTimeoutsElRef {
        DataEbsSnapshotIdsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataEbsSnapshotIdsFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}
impl DataEbsSnapshotIdsFilterEl {}
impl ToListMappable for DataEbsSnapshotIdsFilterEl {
    type O = BlockAssignable<DataEbsSnapshotIdsFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEbsSnapshotIdsFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}
impl BuildDataEbsSnapshotIdsFilterEl {
    pub fn build(self) -> DataEbsSnapshotIdsFilterEl {
        DataEbsSnapshotIdsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}
pub struct DataEbsSnapshotIdsFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEbsSnapshotIdsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEbsSnapshotIdsFilterElRef {
        DataEbsSnapshotIdsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEbsSnapshotIdsFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataEbsSnapshotIdsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}
impl DataEbsSnapshotIdsTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}
impl ToListMappable for DataEbsSnapshotIdsTimeoutsEl {
    type O = BlockAssignable<DataEbsSnapshotIdsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEbsSnapshotIdsTimeoutsEl {}
impl BuildDataEbsSnapshotIdsTimeoutsEl {
    pub fn build(self) -> DataEbsSnapshotIdsTimeoutsEl {
        DataEbsSnapshotIdsTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}
pub struct DataEbsSnapshotIdsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEbsSnapshotIdsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEbsSnapshotIdsTimeoutsElRef {
        DataEbsSnapshotIdsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEbsSnapshotIdsTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataEbsSnapshotIdsDynamic {
    filter: Option<DynamicBlock<DataEbsSnapshotIdsFilterEl>>,
}
