use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDevopsguruResourceCollectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

struct DataDevopsguruResourceCollection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDevopsguruResourceCollectionData>,
}

#[derive(Clone)]
pub struct DataDevopsguruResourceCollection(Rc<DataDevopsguruResourceCollection_>);

impl DataDevopsguruResourceCollection {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cloudformation` after provisioning.\n"]
    pub fn cloudformation(&self) -> ListRef<DataDevopsguruResourceCollectionCloudformationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudformation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataDevopsguruResourceCollectionTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl Referable for DataDevopsguruResourceCollection {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataDevopsguruResourceCollection { }

impl ToListMappable for DataDevopsguruResourceCollection {
    type O = ListRef<DataDevopsguruResourceCollectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDevopsguruResourceCollection_ {
    fn extract_datasource_type(&self) -> String {
        "aws_devopsguru_resource_collection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDevopsguruResourceCollection {
    pub tf_id: String,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildDataDevopsguruResourceCollection {
    pub fn build(self, stack: &mut Stack) -> DataDevopsguruResourceCollection {
        let out = DataDevopsguruResourceCollection(Rc::new(DataDevopsguruResourceCollection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDevopsguruResourceCollectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                type_: self.type_,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDevopsguruResourceCollectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDevopsguruResourceCollectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataDevopsguruResourceCollectionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cloudformation` after provisioning.\n"]
    pub fn cloudformation(&self) -> ListRef<DataDevopsguruResourceCollectionCloudformationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudformation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataDevopsguruResourceCollectionTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDevopsguruResourceCollectionCloudformationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_names: Option<ListField<PrimField<String>>>,
}

impl DataDevopsguruResourceCollectionCloudformationEl {
    #[doc = "Set the field `stack_names`.\n"]
    pub fn set_stack_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.stack_names = Some(v.into());
        self
    }
}

impl ToListMappable for DataDevopsguruResourceCollectionCloudformationEl {
    type O = BlockAssignable<DataDevopsguruResourceCollectionCloudformationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDevopsguruResourceCollectionCloudformationEl {}

impl BuildDataDevopsguruResourceCollectionCloudformationEl {
    pub fn build(self) -> DataDevopsguruResourceCollectionCloudformationEl {
        DataDevopsguruResourceCollectionCloudformationEl { stack_names: core::default::Default::default() }
    }
}

pub struct DataDevopsguruResourceCollectionCloudformationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDevopsguruResourceCollectionCloudformationElRef {
    fn new(shared: StackShared, base: String) -> DataDevopsguruResourceCollectionCloudformationElRef {
        DataDevopsguruResourceCollectionCloudformationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDevopsguruResourceCollectionCloudformationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stack_names` after provisioning.\n"]
    pub fn stack_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.stack_names", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDevopsguruResourceCollectionTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_boundary_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_values: Option<ListField<PrimField<String>>>,
}

impl DataDevopsguruResourceCollectionTagsEl {
    #[doc = "Set the field `app_boundary_key`.\n"]
    pub fn set_app_boundary_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.app_boundary_key = Some(v.into());
        self
    }

    #[doc = "Set the field `tag_values`.\n"]
    pub fn set_tag_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tag_values = Some(v.into());
        self
    }
}

impl ToListMappable for DataDevopsguruResourceCollectionTagsEl {
    type O = BlockAssignable<DataDevopsguruResourceCollectionTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDevopsguruResourceCollectionTagsEl {}

impl BuildDataDevopsguruResourceCollectionTagsEl {
    pub fn build(self) -> DataDevopsguruResourceCollectionTagsEl {
        DataDevopsguruResourceCollectionTagsEl {
            app_boundary_key: core::default::Default::default(),
            tag_values: core::default::Default::default(),
        }
    }
}

pub struct DataDevopsguruResourceCollectionTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDevopsguruResourceCollectionTagsElRef {
    fn new(shared: StackShared, base: String) -> DataDevopsguruResourceCollectionTagsElRef {
        DataDevopsguruResourceCollectionTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDevopsguruResourceCollectionTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_boundary_key` after provisioning.\n"]
    pub fn app_boundary_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_boundary_key", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]
    pub fn tag_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}
