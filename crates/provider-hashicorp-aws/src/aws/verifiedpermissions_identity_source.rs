use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VerifiedpermissionsIdentitySourceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    policy_store_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_entity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<VerifiedpermissionsIdentitySourceConfigurationEl>>,
    dynamic: VerifiedpermissionsIdentitySourceDynamic,
}

struct VerifiedpermissionsIdentitySource_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VerifiedpermissionsIdentitySourceData>,
}

#[derive(Clone)]
pub struct VerifiedpermissionsIdentitySource(Rc<VerifiedpermissionsIdentitySource_>);

impl VerifiedpermissionsIdentitySource {
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

    #[doc = "Set the field `principal_entity_type`.\n"]
    pub fn set_principal_entity_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().principal_entity_type = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl Into<BlockAssignable<VerifiedpermissionsIdentitySourceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_store_id` after provisioning.\n"]
    pub fn policy_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_entity_type` after provisioning.\n"]
    pub fn principal_entity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal_entity_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<VerifiedpermissionsIdentitySourceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl Referable for VerifiedpermissionsIdentitySource {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VerifiedpermissionsIdentitySource { }

impl ToListMappable for VerifiedpermissionsIdentitySource {
    type O = ListRef<VerifiedpermissionsIdentitySourceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VerifiedpermissionsIdentitySource_ {
    fn extract_resource_type(&self) -> String {
        "aws_verifiedpermissions_identity_source".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVerifiedpermissionsIdentitySource {
    pub tf_id: String,
    #[doc = ""]
    pub policy_store_id: PrimField<String>,
}

impl BuildVerifiedpermissionsIdentitySource {
    pub fn build(self, stack: &mut Stack) -> VerifiedpermissionsIdentitySource {
        let out = VerifiedpermissionsIdentitySource(Rc::new(VerifiedpermissionsIdentitySource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VerifiedpermissionsIdentitySourceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                policy_store_id: self.policy_store_id,
                principal_entity_type: core::default::Default::default(),
                region: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VerifiedpermissionsIdentitySourceRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsIdentitySourceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl VerifiedpermissionsIdentitySourceRef {
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

    #[doc = "Get a reference to the value of field `policy_store_id` after provisioning.\n"]
    pub fn policy_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_entity_type` after provisioning.\n"]
    pub fn principal_entity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal_entity_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<VerifiedpermissionsIdentitySourceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationEl {
    group_entity_type: PrimField<String>,
}

impl VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationEl { }

impl ToListMappable for VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationEl {
    type O =
        BlockAssignable<
            VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationEl {
    #[doc = ""]
    pub group_entity_type: PrimField<String>,
}

impl BuildVerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationEl {
    pub fn build(
        self,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationEl {
        VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationEl {
            group_entity_type: self.group_entity_type,
        }
    }
}

pub struct VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationElRef {
        VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_entity_type` after provisioning.\n"]
    pub fn group_entity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_entity_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElDynamic {
    group_configuration: Option<
        DynamicBlock<
            VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_ids: Option<ListField<PrimField<String>>>,
    user_pool_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_configuration: Option<
        Vec<VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationEl>,
    >,
    dynamic: VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElDynamic,
}

impl VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationEl {
    #[doc = "Set the field `client_ids`.\n"]
    pub fn set_client_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.client_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `group_configuration`.\n"]
    pub fn set_group_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.group_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.group_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationEl {
    type O = BlockAssignable<VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationEl {
    #[doc = ""]
    pub user_pool_arn: PrimField<String>,
}

impl BuildVerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationEl {
    pub fn build(self) -> VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationEl {
        VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationEl {
            client_ids: core::default::Default::default(),
            user_pool_arn: self.user_pool_arn,
            group_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElRef {
        VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_ids` after provisioning.\n"]
    pub fn client_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.client_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_arn` after provisioning.\n"]
    pub fn user_pool_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `group_configuration` after provisioning.\n"]
    pub fn group_configuration(
        &self,
    ) -> ListRef<
        VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElGroupConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.group_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationEl {
    group_claim: PrimField<String>,
    group_entity_type: PrimField<String>,
}

impl VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationEl { }

impl ToListMappable for VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationEl {
    type O =
        BlockAssignable<
            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationEl {
    #[doc = ""]
    pub group_claim: PrimField<String>,
    #[doc = ""]
    pub group_entity_type: PrimField<String>,
}

impl BuildVerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationEl {
    pub fn build(
        self,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationEl {
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationEl {
            group_claim: self.group_claim,
            group_entity_type: self.group_entity_type,
        }
    }
}

pub struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationElRef {
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_claim` after provisioning.\n"]
    pub fn group_claim(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_claim", self.base))
    }

    #[doc = "Get a reference to the value of field `group_entity_type` after provisioning.\n"]
    pub fn group_entity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_entity_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audiences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_id_claim: Option<PrimField<String>>,
}

impl VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyEl {
    #[doc = "Set the field `audiences`.\n"]
    pub fn set_audiences(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.audiences = Some(v.into());
        self
    }

    #[doc = "Set the field `principal_id_claim`.\n"]
    pub fn set_principal_id_claim(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal_id_claim = Some(v.into());
        self
    }
}

impl ToListMappable for VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyEl {
    type O =
        BlockAssignable<
            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyEl {}

impl BuildVerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyEl {
    pub fn build(
        self,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyEl {
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyEl {
            audiences: core::default::Default::default(),
            principal_id_claim: core::default::Default::default(),
        }
    }
}

pub struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyElRef {
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audiences` after provisioning.\n"]
    pub fn audiences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.audiences", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_id_claim` after provisioning.\n"]
    pub fn principal_id_claim(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal_id_claim", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_id_claim: Option<PrimField<String>>,
}

impl VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyEl {
    #[doc = "Set the field `client_ids`.\n"]
    pub fn set_client_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.client_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `principal_id_claim`.\n"]
    pub fn set_principal_id_claim(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal_id_claim = Some(v.into());
        self
    }
}

impl ToListMappable for VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyEl {
    type O =
        BlockAssignable<
            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyEl {}

impl BuildVerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyEl {
    pub fn build(
        self,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyEl {
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyEl {
            client_ids: core::default::Default::default(),
            principal_id_claim: core::default::Default::default(),
        }
    }
}

pub struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyElRef {
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_ids` after provisioning.\n"]
    pub fn client_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.client_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_id_claim` after provisioning.\n"]
    pub fn principal_id_claim(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal_id_claim", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElDynamic {
    access_token_only: Option<
        DynamicBlock<
            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyEl,
        >,
    >,
    identity_token_only: Option<
        DynamicBlock<
            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token_only: Option<
        Vec<
            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_token_only: Option<
        Vec<
            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyEl,
        >,
    >,
    dynamic: VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElDynamic,
}

impl VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionEl {
    #[doc = "Set the field `access_token_only`.\n"]
    pub fn set_access_token_only(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_token_only = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_token_only = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identity_token_only`.\n"]
    pub fn set_identity_token_only(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.identity_token_only = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_token_only = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionEl {
    type O =
        BlockAssignable<
            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionEl {}

impl BuildVerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionEl {
    pub fn build(
        self,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionEl {
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionEl {
            access_token_only: core::default::Default::default(),
            identity_token_only: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElRef {
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token_only` after provisioning.\n"]
    pub fn access_token_only(
        &self,
    ) -> ListRef<
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElAccessTokenOnlyElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.access_token_only", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_token_only` after provisioning.\n"]
    pub fn identity_token_only(
        &self,
    ) -> ListRef<
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElIdentityTokenOnlyElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.identity_token_only", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElDynamic {
    group_configuration: Option<
        DynamicBlock<VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationEl>,
    >,
    token_selection: Option<
        DynamicBlock<VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionEl>,
    >,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_id_prefix: Option<PrimField<String>>,
    issuer: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_configuration: Option<
        Vec<VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_selection: Option<
        Vec<VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionEl>,
    >,
    dynamic: VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElDynamic,
}

impl VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationEl {
    #[doc = "Set the field `entity_id_prefix`.\n"]
    pub fn set_entity_id_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entity_id_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `group_configuration`.\n"]
    pub fn set_group_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.group_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.group_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `token_selection`.\n"]
    pub fn set_token_selection(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.token_selection = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.token_selection = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationEl {
    type O = BlockAssignable<VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationEl {
    #[doc = ""]
    pub issuer: PrimField<String>,
}

impl BuildVerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationEl {
    pub fn build(self) -> VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationEl {
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationEl {
            entity_id_prefix: core::default::Default::default(),
            issuer: self.issuer,
            group_configuration: core::default::Default::default(),
            token_selection: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElRef {
        VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id_prefix` after provisioning.\n"]
    pub fn entity_id_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_id_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `group_configuration` after provisioning.\n"]
    pub fn group_configuration(
        &self,
    ) -> ListRef<VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElGroupConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.group_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `token_selection` after provisioning.\n"]
    pub fn token_selection(
        &self,
    ) -> ListRef<VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElTokenSelectionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.token_selection", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsIdentitySourceConfigurationElDynamic {
    cognito_user_pool_configuration: Option<
        DynamicBlock<VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationEl>,
    >,
    open_id_connect_configuration: Option<
        DynamicBlock<VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsIdentitySourceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cognito_user_pool_configuration: Option<
        Vec<VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_id_connect_configuration: Option<
        Vec<VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationEl>,
    >,
    dynamic: VerifiedpermissionsIdentitySourceConfigurationElDynamic,
}

impl VerifiedpermissionsIdentitySourceConfigurationEl {
    #[doc = "Set the field `cognito_user_pool_configuration`.\n"]
    pub fn set_cognito_user_pool_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cognito_user_pool_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cognito_user_pool_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `open_id_connect_configuration`.\n"]
    pub fn set_open_id_connect_configuration(
        mut self,
        v: impl Into<BlockAssignable<VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.open_id_connect_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.open_id_connect_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for VerifiedpermissionsIdentitySourceConfigurationEl {
    type O = BlockAssignable<VerifiedpermissionsIdentitySourceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsIdentitySourceConfigurationEl {}

impl BuildVerifiedpermissionsIdentitySourceConfigurationEl {
    pub fn build(self) -> VerifiedpermissionsIdentitySourceConfigurationEl {
        VerifiedpermissionsIdentitySourceConfigurationEl {
            cognito_user_pool_configuration: core::default::Default::default(),
            open_id_connect_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VerifiedpermissionsIdentitySourceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsIdentitySourceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> VerifiedpermissionsIdentitySourceConfigurationElRef {
        VerifiedpermissionsIdentitySourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsIdentitySourceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cognito_user_pool_configuration` after provisioning.\n"]
    pub fn cognito_user_pool_configuration(
        &self,
    ) -> ListRef<VerifiedpermissionsIdentitySourceConfigurationElCognitoUserPoolConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cognito_user_pool_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `open_id_connect_configuration` after provisioning.\n"]
    pub fn open_id_connect_configuration(
        &self,
    ) -> ListRef<VerifiedpermissionsIdentitySourceConfigurationElOpenIdConnectConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.open_id_connect_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsIdentitySourceDynamic {
    configuration: Option<DynamicBlock<VerifiedpermissionsIdentitySourceConfigurationEl>>,
}
