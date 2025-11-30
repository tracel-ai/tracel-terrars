use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DevopsguruResourceCollectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudformation: Option<Vec<DevopsguruResourceCollectionCloudformationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DevopsguruResourceCollectionTagsEl>>,
    dynamic: DevopsguruResourceCollectionDynamic,
}

struct DevopsguruResourceCollection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DevopsguruResourceCollectionData>,
}

#[derive(Clone)]
pub struct DevopsguruResourceCollection(Rc<DevopsguruResourceCollection_>);

impl DevopsguruResourceCollection {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `cloudformation`.\n"]
    pub fn set_cloudformation(
        self,
        v: impl Into<BlockAssignable<DevopsguruResourceCollectionCloudformationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudformation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudformation = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl Into<BlockAssignable<DevopsguruResourceCollectionTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tags = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloudformation` after provisioning.\n"]
    pub fn cloudformation(&self) -> ListRef<DevopsguruResourceCollectionCloudformationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudformation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DevopsguruResourceCollectionTagsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

impl Referable for DevopsguruResourceCollection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for DevopsguruResourceCollection {}

impl ToListMappable for DevopsguruResourceCollection {
    type O = ListRef<DevopsguruResourceCollectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DevopsguruResourceCollection_ {
    fn extract_resource_type(&self) -> String {
        "aws_devopsguru_resource_collection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevopsguruResourceCollection {
    pub tf_id: String,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildDevopsguruResourceCollection {
    pub fn build(self, stack: &mut Stack) -> DevopsguruResourceCollection {
        let out = DevopsguruResourceCollection(Rc::new(DevopsguruResourceCollection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DevopsguruResourceCollectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                type_: self.type_,
                cloudformation: core::default::Default::default(),
                tags: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevopsguruResourceCollectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruResourceCollectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DevopsguruResourceCollectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloudformation` after provisioning.\n"]
    pub fn cloudformation(&self) -> ListRef<DevopsguruResourceCollectionCloudformationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudformation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DevopsguruResourceCollectionTagsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DevopsguruResourceCollectionCloudformationEl {
    stack_names: ListField<PrimField<String>>,
}

impl DevopsguruResourceCollectionCloudformationEl {}

impl ToListMappable for DevopsguruResourceCollectionCloudformationEl {
    type O = BlockAssignable<DevopsguruResourceCollectionCloudformationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruResourceCollectionCloudformationEl {
    #[doc = ""]
    pub stack_names: ListField<PrimField<String>>,
}

impl BuildDevopsguruResourceCollectionCloudformationEl {
    pub fn build(self) -> DevopsguruResourceCollectionCloudformationEl {
        DevopsguruResourceCollectionCloudformationEl {
            stack_names: self.stack_names,
        }
    }
}

pub struct DevopsguruResourceCollectionCloudformationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruResourceCollectionCloudformationElRef {
    fn new(shared: StackShared, base: String) -> DevopsguruResourceCollectionCloudformationElRef {
        DevopsguruResourceCollectionCloudformationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruResourceCollectionCloudformationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stack_names` after provisioning.\n"]
    pub fn stack_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.stack_names", self.base))
    }
}

#[derive(Serialize)]
pub struct DevopsguruResourceCollectionTagsEl {
    app_boundary_key: PrimField<String>,
    tag_values: ListField<PrimField<String>>,
}

impl DevopsguruResourceCollectionTagsEl {}

impl ToListMappable for DevopsguruResourceCollectionTagsEl {
    type O = BlockAssignable<DevopsguruResourceCollectionTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruResourceCollectionTagsEl {
    #[doc = ""]
    pub app_boundary_key: PrimField<String>,
    #[doc = ""]
    pub tag_values: ListField<PrimField<String>>,
}

impl BuildDevopsguruResourceCollectionTagsEl {
    pub fn build(self) -> DevopsguruResourceCollectionTagsEl {
        DevopsguruResourceCollectionTagsEl {
            app_boundary_key: self.app_boundary_key,
            tag_values: self.tag_values,
        }
    }
}

pub struct DevopsguruResourceCollectionTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruResourceCollectionTagsElRef {
    fn new(shared: StackShared, base: String) -> DevopsguruResourceCollectionTagsElRef {
        DevopsguruResourceCollectionTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruResourceCollectionTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_boundary_key` after provisioning.\n"]
    pub fn app_boundary_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_boundary_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]
    pub fn tag_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DevopsguruResourceCollectionDynamic {
    cloudformation: Option<DynamicBlock<DevopsguruResourceCollectionCloudformationEl>>,
    tags: Option<DynamicBlock<DevopsguruResourceCollectionTagsEl>>,
}
