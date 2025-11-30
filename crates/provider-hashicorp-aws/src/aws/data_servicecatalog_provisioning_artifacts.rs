use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataServicecatalogProvisioningArtifactsData {
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
    timeouts: Option<DataServicecatalogProvisioningArtifactsTimeoutsEl>,
}

struct DataServicecatalogProvisioningArtifacts_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServicecatalogProvisioningArtifactsData>,
}

#[derive(Clone)]
pub struct DataServicecatalogProvisioningArtifacts(Rc<DataServicecatalogProvisioningArtifacts_>);

impl DataServicecatalogProvisioningArtifacts {
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
    pub fn set_timeouts(
        self,
        v: impl Into<DataServicecatalogProvisioningArtifactsTimeoutsEl>,
    ) -> Self {
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

    #[doc = "Get a reference to the value of field `provisioning_artifact_details` after provisioning.\n"]
    pub fn provisioning_artifact_details(
        &self,
    ) -> ListRef<DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioning_artifact_details", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogProvisioningArtifactsTimeoutsElRef {
        DataServicecatalogProvisioningArtifactsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DataServicecatalogProvisioningArtifacts {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataServicecatalogProvisioningArtifacts {}

impl ToListMappable for DataServicecatalogProvisioningArtifacts {
    type O = ListRef<DataServicecatalogProvisioningArtifactsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataServicecatalogProvisioningArtifacts_ {
    fn extract_datasource_type(&self) -> String {
        "aws_servicecatalog_provisioning_artifacts".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServicecatalogProvisioningArtifacts {
    pub tf_id: String,
    #[doc = ""]
    pub product_id: PrimField<String>,
}

impl BuildDataServicecatalogProvisioningArtifacts {
    pub fn build(self, stack: &mut Stack) -> DataServicecatalogProvisioningArtifacts {
        let out = DataServicecatalogProvisioningArtifacts(Rc::new(
            DataServicecatalogProvisioningArtifacts_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataServicecatalogProvisioningArtifactsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    accept_language: core::default::Default::default(),
                    id: core::default::Default::default(),
                    product_id: self.product_id,
                    region: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServicecatalogProvisioningArtifactsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServicecatalogProvisioningArtifactsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataServicecatalogProvisioningArtifactsRef {
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

    #[doc = "Get a reference to the value of field `provisioning_artifact_details` after provisioning.\n"]
    pub fn provisioning_artifact_details(
        &self,
    ) -> ListRef<DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioning_artifact_details", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogProvisioningArtifactsTimeoutsElRef {
        DataServicecatalogProvisioningArtifactsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsEl {
    #[doc = "Set the field `active`.\n"]
    pub fn set_active(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.active = Some(v.into());
        self
    }

    #[doc = "Set the field `created_time`.\n"]
    pub fn set_created_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_time = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `guidance`.\n"]
    pub fn set_guidance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.guidance = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsEl {
    type O = BlockAssignable<DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsEl {}

impl BuildDataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsEl {
    pub fn build(self) -> DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsEl {
        DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsEl {
            active: core::default::Default::default(),
            created_time: core::default::Default::default(),
            description: core::default::Default::default(),
            guidance: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsElRef {
        DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicecatalogProvisioningArtifactsProvisioningArtifactDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `guidance` after provisioning.\n"]
    pub fn guidance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.guidance", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataServicecatalogProvisioningArtifactsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataServicecatalogProvisioningArtifactsTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataServicecatalogProvisioningArtifactsTimeoutsEl {
    type O = BlockAssignable<DataServicecatalogProvisioningArtifactsTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicecatalogProvisioningArtifactsTimeoutsEl {}

impl BuildDataServicecatalogProvisioningArtifactsTimeoutsEl {
    pub fn build(self) -> DataServicecatalogProvisioningArtifactsTimeoutsEl {
        DataServicecatalogProvisioningArtifactsTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}

pub struct DataServicecatalogProvisioningArtifactsTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServicecatalogProvisioningArtifactsTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataServicecatalogProvisioningArtifactsTimeoutsElRef {
        DataServicecatalogProvisioningArtifactsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicecatalogProvisioningArtifactsTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
