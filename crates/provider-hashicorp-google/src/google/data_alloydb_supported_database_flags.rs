use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataAlloydbSupportedDatabaseFlagsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataAlloydbSupportedDatabaseFlags_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAlloydbSupportedDatabaseFlagsData>,
}
#[derive(Clone)]
pub struct DataAlloydbSupportedDatabaseFlags(Rc<DataAlloydbSupportedDatabaseFlags_>);
impl DataAlloydbSupportedDatabaseFlags {
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
    #[doc = "Set the field `project`.\nProject ID of the project."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe canonical id for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_database_flags` after provisioning.\n"]
    pub fn supported_database_flags(
        &self,
    ) -> ListRef<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_database_flags", self.extract_ref()),
        )
    }
}
impl Referable for DataAlloydbSupportedDatabaseFlags {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataAlloydbSupportedDatabaseFlags {}
impl ToListMappable for DataAlloydbSupportedDatabaseFlags {
    type O = ListRef<DataAlloydbSupportedDatabaseFlagsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataAlloydbSupportedDatabaseFlags_ {
    fn extract_datasource_type(&self) -> String {
        "google_alloydb_supported_database_flags".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataAlloydbSupportedDatabaseFlags {
    pub tf_id: String,
    #[doc = "The canonical id for the location. For example: \"us-east1\"."]
    pub location: PrimField<String>,
}
impl BuildDataAlloydbSupportedDatabaseFlags {
    pub fn build(self, stack: &mut Stack) -> DataAlloydbSupportedDatabaseFlags {
        let out = DataAlloydbSupportedDatabaseFlags(Rc::new(DataAlloydbSupportedDatabaseFlags_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAlloydbSupportedDatabaseFlagsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataAlloydbSupportedDatabaseFlagsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbSupportedDatabaseFlagsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataAlloydbSupportedDatabaseFlagsRef {
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe canonical id for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_database_flags` after provisioning.\n"]
    pub fn supported_database_flags(
        &self,
    ) -> ListRef<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_database_flags", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<PrimField<String>>,
}
impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {
    #[doc = "Set the field `max_value`.\n"]
    pub fn set_max_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_value = Some(v.into());
        self
    }
    #[doc = "Set the field `min_value`.\n"]
    pub fn set_min_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl
{
    type O = BlockAssignable<
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {}
impl BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {
    pub fn build(
        self,
    ) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_value` after provisioning.\n"]
    pub fn max_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }
    #[doc = "Get a reference to the value of field `min_value` after provisioning.\n"]
    pub fn min_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_values: Option<ListField<PrimField<String>>>,
}
impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_values = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl
{
    type O = BlockAssignable<
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {}
impl BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {
    pub fn build(
        self,
    ) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {
            allowed_values: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\n"]
    pub fn allowed_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_values", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accepts_multiple_values: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flag_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integer_restrictions: Option<
        ListField<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requires_db_restart: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_restrictions: Option<
        ListField<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_db_versions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_type: Option<PrimField<String>>,
}
impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
    #[doc = "Set the field `accepts_multiple_values`.\n"]
    pub fn set_accepts_multiple_values(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.accepts_multiple_values = Some(v.into());
        self
    }
    #[doc = "Set the field `flag_name`.\n"]
    pub fn set_flag_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flag_name = Some(v.into());
        self
    }
    #[doc = "Set the field `integer_restrictions`.\n"]
    pub fn set_integer_restrictions(
        mut self,
        v: impl Into<
            ListField<
                DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl,
            >,
        >,
    ) -> Self {
        self.integer_restrictions = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `requires_db_restart`.\n"]
    pub fn set_requires_db_restart(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.requires_db_restart = Some(v.into());
        self
    }
    #[doc = "Set the field `string_restrictions`.\n"]
    pub fn set_string_restrictions(
        mut self,
        v: impl Into<
            ListField<
                DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl,
            >,
        >,
    ) -> Self {
        self.string_restrictions = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_db_versions`.\n"]
    pub fn set_supported_db_versions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.supported_db_versions = Some(v.into());
        self
    }
    #[doc = "Set the field `value_type`.\n"]
    pub fn set_value_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
    type O = BlockAssignable<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {}
impl BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
    pub fn build(self) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
            accepts_multiple_values: core::default::Default::default(),
            flag_name: core::default::Default::default(),
            integer_restrictions: core::default::Default::default(),
            name: core::default::Default::default(),
            requires_db_restart: core::default::Default::default(),
            string_restrictions: core::default::Default::default(),
            supported_db_versions: core::default::Default::default(),
            value_type: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accepts_multiple_values` after provisioning.\n"]
    pub fn accepts_multiple_values(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accepts_multiple_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `flag_name` after provisioning.\n"]
    pub fn flag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flag_name", self.base))
    }
    #[doc = "Get a reference to the value of field `integer_restrictions` after provisioning.\n"]
    pub fn integer_restrictions(
        &self,
    ) -> ListRef<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.integer_restrictions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `requires_db_restart` after provisioning.\n"]
    pub fn requires_db_restart(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requires_db_restart", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `string_restrictions` after provisioning.\n"]
    pub fn string_restrictions(
        &self,
    ) -> ListRef<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.string_restrictions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `supported_db_versions` after provisioning.\n"]
    pub fn supported_db_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_db_versions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `value_type` after provisioning.\n"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
}
