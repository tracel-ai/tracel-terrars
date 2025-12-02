use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct QuicksightDataSourceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<PrimField<String>>,
    data_source_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<Vec<QuicksightDataSourceCredentialsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<QuicksightDataSourceParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<Vec<QuicksightDataSourcePermissionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_properties: Option<Vec<QuicksightDataSourceSslPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_connection_properties: Option<Vec<QuicksightDataSourceVpcConnectionPropertiesEl>>,
    dynamic: QuicksightDataSourceDynamic,
}
struct QuicksightDataSource_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<QuicksightDataSourceData>,
}
#[derive(Clone)]
pub struct QuicksightDataSource(Rc<QuicksightDataSource_>);
impl QuicksightDataSource {
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
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = Some(v.into());
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
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSourceCredentialsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.credentials = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `permission`.\n"]
    pub fn set_permission(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSourcePermissionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().permission = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.permission = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ssl_properties`.\n"]
    pub fn set_ssl_properties(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSourceSslPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ssl_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ssl_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_connection_properties`.\n"]
    pub fn set_vpc_connection_properties(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSourceVpcConnectionPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_connection_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_connection_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> ListRef<QuicksightDataSourceCredentialsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.credentials", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<QuicksightDataSourceParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_properties` after provisioning.\n"]
    pub fn ssl_properties(&self) -> ListRef<QuicksightDataSourceSslPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssl_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_connection_properties` after provisioning.\n"]
    pub fn vpc_connection_properties(
        &self,
    ) -> ListRef<QuicksightDataSourceVpcConnectionPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_connection_properties", self.extract_ref()),
        )
    }
}
impl Referable for QuicksightDataSource {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for QuicksightDataSource {}
impl ToListMappable for QuicksightDataSource {
    type O = ListRef<QuicksightDataSourceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for QuicksightDataSource_ {
    fn extract_resource_type(&self) -> String {
        "aws_quicksight_data_source".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildQuicksightDataSource {
    pub tf_id: String,
    #[doc = ""]
    pub data_source_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildQuicksightDataSource {
    pub fn build(self, stack: &mut Stack) -> QuicksightDataSource {
        let out = QuicksightDataSource(Rc::new(QuicksightDataSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(QuicksightDataSourceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                aws_account_id: core::default::Default::default(),
                data_source_id: self.data_source_id,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                credentials: core::default::Default::default(),
                parameters: core::default::Default::default(),
                permission: core::default::Default::default(),
                ssl_properties: core::default::Default::default(),
                vpc_connection_properties: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct QuicksightDataSourceRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl QuicksightDataSourceRef {
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
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> ListRef<QuicksightDataSourceCredentialsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.credentials", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<QuicksightDataSourceParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_properties` after provisioning.\n"]
    pub fn ssl_properties(&self) -> ListRef<QuicksightDataSourceSslPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssl_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_connection_properties` after provisioning.\n"]
    pub fn vpc_connection_properties(
        &self,
    ) -> ListRef<QuicksightDataSourceVpcConnectionPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_connection_properties", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceCredentialsElCredentialPairEl {
    password: PrimField<String>,
    username: PrimField<String>,
}
impl QuicksightDataSourceCredentialsElCredentialPairEl {}
impl ToListMappable for QuicksightDataSourceCredentialsElCredentialPairEl {
    type O = BlockAssignable<QuicksightDataSourceCredentialsElCredentialPairEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceCredentialsElCredentialPairEl {
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildQuicksightDataSourceCredentialsElCredentialPairEl {
    pub fn build(self) -> QuicksightDataSourceCredentialsElCredentialPairEl {
        QuicksightDataSourceCredentialsElCredentialPairEl {
            password: self.password,
            username: self.username,
        }
    }
}
pub struct QuicksightDataSourceCredentialsElCredentialPairElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceCredentialsElCredentialPairElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSourceCredentialsElCredentialPairElRef {
        QuicksightDataSourceCredentialsElCredentialPairElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceCredentialsElCredentialPairElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSourceCredentialsElDynamic {
    credential_pair: Option<DynamicBlock<QuicksightDataSourceCredentialsElCredentialPairEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSourceCredentialsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_source_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credential_pair: Option<Vec<QuicksightDataSourceCredentialsElCredentialPairEl>>,
    dynamic: QuicksightDataSourceCredentialsElDynamic,
}
impl QuicksightDataSourceCredentialsEl {
    #[doc = "Set the field `copy_source_arn`.\n"]
    pub fn set_copy_source_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.copy_source_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `credential_pair`.\n"]
    pub fn set_credential_pair(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceCredentialsElCredentialPairEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.credential_pair = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.credential_pair = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSourceCredentialsEl {
    type O = BlockAssignable<QuicksightDataSourceCredentialsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceCredentialsEl {}
impl BuildQuicksightDataSourceCredentialsEl {
    pub fn build(self) -> QuicksightDataSourceCredentialsEl {
        QuicksightDataSourceCredentialsEl {
            copy_source_arn: core::default::Default::default(),
            secret_arn: core::default::Default::default(),
            credential_pair: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSourceCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceCredentialsElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceCredentialsElRef {
        QuicksightDataSourceCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `copy_source_arn` after provisioning.\n"]
    pub fn copy_source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_source_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `credential_pair` after provisioning.\n"]
    pub fn credential_pair(&self) -> ListRef<QuicksightDataSourceCredentialsElCredentialPairElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.credential_pair", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElAmazonElasticsearchEl {
    domain: PrimField<String>,
}
impl QuicksightDataSourceParametersElAmazonElasticsearchEl {}
impl ToListMappable for QuicksightDataSourceParametersElAmazonElasticsearchEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElAmazonElasticsearchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElAmazonElasticsearchEl {
    #[doc = ""]
    pub domain: PrimField<String>,
}
impl BuildQuicksightDataSourceParametersElAmazonElasticsearchEl {
    pub fn build(self) -> QuicksightDataSourceParametersElAmazonElasticsearchEl {
        QuicksightDataSourceParametersElAmazonElasticsearchEl {
            domain: self.domain,
        }
    }
}
pub struct QuicksightDataSourceParametersElAmazonElasticsearchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElAmazonElasticsearchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSourceParametersElAmazonElasticsearchElRef {
        QuicksightDataSourceParametersElAmazonElasticsearchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElAmazonElasticsearchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElAthenaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    work_group: Option<PrimField<String>>,
}
impl QuicksightDataSourceParametersElAthenaEl {
    #[doc = "Set the field `work_group`.\n"]
    pub fn set_work_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.work_group = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightDataSourceParametersElAthenaEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElAthenaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElAthenaEl {}
impl BuildQuicksightDataSourceParametersElAthenaEl {
    pub fn build(self) -> QuicksightDataSourceParametersElAthenaEl {
        QuicksightDataSourceParametersElAthenaEl {
            work_group: core::default::Default::default(),
        }
    }
}
pub struct QuicksightDataSourceParametersElAthenaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElAthenaElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElAthenaElRef {
        QuicksightDataSourceParametersElAthenaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElAthenaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `work_group` after provisioning.\n"]
    pub fn work_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.work_group", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElAuroraEl {
    database: PrimField<String>,
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl QuicksightDataSourceParametersElAuroraEl {}
impl ToListMappable for QuicksightDataSourceParametersElAuroraEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElAuroraEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElAuroraEl {
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildQuicksightDataSourceParametersElAuroraEl {
    pub fn build(self) -> QuicksightDataSourceParametersElAuroraEl {
        QuicksightDataSourceParametersElAuroraEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}
pub struct QuicksightDataSourceParametersElAuroraElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElAuroraElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElAuroraElRef {
        QuicksightDataSourceParametersElAuroraElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElAuroraElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElAuroraPostgresqlEl {
    database: PrimField<String>,
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl QuicksightDataSourceParametersElAuroraPostgresqlEl {}
impl ToListMappable for QuicksightDataSourceParametersElAuroraPostgresqlEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElAuroraPostgresqlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElAuroraPostgresqlEl {
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildQuicksightDataSourceParametersElAuroraPostgresqlEl {
    pub fn build(self) -> QuicksightDataSourceParametersElAuroraPostgresqlEl {
        QuicksightDataSourceParametersElAuroraPostgresqlEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}
pub struct QuicksightDataSourceParametersElAuroraPostgresqlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElAuroraPostgresqlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSourceParametersElAuroraPostgresqlElRef {
        QuicksightDataSourceParametersElAuroraPostgresqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElAuroraPostgresqlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElAwsIotAnalyticsEl {
    data_set_name: PrimField<String>,
}
impl QuicksightDataSourceParametersElAwsIotAnalyticsEl {}
impl ToListMappable for QuicksightDataSourceParametersElAwsIotAnalyticsEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElAwsIotAnalyticsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElAwsIotAnalyticsEl {
    #[doc = ""]
    pub data_set_name: PrimField<String>,
}
impl BuildQuicksightDataSourceParametersElAwsIotAnalyticsEl {
    pub fn build(self) -> QuicksightDataSourceParametersElAwsIotAnalyticsEl {
        QuicksightDataSourceParametersElAwsIotAnalyticsEl {
            data_set_name: self.data_set_name,
        }
    }
}
pub struct QuicksightDataSourceParametersElAwsIotAnalyticsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElAwsIotAnalyticsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSourceParametersElAwsIotAnalyticsElRef {
        QuicksightDataSourceParametersElAwsIotAnalyticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElAwsIotAnalyticsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_set_name` after provisioning.\n"]
    pub fn data_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_set_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElDatabricksEl {
    host: PrimField<String>,
    port: PrimField<f64>,
    sql_endpoint_path: PrimField<String>,
}
impl QuicksightDataSourceParametersElDatabricksEl {}
impl ToListMappable for QuicksightDataSourceParametersElDatabricksEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElDatabricksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElDatabricksEl {
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
    #[doc = ""]
    pub sql_endpoint_path: PrimField<String>,
}
impl BuildQuicksightDataSourceParametersElDatabricksEl {
    pub fn build(self) -> QuicksightDataSourceParametersElDatabricksEl {
        QuicksightDataSourceParametersElDatabricksEl {
            host: self.host,
            port: self.port,
            sql_endpoint_path: self.sql_endpoint_path,
        }
    }
}
pub struct QuicksightDataSourceParametersElDatabricksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElDatabricksElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElDatabricksElRef {
        QuicksightDataSourceParametersElDatabricksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElDatabricksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `sql_endpoint_path` after provisioning.\n"]
    pub fn sql_endpoint_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_endpoint_path", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElJiraEl {
    site_base_url: PrimField<String>,
}
impl QuicksightDataSourceParametersElJiraEl {}
impl ToListMappable for QuicksightDataSourceParametersElJiraEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElJiraEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElJiraEl {
    #[doc = ""]
    pub site_base_url: PrimField<String>,
}
impl BuildQuicksightDataSourceParametersElJiraEl {
    pub fn build(self) -> QuicksightDataSourceParametersElJiraEl {
        QuicksightDataSourceParametersElJiraEl {
            site_base_url: self.site_base_url,
        }
    }
}
pub struct QuicksightDataSourceParametersElJiraElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElJiraElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElJiraElRef {
        QuicksightDataSourceParametersElJiraElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElJiraElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `site_base_url` after provisioning.\n"]
    pub fn site_base_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site_base_url", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElMariaDbEl {
    database: PrimField<String>,
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl QuicksightDataSourceParametersElMariaDbEl {}
impl ToListMappable for QuicksightDataSourceParametersElMariaDbEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElMariaDbEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElMariaDbEl {
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildQuicksightDataSourceParametersElMariaDbEl {
    pub fn build(self) -> QuicksightDataSourceParametersElMariaDbEl {
        QuicksightDataSourceParametersElMariaDbEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}
pub struct QuicksightDataSourceParametersElMariaDbElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElMariaDbElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElMariaDbElRef {
        QuicksightDataSourceParametersElMariaDbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElMariaDbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElMysqlEl {
    database: PrimField<String>,
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl QuicksightDataSourceParametersElMysqlEl {}
impl ToListMappable for QuicksightDataSourceParametersElMysqlEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElMysqlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElMysqlEl {
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildQuicksightDataSourceParametersElMysqlEl {
    pub fn build(self) -> QuicksightDataSourceParametersElMysqlEl {
        QuicksightDataSourceParametersElMysqlEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}
pub struct QuicksightDataSourceParametersElMysqlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElMysqlElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElMysqlElRef {
        QuicksightDataSourceParametersElMysqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElMysqlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElOracleEl {
    database: PrimField<String>,
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl QuicksightDataSourceParametersElOracleEl {}
impl ToListMappable for QuicksightDataSourceParametersElOracleEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElOracleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElOracleEl {
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildQuicksightDataSourceParametersElOracleEl {
    pub fn build(self) -> QuicksightDataSourceParametersElOracleEl {
        QuicksightDataSourceParametersElOracleEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}
pub struct QuicksightDataSourceParametersElOracleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElOracleElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElOracleElRef {
        QuicksightDataSourceParametersElOracleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElOracleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElPostgresqlEl {
    database: PrimField<String>,
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl QuicksightDataSourceParametersElPostgresqlEl {}
impl ToListMappable for QuicksightDataSourceParametersElPostgresqlEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElPostgresqlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElPostgresqlEl {
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildQuicksightDataSourceParametersElPostgresqlEl {
    pub fn build(self) -> QuicksightDataSourceParametersElPostgresqlEl {
        QuicksightDataSourceParametersElPostgresqlEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}
pub struct QuicksightDataSourceParametersElPostgresqlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElPostgresqlElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElPostgresqlElRef {
        QuicksightDataSourceParametersElPostgresqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElPostgresqlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElPrestoEl {
    catalog: PrimField<String>,
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl QuicksightDataSourceParametersElPrestoEl {}
impl ToListMappable for QuicksightDataSourceParametersElPrestoEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElPrestoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElPrestoEl {
    #[doc = ""]
    pub catalog: PrimField<String>,
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildQuicksightDataSourceParametersElPrestoEl {
    pub fn build(self) -> QuicksightDataSourceParametersElPrestoEl {
        QuicksightDataSourceParametersElPrestoEl {
            catalog: self.catalog,
            host: self.host,
            port: self.port,
        }
    }
}
pub struct QuicksightDataSourceParametersElPrestoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElPrestoElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElPrestoElRef {
        QuicksightDataSourceParametersElPrestoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElPrestoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog` after provisioning.\n"]
    pub fn catalog(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElRdsEl {
    database: PrimField<String>,
    instance_id: PrimField<String>,
}
impl QuicksightDataSourceParametersElRdsEl {}
impl ToListMappable for QuicksightDataSourceParametersElRdsEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElRdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElRdsEl {
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub instance_id: PrimField<String>,
}
impl BuildQuicksightDataSourceParametersElRdsEl {
    pub fn build(self) -> QuicksightDataSourceParametersElRdsEl {
        QuicksightDataSourceParametersElRdsEl {
            database: self.database,
            instance_id: self.instance_id,
        }
    }
}
pub struct QuicksightDataSourceParametersElRdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElRdsElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElRdsElRef {
        QuicksightDataSourceParametersElRdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElRdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElRedshiftEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_id: Option<PrimField<String>>,
    database: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl QuicksightDataSourceParametersElRedshiftEl {
    #[doc = "Set the field `cluster_id`.\n"]
    pub fn set_cluster_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_id = Some(v.into());
        self
    }
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightDataSourceParametersElRedshiftEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElRedshiftEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElRedshiftEl {
    #[doc = ""]
    pub database: PrimField<String>,
}
impl BuildQuicksightDataSourceParametersElRedshiftEl {
    pub fn build(self) -> QuicksightDataSourceParametersElRedshiftEl {
        QuicksightDataSourceParametersElRedshiftEl {
            cluster_id: core::default::Default::default(),
            database: self.database,
            host: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct QuicksightDataSourceParametersElRedshiftElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElRedshiftElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElRedshiftElRef {
        QuicksightDataSourceParametersElRedshiftElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElRedshiftElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElS3ElManifestFileLocationEl {
    bucket: PrimField<String>,
    key: PrimField<String>,
}
impl QuicksightDataSourceParametersElS3ElManifestFileLocationEl {}
impl ToListMappable for QuicksightDataSourceParametersElS3ElManifestFileLocationEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElS3ElManifestFileLocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElS3ElManifestFileLocationEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
}
impl BuildQuicksightDataSourceParametersElS3ElManifestFileLocationEl {
    pub fn build(self) -> QuicksightDataSourceParametersElS3ElManifestFileLocationEl {
        QuicksightDataSourceParametersElS3ElManifestFileLocationEl {
            bucket: self.bucket,
            key: self.key,
        }
    }
}
pub struct QuicksightDataSourceParametersElS3ElManifestFileLocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElS3ElManifestFileLocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSourceParametersElS3ElManifestFileLocationElRef {
        QuicksightDataSourceParametersElS3ElManifestFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElS3ElManifestFileLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSourceParametersElS3ElDynamic {
    manifest_file_location:
        Option<DynamicBlock<QuicksightDataSourceParametersElS3ElManifestFileLocationEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manifest_file_location: Option<Vec<QuicksightDataSourceParametersElS3ElManifestFileLocationEl>>,
    dynamic: QuicksightDataSourceParametersElS3ElDynamic,
}
impl QuicksightDataSourceParametersElS3El {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `manifest_file_location`.\n"]
    pub fn set_manifest_file_location(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElS3ElManifestFileLocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.manifest_file_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.manifest_file_location = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSourceParametersElS3El {
    type O = BlockAssignable<QuicksightDataSourceParametersElS3El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElS3El {}
impl BuildQuicksightDataSourceParametersElS3El {
    pub fn build(self) -> QuicksightDataSourceParametersElS3El {
        QuicksightDataSourceParametersElS3El {
            role_arn: core::default::Default::default(),
            manifest_file_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSourceParametersElS3ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElS3ElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElS3ElRef {
        QuicksightDataSourceParametersElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `manifest_file_location` after provisioning.\n"]
    pub fn manifest_file_location(
        &self,
    ) -> ListRef<QuicksightDataSourceParametersElS3ElManifestFileLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.manifest_file_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElServiceNowEl {
    site_base_url: PrimField<String>,
}
impl QuicksightDataSourceParametersElServiceNowEl {}
impl ToListMappable for QuicksightDataSourceParametersElServiceNowEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElServiceNowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElServiceNowEl {
    #[doc = ""]
    pub site_base_url: PrimField<String>,
}
impl BuildQuicksightDataSourceParametersElServiceNowEl {
    pub fn build(self) -> QuicksightDataSourceParametersElServiceNowEl {
        QuicksightDataSourceParametersElServiceNowEl {
            site_base_url: self.site_base_url,
        }
    }
}
pub struct QuicksightDataSourceParametersElServiceNowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElServiceNowElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElServiceNowElRef {
        QuicksightDataSourceParametersElServiceNowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElServiceNowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `site_base_url` after provisioning.\n"]
    pub fn site_base_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site_base_url", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElSnowflakeEl {
    database: PrimField<String>,
    host: PrimField<String>,
    warehouse: PrimField<String>,
}
impl QuicksightDataSourceParametersElSnowflakeEl {}
impl ToListMappable for QuicksightDataSourceParametersElSnowflakeEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElSnowflakeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElSnowflakeEl {
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub warehouse: PrimField<String>,
}
impl BuildQuicksightDataSourceParametersElSnowflakeEl {
    pub fn build(self) -> QuicksightDataSourceParametersElSnowflakeEl {
        QuicksightDataSourceParametersElSnowflakeEl {
            database: self.database,
            host: self.host,
            warehouse: self.warehouse,
        }
    }
}
pub struct QuicksightDataSourceParametersElSnowflakeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElSnowflakeElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElSnowflakeElRef {
        QuicksightDataSourceParametersElSnowflakeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElSnowflakeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `warehouse` after provisioning.\n"]
    pub fn warehouse(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.warehouse", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElSparkEl {
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl QuicksightDataSourceParametersElSparkEl {}
impl ToListMappable for QuicksightDataSourceParametersElSparkEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElSparkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElSparkEl {
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildQuicksightDataSourceParametersElSparkEl {
    pub fn build(self) -> QuicksightDataSourceParametersElSparkEl {
        QuicksightDataSourceParametersElSparkEl {
            host: self.host,
            port: self.port,
        }
    }
}
pub struct QuicksightDataSourceParametersElSparkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElSparkElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElSparkElRef {
        QuicksightDataSourceParametersElSparkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElSparkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElSqlServerEl {
    database: PrimField<String>,
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl QuicksightDataSourceParametersElSqlServerEl {}
impl ToListMappable for QuicksightDataSourceParametersElSqlServerEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElSqlServerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElSqlServerEl {
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildQuicksightDataSourceParametersElSqlServerEl {
    pub fn build(self) -> QuicksightDataSourceParametersElSqlServerEl {
        QuicksightDataSourceParametersElSqlServerEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}
pub struct QuicksightDataSourceParametersElSqlServerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElSqlServerElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElSqlServerElRef {
        QuicksightDataSourceParametersElSqlServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElSqlServerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElTeradataEl {
    database: PrimField<String>,
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl QuicksightDataSourceParametersElTeradataEl {}
impl ToListMappable for QuicksightDataSourceParametersElTeradataEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElTeradataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElTeradataEl {
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildQuicksightDataSourceParametersElTeradataEl {
    pub fn build(self) -> QuicksightDataSourceParametersElTeradataEl {
        QuicksightDataSourceParametersElTeradataEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}
pub struct QuicksightDataSourceParametersElTeradataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElTeradataElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElTeradataElRef {
        QuicksightDataSourceParametersElTeradataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElTeradataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElTwitterEl {
    max_rows: PrimField<f64>,
    query: PrimField<String>,
}
impl QuicksightDataSourceParametersElTwitterEl {}
impl ToListMappable for QuicksightDataSourceParametersElTwitterEl {
    type O = BlockAssignable<QuicksightDataSourceParametersElTwitterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersElTwitterEl {
    #[doc = ""]
    pub max_rows: PrimField<f64>,
    #[doc = ""]
    pub query: PrimField<String>,
}
impl BuildQuicksightDataSourceParametersElTwitterEl {
    pub fn build(self) -> QuicksightDataSourceParametersElTwitterEl {
        QuicksightDataSourceParametersElTwitterEl {
            max_rows: self.max_rows,
            query: self.query,
        }
    }
}
pub struct QuicksightDataSourceParametersElTwitterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElTwitterElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElTwitterElRef {
        QuicksightDataSourceParametersElTwitterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElTwitterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_rows` after provisioning.\n"]
    pub fn max_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_rows", self.base))
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSourceParametersElDynamic {
    amazon_elasticsearch:
        Option<DynamicBlock<QuicksightDataSourceParametersElAmazonElasticsearchEl>>,
    athena: Option<DynamicBlock<QuicksightDataSourceParametersElAthenaEl>>,
    aurora: Option<DynamicBlock<QuicksightDataSourceParametersElAuroraEl>>,
    aurora_postgresql: Option<DynamicBlock<QuicksightDataSourceParametersElAuroraPostgresqlEl>>,
    aws_iot_analytics: Option<DynamicBlock<QuicksightDataSourceParametersElAwsIotAnalyticsEl>>,
    databricks: Option<DynamicBlock<QuicksightDataSourceParametersElDatabricksEl>>,
    jira: Option<DynamicBlock<QuicksightDataSourceParametersElJiraEl>>,
    maria_db: Option<DynamicBlock<QuicksightDataSourceParametersElMariaDbEl>>,
    mysql: Option<DynamicBlock<QuicksightDataSourceParametersElMysqlEl>>,
    oracle: Option<DynamicBlock<QuicksightDataSourceParametersElOracleEl>>,
    postgresql: Option<DynamicBlock<QuicksightDataSourceParametersElPostgresqlEl>>,
    presto: Option<DynamicBlock<QuicksightDataSourceParametersElPrestoEl>>,
    rds: Option<DynamicBlock<QuicksightDataSourceParametersElRdsEl>>,
    redshift: Option<DynamicBlock<QuicksightDataSourceParametersElRedshiftEl>>,
    s3: Option<DynamicBlock<QuicksightDataSourceParametersElS3El>>,
    service_now: Option<DynamicBlock<QuicksightDataSourceParametersElServiceNowEl>>,
    snowflake: Option<DynamicBlock<QuicksightDataSourceParametersElSnowflakeEl>>,
    spark: Option<DynamicBlock<QuicksightDataSourceParametersElSparkEl>>,
    sql_server: Option<DynamicBlock<QuicksightDataSourceParametersElSqlServerEl>>,
    teradata: Option<DynamicBlock<QuicksightDataSourceParametersElTeradataEl>>,
    twitter: Option<DynamicBlock<QuicksightDataSourceParametersElTwitterEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSourceParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_elasticsearch: Option<Vec<QuicksightDataSourceParametersElAmazonElasticsearchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    athena: Option<Vec<QuicksightDataSourceParametersElAthenaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aurora: Option<Vec<QuicksightDataSourceParametersElAuroraEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aurora_postgresql: Option<Vec<QuicksightDataSourceParametersElAuroraPostgresqlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_iot_analytics: Option<Vec<QuicksightDataSourceParametersElAwsIotAnalyticsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    databricks: Option<Vec<QuicksightDataSourceParametersElDatabricksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jira: Option<Vec<QuicksightDataSourceParametersElJiraEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maria_db: Option<Vec<QuicksightDataSourceParametersElMariaDbEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql: Option<Vec<QuicksightDataSourceParametersElMysqlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle: Option<Vec<QuicksightDataSourceParametersElOracleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postgresql: Option<Vec<QuicksightDataSourceParametersElPostgresqlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presto: Option<Vec<QuicksightDataSourceParametersElPrestoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rds: Option<Vec<QuicksightDataSourceParametersElRdsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redshift: Option<Vec<QuicksightDataSourceParametersElRedshiftEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<QuicksightDataSourceParametersElS3El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_now: Option<Vec<QuicksightDataSourceParametersElServiceNowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snowflake: Option<Vec<QuicksightDataSourceParametersElSnowflakeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark: Option<Vec<QuicksightDataSourceParametersElSparkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_server: Option<Vec<QuicksightDataSourceParametersElSqlServerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    teradata: Option<Vec<QuicksightDataSourceParametersElTeradataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    twitter: Option<Vec<QuicksightDataSourceParametersElTwitterEl>>,
    dynamic: QuicksightDataSourceParametersElDynamic,
}
impl QuicksightDataSourceParametersEl {
    #[doc = "Set the field `amazon_elasticsearch`.\n"]
    pub fn set_amazon_elasticsearch(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElAmazonElasticsearchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amazon_elasticsearch = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_elasticsearch = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `athena`.\n"]
    pub fn set_athena(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElAthenaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.athena = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.athena = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `aurora`.\n"]
    pub fn set_aurora(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElAuroraEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aurora = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aurora = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `aurora_postgresql`.\n"]
    pub fn set_aurora_postgresql(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElAuroraPostgresqlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aurora_postgresql = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aurora_postgresql = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `aws_iot_analytics`.\n"]
    pub fn set_aws_iot_analytics(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElAwsIotAnalyticsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_iot_analytics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_iot_analytics = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `databricks`.\n"]
    pub fn set_databricks(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElDatabricksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.databricks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.databricks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `jira`.\n"]
    pub fn set_jira(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElJiraEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jira = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jira = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `maria_db`.\n"]
    pub fn set_maria_db(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElMariaDbEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.maria_db = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.maria_db = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mysql`.\n"]
    pub fn set_mysql(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElMysqlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mysql = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oracle`.\n"]
    pub fn set_oracle(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElOracleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oracle = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `postgresql`.\n"]
    pub fn set_postgresql(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElPostgresqlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postgresql = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `presto`.\n"]
    pub fn set_presto(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElPrestoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.presto = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.presto = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rds`.\n"]
    pub fn set_rds(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElRdsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rds = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redshift`.\n"]
    pub fn set_redshift(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElRedshiftEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redshift = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElServiceNowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_now = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_now = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `snowflake`.\n"]
    pub fn set_snowflake(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElSnowflakeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.snowflake = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark`.\n"]
    pub fn set_spark(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElSparkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spark = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spark = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sql_server`.\n"]
    pub fn set_sql_server(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElSqlServerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sql_server = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_server = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `teradata`.\n"]
    pub fn set_teradata(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElTeradataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.teradata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.teradata = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `twitter`.\n"]
    pub fn set_twitter(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSourceParametersElTwitterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.twitter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.twitter = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSourceParametersEl {
    type O = BlockAssignable<QuicksightDataSourceParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceParametersEl {}
impl BuildQuicksightDataSourceParametersEl {
    pub fn build(self) -> QuicksightDataSourceParametersEl {
        QuicksightDataSourceParametersEl {
            amazon_elasticsearch: core::default::Default::default(),
            athena: core::default::Default::default(),
            aurora: core::default::Default::default(),
            aurora_postgresql: core::default::Default::default(),
            aws_iot_analytics: core::default::Default::default(),
            databricks: core::default::Default::default(),
            jira: core::default::Default::default(),
            maria_db: core::default::Default::default(),
            mysql: core::default::Default::default(),
            oracle: core::default::Default::default(),
            postgresql: core::default::Default::default(),
            presto: core::default::Default::default(),
            rds: core::default::Default::default(),
            redshift: core::default::Default::default(),
            s3: core::default::Default::default(),
            service_now: core::default::Default::default(),
            snowflake: core::default::Default::default(),
            spark: core::default::Default::default(),
            sql_server: core::default::Default::default(),
            teradata: core::default::Default::default(),
            twitter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSourceParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceParametersElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceParametersElRef {
        QuicksightDataSourceParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `amazon_elasticsearch` after provisioning.\n"]
    pub fn amazon_elasticsearch(
        &self,
    ) -> ListRef<QuicksightDataSourceParametersElAmazonElasticsearchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_elasticsearch", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `athena` after provisioning.\n"]
    pub fn athena(&self) -> ListRef<QuicksightDataSourceParametersElAthenaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.athena", self.base))
    }
    #[doc = "Get a reference to the value of field `aurora` after provisioning.\n"]
    pub fn aurora(&self) -> ListRef<QuicksightDataSourceParametersElAuroraElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aurora", self.base))
    }
    #[doc = "Get a reference to the value of field `aurora_postgresql` after provisioning.\n"]
    pub fn aurora_postgresql(
        &self,
    ) -> ListRef<QuicksightDataSourceParametersElAuroraPostgresqlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aurora_postgresql", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `aws_iot_analytics` after provisioning.\n"]
    pub fn aws_iot_analytics(
        &self,
    ) -> ListRef<QuicksightDataSourceParametersElAwsIotAnalyticsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_iot_analytics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `databricks` after provisioning.\n"]
    pub fn databricks(&self) -> ListRef<QuicksightDataSourceParametersElDatabricksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.databricks", self.base))
    }
    #[doc = "Get a reference to the value of field `jira` after provisioning.\n"]
    pub fn jira(&self) -> ListRef<QuicksightDataSourceParametersElJiraElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jira", self.base))
    }
    #[doc = "Get a reference to the value of field `maria_db` after provisioning.\n"]
    pub fn maria_db(&self) -> ListRef<QuicksightDataSourceParametersElMariaDbElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maria_db", self.base))
    }
    #[doc = "Get a reference to the value of field `mysql` after provisioning.\n"]
    pub fn mysql(&self) -> ListRef<QuicksightDataSourceParametersElMysqlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mysql", self.base))
    }
    #[doc = "Get a reference to the value of field `oracle` after provisioning.\n"]
    pub fn oracle(&self) -> ListRef<QuicksightDataSourceParametersElOracleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oracle", self.base))
    }
    #[doc = "Get a reference to the value of field `postgresql` after provisioning.\n"]
    pub fn postgresql(&self) -> ListRef<QuicksightDataSourceParametersElPostgresqlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.postgresql", self.base))
    }
    #[doc = "Get a reference to the value of field `presto` after provisioning.\n"]
    pub fn presto(&self) -> ListRef<QuicksightDataSourceParametersElPrestoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.presto", self.base))
    }
    #[doc = "Get a reference to the value of field `rds` after provisioning.\n"]
    pub fn rds(&self) -> ListRef<QuicksightDataSourceParametersElRdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rds", self.base))
    }
    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(&self) -> ListRef<QuicksightDataSourceParametersElRedshiftElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }
    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<QuicksightDataSourceParametersElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(&self) -> ListRef<QuicksightDataSourceParametersElServiceNowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }
    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(&self) -> ListRef<QuicksightDataSourceParametersElSnowflakeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }
    #[doc = "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> ListRef<QuicksightDataSourceParametersElSparkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spark", self.base))
    }
    #[doc = "Get a reference to the value of field `sql_server` after provisioning.\n"]
    pub fn sql_server(&self) -> ListRef<QuicksightDataSourceParametersElSqlServerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sql_server", self.base))
    }
    #[doc = "Get a reference to the value of field `teradata` after provisioning.\n"]
    pub fn teradata(&self) -> ListRef<QuicksightDataSourceParametersElTeradataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.teradata", self.base))
    }
    #[doc = "Get a reference to the value of field `twitter` after provisioning.\n"]
    pub fn twitter(&self) -> ListRef<QuicksightDataSourceParametersElTwitterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.twitter", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourcePermissionEl {
    actions: SetField<PrimField<String>>,
    principal: PrimField<String>,
}
impl QuicksightDataSourcePermissionEl {}
impl ToListMappable for QuicksightDataSourcePermissionEl {
    type O = BlockAssignable<QuicksightDataSourcePermissionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourcePermissionEl {
    #[doc = ""]
    pub actions: SetField<PrimField<String>>,
    #[doc = ""]
    pub principal: PrimField<String>,
}
impl BuildQuicksightDataSourcePermissionEl {
    pub fn build(self) -> QuicksightDataSourcePermissionEl {
        QuicksightDataSourcePermissionEl {
            actions: self.actions,
            principal: self.principal,
        }
    }
}
pub struct QuicksightDataSourcePermissionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourcePermissionElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourcePermissionElRef {
        QuicksightDataSourcePermissionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourcePermissionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceSslPropertiesEl {
    disable_ssl: PrimField<bool>,
}
impl QuicksightDataSourceSslPropertiesEl {}
impl ToListMappable for QuicksightDataSourceSslPropertiesEl {
    type O = BlockAssignable<QuicksightDataSourceSslPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceSslPropertiesEl {
    #[doc = ""]
    pub disable_ssl: PrimField<bool>,
}
impl BuildQuicksightDataSourceSslPropertiesEl {
    pub fn build(self) -> QuicksightDataSourceSslPropertiesEl {
        QuicksightDataSourceSslPropertiesEl {
            disable_ssl: self.disable_ssl,
        }
    }
}
pub struct QuicksightDataSourceSslPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceSslPropertiesElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceSslPropertiesElRef {
        QuicksightDataSourceSslPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceSslPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_ssl` after provisioning.\n"]
    pub fn disable_ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_ssl", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSourceVpcConnectionPropertiesEl {
    vpc_connection_arn: PrimField<String>,
}
impl QuicksightDataSourceVpcConnectionPropertiesEl {}
impl ToListMappable for QuicksightDataSourceVpcConnectionPropertiesEl {
    type O = BlockAssignable<QuicksightDataSourceVpcConnectionPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSourceVpcConnectionPropertiesEl {
    #[doc = ""]
    pub vpc_connection_arn: PrimField<String>,
}
impl BuildQuicksightDataSourceVpcConnectionPropertiesEl {
    pub fn build(self) -> QuicksightDataSourceVpcConnectionPropertiesEl {
        QuicksightDataSourceVpcConnectionPropertiesEl {
            vpc_connection_arn: self.vpc_connection_arn,
        }
    }
}
pub struct QuicksightDataSourceVpcConnectionPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSourceVpcConnectionPropertiesElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSourceVpcConnectionPropertiesElRef {
        QuicksightDataSourceVpcConnectionPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSourceVpcConnectionPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `vpc_connection_arn` after provisioning.\n"]
    pub fn vpc_connection_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connection_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSourceDynamic {
    credentials: Option<DynamicBlock<QuicksightDataSourceCredentialsEl>>,
    parameters: Option<DynamicBlock<QuicksightDataSourceParametersEl>>,
    permission: Option<DynamicBlock<QuicksightDataSourcePermissionEl>>,
    ssl_properties: Option<DynamicBlock<QuicksightDataSourceSslPropertiesEl>>,
    vpc_connection_properties: Option<DynamicBlock<QuicksightDataSourceVpcConnectionPropertiesEl>>,
}
