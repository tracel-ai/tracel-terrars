use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataServicecatalogLaunchPathsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accept_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    product_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataServicecatalogLaunchPathsTimeoutsEl>,
}
struct DataServicecatalogLaunchPaths_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServicecatalogLaunchPathsData>,
}
#[derive(Clone)]
pub struct DataServicecatalogLaunchPaths(Rc<DataServicecatalogLaunchPaths_>);
impl DataServicecatalogLaunchPaths {
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
    #[doc = "Set the field `accept_language`.\n"]
    pub fn set_accept_language(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().accept_language = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataServicecatalogLaunchPathsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accept_language", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `summaries` after provisioning.\n"]
    pub fn summaries(&self) -> ListRef<DataServicecatalogLaunchPathsSummariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.summaries", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogLaunchPathsTimeoutsElRef {
        DataServicecatalogLaunchPathsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataServicecatalogLaunchPaths {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataServicecatalogLaunchPaths {}
impl ToListMappable for DataServicecatalogLaunchPaths {
    type O = ListRef<DataServicecatalogLaunchPathsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataServicecatalogLaunchPaths_ {
    fn extract_datasource_type(&self) -> String {
        "aws_servicecatalog_launch_paths".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataServicecatalogLaunchPaths {
    pub tf_id: String,
    #[doc = ""]
    pub product_id: PrimField<String>,
}
impl BuildDataServicecatalogLaunchPaths {
    pub fn build(self, stack: &mut Stack) -> DataServicecatalogLaunchPaths {
        let out = DataServicecatalogLaunchPaths(Rc::new(DataServicecatalogLaunchPaths_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataServicecatalogLaunchPathsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                accept_language: core::default::Default::default(),
                id: core::default::Default::default(),
                product_id: self.product_id,
                region: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataServicecatalogLaunchPathsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServicecatalogLaunchPathsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataServicecatalogLaunchPathsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accept_language", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `summaries` after provisioning.\n"]
    pub fn summaries(&self) -> ListRef<DataServicecatalogLaunchPathsSummariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.summaries", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogLaunchPathsTimeoutsElRef {
        DataServicecatalogLaunchPathsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
    type O = BlockAssignable<DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {}
impl BuildDataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
    pub fn build(self) -> DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
        DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
            description: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef {
        DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataServicecatalogLaunchPathsSummariesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    constraint_summaries:
        Option<ListField<DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
impl DataServicecatalogLaunchPathsSummariesEl {
    #[doc = "Set the field `constraint_summaries`.\n"]
    pub fn set_constraint_summaries(
        mut self,
        v: impl Into<ListField<DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl>>,
    ) -> Self {
        self.constraint_summaries = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `path_id`.\n"]
    pub fn set_path_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_id = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataServicecatalogLaunchPathsSummariesEl {
    type O = BlockAssignable<DataServicecatalogLaunchPathsSummariesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataServicecatalogLaunchPathsSummariesEl {}
impl BuildDataServicecatalogLaunchPathsSummariesEl {
    pub fn build(self) -> DataServicecatalogLaunchPathsSummariesEl {
        DataServicecatalogLaunchPathsSummariesEl {
            constraint_summaries: core::default::Default::default(),
            name: core::default::Default::default(),
            path_id: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct DataServicecatalogLaunchPathsSummariesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServicecatalogLaunchPathsSummariesElRef {
    fn new(shared: StackShared, base: String) -> DataServicecatalogLaunchPathsSummariesElRef {
        DataServicecatalogLaunchPathsSummariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataServicecatalogLaunchPathsSummariesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `constraint_summaries` after provisioning.\n"]
    pub fn constraint_summaries(
        &self,
    ) -> ListRef<DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.constraint_summaries", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `path_id` after provisioning.\n"]
    pub fn path_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_id", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize)]
pub struct DataServicecatalogLaunchPathsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}
impl DataServicecatalogLaunchPathsTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}
impl ToListMappable for DataServicecatalogLaunchPathsTimeoutsEl {
    type O = BlockAssignable<DataServicecatalogLaunchPathsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataServicecatalogLaunchPathsTimeoutsEl {}
impl BuildDataServicecatalogLaunchPathsTimeoutsEl {
    pub fn build(self) -> DataServicecatalogLaunchPathsTimeoutsEl {
        DataServicecatalogLaunchPathsTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}
pub struct DataServicecatalogLaunchPathsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServicecatalogLaunchPathsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataServicecatalogLaunchPathsTimeoutsElRef {
        DataServicecatalogLaunchPathsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataServicecatalogLaunchPathsTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
