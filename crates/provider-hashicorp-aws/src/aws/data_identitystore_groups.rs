use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataIdentitystoreGroupsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    identity_store_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataIdentitystoreGroups_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIdentitystoreGroupsData>,
}
#[derive(Clone)]
pub struct DataIdentitystoreGroups(Rc<DataIdentitystoreGroups_>);
impl DataIdentitystoreGroups {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> ListRef<DataIdentitystoreGroupsGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for DataIdentitystoreGroups {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataIdentitystoreGroups {}
impl ToListMappable for DataIdentitystoreGroups {
    type O = ListRef<DataIdentitystoreGroupsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataIdentitystoreGroups_ {
    fn extract_datasource_type(&self) -> String {
        "aws_identitystore_groups".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataIdentitystoreGroups {
    pub tf_id: String,
    #[doc = ""]
    pub identity_store_id: PrimField<String>,
}
impl BuildDataIdentitystoreGroups {
    pub fn build(self, stack: &mut Stack) -> DataIdentitystoreGroups {
        let out = DataIdentitystoreGroups(Rc::new(DataIdentitystoreGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIdentitystoreGroupsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                identity_store_id: self.identity_store_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataIdentitystoreGroupsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreGroupsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataIdentitystoreGroupsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> ListRef<DataIdentitystoreGroupsGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataIdentitystoreGroupsGroupsElExternalIdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
}
impl DataIdentitystoreGroupsGroupsElExternalIdsEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }
}
impl ToListMappable for DataIdentitystoreGroupsGroupsElExternalIdsEl {
    type O = BlockAssignable<DataIdentitystoreGroupsGroupsElExternalIdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIdentitystoreGroupsGroupsElExternalIdsEl {}
impl BuildDataIdentitystoreGroupsGroupsElExternalIdsEl {
    pub fn build(self) -> DataIdentitystoreGroupsGroupsElExternalIdsEl {
        DataIdentitystoreGroupsGroupsElExternalIdsEl {
            id: core::default::Default::default(),
            issuer: core::default::Default::default(),
        }
    }
}
pub struct DataIdentitystoreGroupsGroupsElExternalIdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreGroupsGroupsElExternalIdsElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreGroupsGroupsElExternalIdsElRef {
        DataIdentitystoreGroupsGroupsElExternalIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIdentitystoreGroupsGroupsElExternalIdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}
#[derive(Serialize)]
pub struct DataIdentitystoreGroupsGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ids: Option<ListField<DataIdentitystoreGroupsGroupsElExternalIdsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_store_id: Option<PrimField<String>>,
}
impl DataIdentitystoreGroupsGroupsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `external_ids`.\n"]
    pub fn set_external_ids(
        mut self,
        v: impl Into<ListField<DataIdentitystoreGroupsGroupsElExternalIdsEl>>,
    ) -> Self {
        self.external_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_store_id`.\n"]
    pub fn set_identity_store_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_store_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataIdentitystoreGroupsGroupsEl {
    type O = BlockAssignable<DataIdentitystoreGroupsGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIdentitystoreGroupsGroupsEl {}
impl BuildDataIdentitystoreGroupsGroupsEl {
    pub fn build(self) -> DataIdentitystoreGroupsGroupsEl {
        DataIdentitystoreGroupsGroupsEl {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            external_ids: core::default::Default::default(),
            group_id: core::default::Default::default(),
            identity_store_id: core::default::Default::default(),
        }
    }
}
pub struct DataIdentitystoreGroupsGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIdentitystoreGroupsGroupsElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreGroupsGroupsElRef {
        DataIdentitystoreGroupsGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIdentitystoreGroupsGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> ListRef<DataIdentitystoreGroupsGroupsElExternalIdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }
    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_store_id", self.base),
        )
    }
}
