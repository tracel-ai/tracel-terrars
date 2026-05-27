use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataApphubDiscoveredWorkloadData {
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
    workload_uri: PrimField<String>,
}
struct DataApphubDiscoveredWorkload_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataApphubDiscoveredWorkloadData>,
}
#[derive(Clone)]
pub struct DataApphubDiscoveredWorkload(Rc<DataApphubDiscoveredWorkload_>);
impl DataApphubDiscoveredWorkload {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_properties` after provisioning.\n"]
    pub fn workload_properties(
        &self,
    ) -> ListRef<DataApphubDiscoveredWorkloadWorkloadPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_reference` after provisioning.\n"]
    pub fn workload_reference(
        &self,
    ) -> ListRef<DataApphubDiscoveredWorkloadWorkloadReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_uri` after provisioning.\n"]
    pub fn workload_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_uri", self.extract_ref()),
        )
    }
}
impl Referable for DataApphubDiscoveredWorkload {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataApphubDiscoveredWorkload {}
impl ToListMappable for DataApphubDiscoveredWorkload {
    type O = ListRef<DataApphubDiscoveredWorkloadRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataApphubDiscoveredWorkload_ {
    fn extract_datasource_type(&self) -> String {
        "google_apphub_discovered_workload".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataApphubDiscoveredWorkload {
    pub tf_id: String,
    #[doc = ""]
    pub location: PrimField<String>,
    #[doc = ""]
    pub workload_uri: PrimField<String>,
}
impl BuildDataApphubDiscoveredWorkload {
    pub fn build(self, stack: &mut Stack) -> DataApphubDiscoveredWorkload {
        let out = DataApphubDiscoveredWorkload(Rc::new(DataApphubDiscoveredWorkload_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataApphubDiscoveredWorkloadData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                workload_uri: self.workload_uri,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataApphubDiscoveredWorkloadRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubDiscoveredWorkloadRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataApphubDiscoveredWorkloadRef {
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_properties` after provisioning.\n"]
    pub fn workload_properties(
        &self,
    ) -> ListRef<DataApphubDiscoveredWorkloadWorkloadPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_reference` after provisioning.\n"]
    pub fn workload_reference(
        &self,
    ) -> ListRef<DataApphubDiscoveredWorkloadWorkloadReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_uri` after provisioning.\n"]
    pub fn workload_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_uri", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataApphubDiscoveredWorkloadWorkloadPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl DataApphubDiscoveredWorkloadWorkloadPropertiesEl {
    #[doc = "Set the field `gcp_project`.\n"]
    pub fn set_gcp_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_project = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataApphubDiscoveredWorkloadWorkloadPropertiesEl {
    type O = BlockAssignable<DataApphubDiscoveredWorkloadWorkloadPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataApphubDiscoveredWorkloadWorkloadPropertiesEl {}
impl BuildDataApphubDiscoveredWorkloadWorkloadPropertiesEl {
    pub fn build(self) -> DataApphubDiscoveredWorkloadWorkloadPropertiesEl {
        DataApphubDiscoveredWorkloadWorkloadPropertiesEl {
            gcp_project: core::default::Default::default(),
            location: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct DataApphubDiscoveredWorkloadWorkloadPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubDiscoveredWorkloadWorkloadPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataApphubDiscoveredWorkloadWorkloadPropertiesElRef {
        DataApphubDiscoveredWorkloadWorkloadPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataApphubDiscoveredWorkloadWorkloadPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_project` after provisioning.\n"]
    pub fn gcp_project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcp_project", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct DataApphubDiscoveredWorkloadWorkloadReferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DataApphubDiscoveredWorkloadWorkloadReferenceEl {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataApphubDiscoveredWorkloadWorkloadReferenceEl {
    type O = BlockAssignable<DataApphubDiscoveredWorkloadWorkloadReferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataApphubDiscoveredWorkloadWorkloadReferenceEl {}
impl BuildDataApphubDiscoveredWorkloadWorkloadReferenceEl {
    pub fn build(self) -> DataApphubDiscoveredWorkloadWorkloadReferenceEl {
        DataApphubDiscoveredWorkloadWorkloadReferenceEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct DataApphubDiscoveredWorkloadWorkloadReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataApphubDiscoveredWorkloadWorkloadReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataApphubDiscoveredWorkloadWorkloadReferenceElRef {
        DataApphubDiscoveredWorkloadWorkloadReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataApphubDiscoveredWorkloadWorkloadReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
