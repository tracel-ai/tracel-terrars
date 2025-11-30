use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataAppmeshMeshData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mesh_owner: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataAppmeshMesh_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppmeshMeshData>,
}

#[derive(Clone)]
pub struct DataAppmeshMesh(Rc<DataAppmeshMesh_>);

impl DataAppmeshMesh {
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

    #[doc = "Set the field `mesh_owner`.\n"]
    pub fn set_mesh_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mesh_owner = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mesh_owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<DataAppmeshMeshSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

impl Referable for DataAppmeshMesh {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataAppmeshMesh {}

impl ToListMappable for DataAppmeshMesh {
    type O = ListRef<DataAppmeshMeshRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAppmeshMesh_ {
    fn extract_datasource_type(&self) -> String {
        "aws_appmesh_mesh".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppmeshMesh {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataAppmeshMesh {
    pub fn build(self, stack: &mut Stack) -> DataAppmeshMesh {
        let out = DataAppmeshMesh(Rc::new(DataAppmeshMesh_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppmeshMeshData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                mesh_owner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppmeshMeshRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshMeshRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataAppmeshMeshRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mesh_owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<DataAppmeshMeshSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataAppmeshMeshSpecElEgressFilterEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataAppmeshMeshSpecElEgressFilterEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshMeshSpecElEgressFilterEl {
    type O = BlockAssignable<DataAppmeshMeshSpecElEgressFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshMeshSpecElEgressFilterEl {}

impl BuildDataAppmeshMeshSpecElEgressFilterEl {
    pub fn build(self) -> DataAppmeshMeshSpecElEgressFilterEl {
        DataAppmeshMeshSpecElEgressFilterEl {
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshMeshSpecElEgressFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshMeshSpecElEgressFilterElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshMeshSpecElEgressFilterElRef {
        DataAppmeshMeshSpecElEgressFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshMeshSpecElEgressFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshMeshSpecElServiceDiscoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_preference: Option<PrimField<String>>,
}

impl DataAppmeshMeshSpecElServiceDiscoveryEl {
    #[doc = "Set the field `ip_preference`.\n"]
    pub fn set_ip_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_preference = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshMeshSpecElServiceDiscoveryEl {
    type O = BlockAssignable<DataAppmeshMeshSpecElServiceDiscoveryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshMeshSpecElServiceDiscoveryEl {}

impl BuildDataAppmeshMeshSpecElServiceDiscoveryEl {
    pub fn build(self) -> DataAppmeshMeshSpecElServiceDiscoveryEl {
        DataAppmeshMeshSpecElServiceDiscoveryEl {
            ip_preference: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshMeshSpecElServiceDiscoveryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshMeshSpecElServiceDiscoveryElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshMeshSpecElServiceDiscoveryElRef {
        DataAppmeshMeshSpecElServiceDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshMeshSpecElServiceDiscoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_preference` after provisioning.\n"]
    pub fn ip_preference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_preference", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataAppmeshMeshSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_filter: Option<ListField<DataAppmeshMeshSpecElEgressFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_discovery: Option<ListField<DataAppmeshMeshSpecElServiceDiscoveryEl>>,
}

impl DataAppmeshMeshSpecEl {
    #[doc = "Set the field `egress_filter`.\n"]
    pub fn set_egress_filter(
        mut self,
        v: impl Into<ListField<DataAppmeshMeshSpecElEgressFilterEl>>,
    ) -> Self {
        self.egress_filter = Some(v.into());
        self
    }

    #[doc = "Set the field `service_discovery`.\n"]
    pub fn set_service_discovery(
        mut self,
        v: impl Into<ListField<DataAppmeshMeshSpecElServiceDiscoveryEl>>,
    ) -> Self {
        self.service_discovery = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshMeshSpecEl {
    type O = BlockAssignable<DataAppmeshMeshSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshMeshSpecEl {}

impl BuildDataAppmeshMeshSpecEl {
    pub fn build(self) -> DataAppmeshMeshSpecEl {
        DataAppmeshMeshSpecEl {
            egress_filter: core::default::Default::default(),
            service_discovery: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshMeshSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshMeshSpecElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshMeshSpecElRef {
        DataAppmeshMeshSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshMeshSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_filter` after provisioning.\n"]
    pub fn egress_filter(&self) -> ListRef<DataAppmeshMeshSpecElEgressFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.egress_filter", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\n"]
    pub fn service_discovery(&self) -> ListRef<DataAppmeshMeshSpecElServiceDiscoveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_discovery", self.base),
        )
    }
}
