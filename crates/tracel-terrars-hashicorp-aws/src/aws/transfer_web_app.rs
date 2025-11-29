use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TransferWebAppData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_app_endpoint_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_app_units: Option<ListField<TransferWebAppWebAppUnitsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_details: Option<Vec<TransferWebAppIdentityProviderDetailsEl>>,
    dynamic: TransferWebAppDynamic,
}

struct TransferWebApp_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TransferWebAppData>,
}

#[derive(Clone)]
pub struct TransferWebApp(Rc<TransferWebApp_>);

impl TransferWebApp {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `access_endpoint`.\n"]
    pub fn set_access_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_endpoint = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `web_app_endpoint_policy`.\n"]
    pub fn set_web_app_endpoint_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().web_app_endpoint_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `web_app_units`.\n"]
    pub fn set_web_app_units(self, v: impl Into<ListField<TransferWebAppWebAppUnitsEl>>) -> Self {
        self.0.data.borrow_mut().web_app_units = Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_details`.\n"]
    pub fn set_identity_provider_details(
        self,
        v: impl Into<BlockAssignable<TransferWebAppIdentityProviderDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_provider_details = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_provider_details = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `access_endpoint` after provisioning.\n"]
    pub fn access_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_app_endpoint_policy` after provisioning.\n"]
    pub fn web_app_endpoint_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_app_endpoint_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_app_id` after provisioning.\n"]
    pub fn web_app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_app_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_app_units` after provisioning.\n"]
    pub fn web_app_units(&self) -> ListRef<TransferWebAppWebAppUnitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.web_app_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_provider_details` after provisioning.\n"]
    pub fn identity_provider_details(&self) -> ListRef<TransferWebAppIdentityProviderDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity_provider_details", self.extract_ref()))
    }
}

impl Referable for TransferWebApp {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TransferWebApp { }

impl ToListMappable for TransferWebApp {
    type O = ListRef<TransferWebAppRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TransferWebApp_ {
    fn extract_resource_type(&self) -> String {
        "aws_transfer_web_app".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTransferWebApp {
    pub tf_id: String,
}

impl BuildTransferWebApp {
    pub fn build(self, stack: &mut Stack) -> TransferWebApp {
        let out = TransferWebApp(Rc::new(TransferWebApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TransferWebAppData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_endpoint: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                web_app_endpoint_policy: core::default::Default::default(),
                web_app_units: core::default::Default::default(),
                identity_provider_details: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TransferWebAppRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWebAppRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl TransferWebAppRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_endpoint` after provisioning.\n"]
    pub fn access_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_app_endpoint_policy` after provisioning.\n"]
    pub fn web_app_endpoint_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_app_endpoint_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_app_id` after provisioning.\n"]
    pub fn web_app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_app_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_app_units` after provisioning.\n"]
    pub fn web_app_units(&self) -> ListRef<TransferWebAppWebAppUnitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.web_app_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_provider_details` after provisioning.\n"]
    pub fn identity_provider_details(&self) -> ListRef<TransferWebAppIdentityProviderDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity_provider_details", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TransferWebAppWebAppUnitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned: Option<PrimField<f64>>,
}

impl TransferWebAppWebAppUnitsEl {
    #[doc = "Set the field `provisioned`.\n"]
    pub fn set_provisioned(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned = Some(v.into());
        self
    }
}

impl ToListMappable for TransferWebAppWebAppUnitsEl {
    type O = BlockAssignable<TransferWebAppWebAppUnitsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWebAppWebAppUnitsEl {}

impl BuildTransferWebAppWebAppUnitsEl {
    pub fn build(self) -> TransferWebAppWebAppUnitsEl {
        TransferWebAppWebAppUnitsEl { provisioned: core::default::Default::default() }
    }
}

pub struct TransferWebAppWebAppUnitsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWebAppWebAppUnitsElRef {
    fn new(shared: StackShared, base: String) -> TransferWebAppWebAppUnitsElRef {
        TransferWebAppWebAppUnitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWebAppWebAppUnitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provisioned` after provisioning.\n"]
    pub fn provisioned(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWebAppIdentityProviderDetailsElIdentityCenterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
}

impl TransferWebAppIdentityProviderDetailsElIdentityCenterConfigEl {
    #[doc = "Set the field `instance_arn`.\n"]
    pub fn set_instance_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }
}

impl ToListMappable for TransferWebAppIdentityProviderDetailsElIdentityCenterConfigEl {
    type O = BlockAssignable<TransferWebAppIdentityProviderDetailsElIdentityCenterConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWebAppIdentityProviderDetailsElIdentityCenterConfigEl {}

impl BuildTransferWebAppIdentityProviderDetailsElIdentityCenterConfigEl {
    pub fn build(self) -> TransferWebAppIdentityProviderDetailsElIdentityCenterConfigEl {
        TransferWebAppIdentityProviderDetailsElIdentityCenterConfigEl {
            instance_arn: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}

pub struct TransferWebAppIdentityProviderDetailsElIdentityCenterConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWebAppIdentityProviderDetailsElIdentityCenterConfigElRef {
    fn new(shared: StackShared, base: String) -> TransferWebAppIdentityProviderDetailsElIdentityCenterConfigElRef {
        TransferWebAppIdentityProviderDetailsElIdentityCenterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWebAppIdentityProviderDetailsElIdentityCenterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWebAppIdentityProviderDetailsElDynamic {
    identity_center_config: Option<DynamicBlock<TransferWebAppIdentityProviderDetailsElIdentityCenterConfigEl>>,
}

#[derive(Serialize)]
pub struct TransferWebAppIdentityProviderDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_center_config: Option<Vec<TransferWebAppIdentityProviderDetailsElIdentityCenterConfigEl>>,
    dynamic: TransferWebAppIdentityProviderDetailsElDynamic,
}

impl TransferWebAppIdentityProviderDetailsEl {
    #[doc = "Set the field `identity_center_config`.\n"]
    pub fn set_identity_center_config(
        mut self,
        v: impl Into<BlockAssignable<TransferWebAppIdentityProviderDetailsElIdentityCenterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.identity_center_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_center_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for TransferWebAppIdentityProviderDetailsEl {
    type O = BlockAssignable<TransferWebAppIdentityProviderDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWebAppIdentityProviderDetailsEl {}

impl BuildTransferWebAppIdentityProviderDetailsEl {
    pub fn build(self) -> TransferWebAppIdentityProviderDetailsEl {
        TransferWebAppIdentityProviderDetailsEl {
            identity_center_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct TransferWebAppIdentityProviderDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferWebAppIdentityProviderDetailsElRef {
    fn new(shared: StackShared, base: String) -> TransferWebAppIdentityProviderDetailsElRef {
        TransferWebAppIdentityProviderDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWebAppIdentityProviderDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_center_config` after provisioning.\n"]
    pub fn identity_center_config(&self) -> ListRef<TransferWebAppIdentityProviderDetailsElIdentityCenterConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity_center_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWebAppDynamic {
    identity_provider_details: Option<DynamicBlock<TransferWebAppIdentityProviderDetailsEl>>,
}
