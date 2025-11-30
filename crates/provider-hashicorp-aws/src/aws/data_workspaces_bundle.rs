use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataWorkspacesBundleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bundle_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataWorkspacesBundle_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataWorkspacesBundleData>,
}

#[derive(Clone)]
pub struct DataWorkspacesBundle(Rc<DataWorkspacesBundle_>);

impl DataWorkspacesBundle {
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

    #[doc = "Set the field `bundle_id`.\n"]
    pub fn set_bundle_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bundle_id = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().owner = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bundle_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> ListRef<DataWorkspacesBundleComputeTypeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `root_storage` after provisioning.\n"]
    pub fn root_storage(&self) -> ListRef<DataWorkspacesBundleRootStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_storage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_storage` after provisioning.\n"]
    pub fn user_storage(&self) -> ListRef<DataWorkspacesBundleUserStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_storage", self.extract_ref()),
        )
    }
}

impl Referable for DataWorkspacesBundle {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataWorkspacesBundle {}

impl ToListMappable for DataWorkspacesBundle {
    type O = ListRef<DataWorkspacesBundleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataWorkspacesBundle_ {
    fn extract_datasource_type(&self) -> String {
        "aws_workspaces_bundle".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWorkspacesBundle {
    pub tf_id: String,
}

impl BuildDataWorkspacesBundle {
    pub fn build(self, stack: &mut Stack) -> DataWorkspacesBundle {
        let out = DataWorkspacesBundle(Rc::new(DataWorkspacesBundle_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataWorkspacesBundleData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                bundle_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                owner: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWorkspacesBundleRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesBundleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataWorkspacesBundleRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bundle_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> ListRef<DataWorkspacesBundleComputeTypeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `root_storage` after provisioning.\n"]
    pub fn root_storage(&self) -> ListRef<DataWorkspacesBundleRootStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_storage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_storage` after provisioning.\n"]
    pub fn user_storage(&self) -> ListRef<DataWorkspacesBundleUserStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_storage", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesBundleComputeTypeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataWorkspacesBundleComputeTypeEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataWorkspacesBundleComputeTypeEl {
    type O = BlockAssignable<DataWorkspacesBundleComputeTypeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesBundleComputeTypeEl {}

impl BuildDataWorkspacesBundleComputeTypeEl {
    pub fn build(self) -> DataWorkspacesBundleComputeTypeEl {
        DataWorkspacesBundleComputeTypeEl {
            name: core::default::Default::default(),
        }
    }
}

pub struct DataWorkspacesBundleComputeTypeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesBundleComputeTypeElRef {
    fn new(shared: StackShared, base: String) -> DataWorkspacesBundleComputeTypeElRef {
        DataWorkspacesBundleComputeTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesBundleComputeTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesBundleRootStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<PrimField<String>>,
}

impl DataWorkspacesBundleRootStorageEl {
    #[doc = "Set the field `capacity`.\n"]
    pub fn set_capacity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capacity = Some(v.into());
        self
    }
}

impl ToListMappable for DataWorkspacesBundleRootStorageEl {
    type O = BlockAssignable<DataWorkspacesBundleRootStorageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesBundleRootStorageEl {}

impl BuildDataWorkspacesBundleRootStorageEl {
    pub fn build(self) -> DataWorkspacesBundleRootStorageEl {
        DataWorkspacesBundleRootStorageEl {
            capacity: core::default::Default::default(),
        }
    }
}

pub struct DataWorkspacesBundleRootStorageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesBundleRootStorageElRef {
    fn new(shared: StackShared, base: String) -> DataWorkspacesBundleRootStorageElRef {
        DataWorkspacesBundleRootStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesBundleRootStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesBundleUserStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<PrimField<String>>,
}

impl DataWorkspacesBundleUserStorageEl {
    #[doc = "Set the field `capacity`.\n"]
    pub fn set_capacity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.capacity = Some(v.into());
        self
    }
}

impl ToListMappable for DataWorkspacesBundleUserStorageEl {
    type O = BlockAssignable<DataWorkspacesBundleUserStorageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesBundleUserStorageEl {}

impl BuildDataWorkspacesBundleUserStorageEl {
    pub fn build(self) -> DataWorkspacesBundleUserStorageEl {
        DataWorkspacesBundleUserStorageEl {
            capacity: core::default::Default::default(),
        }
    }
}

pub struct DataWorkspacesBundleUserStorageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesBundleUserStorageElRef {
    fn new(shared: StackShared, base: String) -> DataWorkspacesBundleUserStorageElRef {
        DataWorkspacesBundleUserStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesBundleUserStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.base))
    }
}
