use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SagemakerProjectData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_description: Option<PrimField<String>>,
    project_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_catalog_provisioning_details:
        Option<Vec<SagemakerProjectServiceCatalogProvisioningDetailsEl>>,
    dynamic: SagemakerProjectDynamic,
}
struct SagemakerProject_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerProjectData>,
}
#[derive(Clone)]
pub struct SagemakerProject(Rc<SagemakerProject_>);
impl SagemakerProject {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project_description`.\n"]
    pub fn set_project_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project_description = Some(v.into());
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
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `service_catalog_provisioning_details`.\n"]
    pub fn set_service_catalog_provisioning_details(
        self,
        v: impl Into<BlockAssignable<SagemakerProjectServiceCatalogProvisioningDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .service_catalog_provisioning_details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .service_catalog_provisioning_details = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project_description` after provisioning.\n"]
    pub fn project_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_catalog_provisioning_details` after provisioning.\n"]
    pub fn service_catalog_provisioning_details(
        &self,
    ) -> ListRef<SagemakerProjectServiceCatalogProvisioningDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.service_catalog_provisioning_details",
                self.extract_ref()
            ),
        )
    }
}
impl Referable for SagemakerProject {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SagemakerProject {}
impl ToListMappable for SagemakerProject {
    type O = ListRef<SagemakerProjectRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SagemakerProject_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_project".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSagemakerProject {
    pub tf_id: String,
    #[doc = ""]
    pub project_name: PrimField<String>,
}
impl BuildSagemakerProject {
    pub fn build(self, stack: &mut Stack) -> SagemakerProject {
        let out = SagemakerProject(Rc::new(SagemakerProject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerProjectData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                project_description: core::default::Default::default(),
                project_name: self.project_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                service_catalog_provisioning_details: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SagemakerProjectRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerProjectRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SagemakerProjectRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project_description` after provisioning.\n"]
    pub fn project_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_catalog_provisioning_details` after provisioning.\n"]
    pub fn service_catalog_provisioning_details(
        &self,
    ) -> ListRef<SagemakerProjectServiceCatalogProvisioningDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.service_catalog_provisioning_details",
                self.extract_ref()
            ),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
    type O =
        BlockAssignable<SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
    #[doc = ""]
    pub key: PrimField<String>,
}
impl BuildSagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
    pub fn build(
        self,
    ) -> SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
        SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
            key: self.key,
            value: core::default::Default::default(),
        }
    }
}
pub struct SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef {
        SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerProjectServiceCatalogProvisioningDetailsElDynamic {
    provisioning_parameter: Option<
        DynamicBlock<SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl>,
    >,
}
#[derive(Serialize)]
pub struct SagemakerProjectServiceCatalogProvisioningDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path_id: Option<PrimField<String>>,
    product_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioning_artifact_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioning_parameter:
        Option<Vec<SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl>>,
    dynamic: SagemakerProjectServiceCatalogProvisioningDetailsElDynamic,
}
impl SagemakerProjectServiceCatalogProvisioningDetailsEl {
    #[doc = "Set the field `path_id`.\n"]
    pub fn set_path_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_id = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioning_artifact_id`.\n"]
    pub fn set_provisioning_artifact_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provisioning_artifact_id = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioning_parameter`.\n"]
    pub fn set_provisioning_parameter(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.provisioning_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.provisioning_parameter = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerProjectServiceCatalogProvisioningDetailsEl {
    type O = BlockAssignable<SagemakerProjectServiceCatalogProvisioningDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerProjectServiceCatalogProvisioningDetailsEl {
    #[doc = ""]
    pub product_id: PrimField<String>,
}
impl BuildSagemakerProjectServiceCatalogProvisioningDetailsEl {
    pub fn build(self) -> SagemakerProjectServiceCatalogProvisioningDetailsEl {
        SagemakerProjectServiceCatalogProvisioningDetailsEl {
            path_id: core::default::Default::default(),
            product_id: self.product_id,
            provisioning_artifact_id: core::default::Default::default(),
            provisioning_parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerProjectServiceCatalogProvisioningDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerProjectServiceCatalogProvisioningDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerProjectServiceCatalogProvisioningDetailsElRef {
        SagemakerProjectServiceCatalogProvisioningDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerProjectServiceCatalogProvisioningDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path_id` after provisioning.\n"]
    pub fn path_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_id", self.base))
    }
    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.base))
    }
    #[doc = "Get a reference to the value of field `provisioning_artifact_id` after provisioning.\n"]
    pub fn provisioning_artifact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioning_artifact_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provisioning_parameter` after provisioning.\n"]
    pub fn provisioning_parameter(
        &self,
    ) -> ListRef<SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioning_parameter", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerProjectDynamic {
    service_catalog_provisioning_details:
        Option<DynamicBlock<SagemakerProjectServiceCatalogProvisioningDetailsEl>>,
}
