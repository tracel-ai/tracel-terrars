use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ServicecatalogProvisionedProductData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accept_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_errors: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_arns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioning_artifact_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioning_artifact_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_physical_resources: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioning_parameters: Option<Vec<ServicecatalogProvisionedProductProvisioningParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_set_provisioning_preferences:
        Option<Vec<ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ServicecatalogProvisionedProductTimeoutsEl>,
    dynamic: ServicecatalogProvisionedProductDynamic,
}

struct ServicecatalogProvisionedProduct_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ServicecatalogProvisionedProductData>,
}

#[derive(Clone)]
pub struct ServicecatalogProvisionedProduct(Rc<ServicecatalogProvisionedProduct_>);

impl ServicecatalogProvisionedProduct {
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

    #[doc = "Set the field `ignore_errors`.\n"]
    pub fn set_ignore_errors(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_errors = Some(v.into());
        self
    }

    #[doc = "Set the field `notification_arns`.\n"]
    pub fn set_notification_arns(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().notification_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `path_id`.\n"]
    pub fn set_path_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().path_id = Some(v.into());
        self
    }

    #[doc = "Set the field `path_name`.\n"]
    pub fn set_path_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().path_name = Some(v.into());
        self
    }

    #[doc = "Set the field `product_id`.\n"]
    pub fn set_product_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().product_id = Some(v.into());
        self
    }

    #[doc = "Set the field `product_name`.\n"]
    pub fn set_product_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().product_name = Some(v.into());
        self
    }

    #[doc = "Set the field `provisioning_artifact_id`.\n"]
    pub fn set_provisioning_artifact_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().provisioning_artifact_id = Some(v.into());
        self
    }

    #[doc = "Set the field `provisioning_artifact_name`.\n"]
    pub fn set_provisioning_artifact_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().provisioning_artifact_name = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `retain_physical_resources`.\n"]
    pub fn set_retain_physical_resources(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().retain_physical_resources = Some(v.into());
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

    #[doc = "Set the field `provisioning_parameters`.\n"]
    pub fn set_provisioning_parameters(
        self,
        v: impl Into<BlockAssignable<ServicecatalogProvisionedProductProvisioningParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().provisioning_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.provisioning_parameters = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `stack_set_provisioning_preferences`.\n"]
    pub fn set_stack_set_provisioning_preferences(
        self,
        v: impl Into<BlockAssignable<ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stack_set_provisioning_preferences = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .stack_set_provisioning_preferences = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ServicecatalogProvisionedProductTimeoutsEl>) -> Self {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_dashboard_names` after provisioning.\n"]
    pub fn cloudwatch_dashboard_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_dashboard_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_errors` after provisioning.\n"]
    pub fn ignore_errors(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_errors", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_provisioning_record_id` after provisioning.\n"]
    pub fn last_provisioning_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_provisioning_record_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_record_id` after provisioning.\n"]
    pub fn last_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_record_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_successful_provisioning_record_id` after provisioning.\n"]
    pub fn last_successful_provisioning_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.last_successful_provisioning_record_id",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `launch_role_arn` after provisioning.\n"]
    pub fn launch_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `notification_arns` after provisioning.\n"]
    pub fn notification_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> SetRef<ServicecatalogProvisionedProductOutputsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.outputs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `path_id` after provisioning.\n"]
    pub fn path_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `path_name` after provisioning.\n"]
    pub fn path_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `product_name` after provisioning.\n"]
    pub fn product_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `provisioning_artifact_id` after provisioning.\n"]
    pub fn provisioning_artifact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioning_artifact_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `provisioning_artifact_name` after provisioning.\n"]
    pub fn provisioning_artifact_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioning_artifact_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retain_physical_resources` after provisioning.\n"]
    pub fn retain_physical_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_physical_resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_message", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `provisioning_parameters` after provisioning.\n"]
    pub fn provisioning_parameters(
        &self,
    ) -> ListRef<ServicecatalogProvisionedProductProvisioningParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioning_parameters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stack_set_provisioning_preferences` after provisioning.\n"]
    pub fn stack_set_provisioning_preferences(
        &self,
    ) -> ListRef<ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stack_set_provisioning_preferences", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ServicecatalogProvisionedProductTimeoutsElRef {
        ServicecatalogProvisionedProductTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ServicecatalogProvisionedProduct {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ServicecatalogProvisionedProduct {}

impl ToListMappable for ServicecatalogProvisionedProduct {
    type O = ListRef<ServicecatalogProvisionedProductRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ServicecatalogProvisionedProduct_ {
    fn extract_resource_type(&self) -> String {
        "aws_servicecatalog_provisioned_product".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildServicecatalogProvisionedProduct {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildServicecatalogProvisionedProduct {
    pub fn build(self, stack: &mut Stack) -> ServicecatalogProvisionedProduct {
        let out = ServicecatalogProvisionedProduct(Rc::new(ServicecatalogProvisionedProduct_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ServicecatalogProvisionedProductData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                accept_language: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_errors: core::default::Default::default(),
                name: self.name,
                notification_arns: core::default::Default::default(),
                path_id: core::default::Default::default(),
                path_name: core::default::Default::default(),
                product_id: core::default::Default::default(),
                product_name: core::default::Default::default(),
                provisioning_artifact_id: core::default::Default::default(),
                provisioning_artifact_name: core::default::Default::default(),
                region: core::default::Default::default(),
                retain_physical_resources: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                provisioning_parameters: core::default::Default::default(),
                stack_set_provisioning_preferences: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServicecatalogProvisionedProductRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServicecatalogProvisionedProductRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ServicecatalogProvisionedProductRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accept_language", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_dashboard_names` after provisioning.\n"]
    pub fn cloudwatch_dashboard_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_dashboard_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_errors` after provisioning.\n"]
    pub fn ignore_errors(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_errors", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_provisioning_record_id` after provisioning.\n"]
    pub fn last_provisioning_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_provisioning_record_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_record_id` after provisioning.\n"]
    pub fn last_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_record_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_successful_provisioning_record_id` after provisioning.\n"]
    pub fn last_successful_provisioning_record_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.last_successful_provisioning_record_id",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `launch_role_arn` after provisioning.\n"]
    pub fn launch_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `notification_arns` after provisioning.\n"]
    pub fn notification_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> SetRef<ServicecatalogProvisionedProductOutputsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.outputs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `path_id` after provisioning.\n"]
    pub fn path_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `path_name` after provisioning.\n"]
    pub fn path_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `product_name` after provisioning.\n"]
    pub fn product_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `provisioning_artifact_id` after provisioning.\n"]
    pub fn provisioning_artifact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioning_artifact_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `provisioning_artifact_name` after provisioning.\n"]
    pub fn provisioning_artifact_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioning_artifact_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retain_physical_resources` after provisioning.\n"]
    pub fn retain_physical_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retain_physical_resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_message", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `provisioning_parameters` after provisioning.\n"]
    pub fn provisioning_parameters(
        &self,
    ) -> ListRef<ServicecatalogProvisionedProductProvisioningParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioning_parameters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stack_set_provisioning_preferences` after provisioning.\n"]
    pub fn stack_set_provisioning_preferences(
        &self,
    ) -> ListRef<ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stack_set_provisioning_preferences", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ServicecatalogProvisionedProductTimeoutsElRef {
        ServicecatalogProvisionedProductTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ServicecatalogProvisionedProductOutputsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl ServicecatalogProvisionedProductOutputsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for ServicecatalogProvisionedProductOutputsEl {
    type O = BlockAssignable<ServicecatalogProvisionedProductOutputsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicecatalogProvisionedProductOutputsEl {}

impl BuildServicecatalogProvisionedProductOutputsEl {
    pub fn build(self) -> ServicecatalogProvisionedProductOutputsEl {
        ServicecatalogProvisionedProductOutputsEl {
            description: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct ServicecatalogProvisionedProductOutputsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServicecatalogProvisionedProductOutputsElRef {
    fn new(shared: StackShared, base: String) -> ServicecatalogProvisionedProductOutputsElRef {
        ServicecatalogProvisionedProductOutputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicecatalogProvisionedProductOutputsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
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

#[derive(Serialize)]
pub struct ServicecatalogProvisionedProductProvisioningParametersEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_previous_value: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl ServicecatalogProvisionedProductProvisioningParametersEl {
    #[doc = "Set the field `use_previous_value`.\n"]
    pub fn set_use_previous_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_previous_value = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for ServicecatalogProvisionedProductProvisioningParametersEl {
    type O = BlockAssignable<ServicecatalogProvisionedProductProvisioningParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicecatalogProvisionedProductProvisioningParametersEl {
    #[doc = ""]
    pub key: PrimField<String>,
}

impl BuildServicecatalogProvisionedProductProvisioningParametersEl {
    pub fn build(self) -> ServicecatalogProvisionedProductProvisioningParametersEl {
        ServicecatalogProvisionedProductProvisioningParametersEl {
            key: self.key,
            use_previous_value: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct ServicecatalogProvisionedProductProvisioningParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServicecatalogProvisionedProductProvisioningParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ServicecatalogProvisionedProductProvisioningParametersElRef {
        ServicecatalogProvisionedProductProvisioningParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicecatalogProvisionedProductProvisioningParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `use_previous_value` after provisioning.\n"]
    pub fn use_previous_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_previous_value", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_tolerance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_tolerance_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrency_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrency_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<ListField<PrimField<String>>>,
}

impl ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
    #[doc = "Set the field `accounts`.\n"]
    pub fn set_accounts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.accounts = Some(v.into());
        self
    }

    #[doc = "Set the field `failure_tolerance_count`.\n"]
    pub fn set_failure_tolerance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_tolerance_count = Some(v.into());
        self
    }

    #[doc = "Set the field `failure_tolerance_percentage`.\n"]
    pub fn set_failure_tolerance_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_tolerance_percentage = Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrency_count`.\n"]
    pub fn set_max_concurrency_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrency_count = Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrency_percentage`.\n"]
    pub fn set_max_concurrency_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrency_percentage = Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }
}

impl ToListMappable for ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
    type O = BlockAssignable<ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {}

impl BuildServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
    pub fn build(self) -> ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
        ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
            accounts: core::default::Default::default(),
            failure_tolerance_count: core::default::Default::default(),
            failure_tolerance_percentage: core::default::Default::default(),
            max_concurrency_count: core::default::Default::default(),
            max_concurrency_percentage: core::default::Default::default(),
            regions: core::default::Default::default(),
        }
    }
}

pub struct ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef {
        ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_tolerance_count` after provisioning.\n"]
    pub fn failure_tolerance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_tolerance_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `failure_tolerance_percentage` after provisioning.\n"]
    pub fn failure_tolerance_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_tolerance_percentage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_concurrency_count` after provisioning.\n"]
    pub fn max_concurrency_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrency_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_concurrency_percentage` after provisioning.\n"]
    pub fn max_concurrency_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrency_percentage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}

#[derive(Serialize)]
pub struct ServicecatalogProvisionedProductTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ServicecatalogProvisionedProductTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ServicecatalogProvisionedProductTimeoutsEl {
    type O = BlockAssignable<ServicecatalogProvisionedProductTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicecatalogProvisionedProductTimeoutsEl {}

impl BuildServicecatalogProvisionedProductTimeoutsEl {
    pub fn build(self) -> ServicecatalogProvisionedProductTimeoutsEl {
        ServicecatalogProvisionedProductTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ServicecatalogProvisionedProductTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServicecatalogProvisionedProductTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ServicecatalogProvisionedProductTimeoutsElRef {
        ServicecatalogProvisionedProductTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicecatalogProvisionedProductTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServicecatalogProvisionedProductDynamic {
    provisioning_parameters:
        Option<DynamicBlock<ServicecatalogProvisionedProductProvisioningParametersEl>>,
    stack_set_provisioning_preferences:
        Option<DynamicBlock<ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl>>,
}
