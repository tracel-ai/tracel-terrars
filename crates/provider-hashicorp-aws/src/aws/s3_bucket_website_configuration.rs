use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct S3BucketWebsiteConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_bucket_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_rules: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_document: Option<Vec<S3BucketWebsiteConfigurationErrorDocumentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_document: Option<Vec<S3BucketWebsiteConfigurationIndexDocumentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_all_requests_to: Option<Vec<S3BucketWebsiteConfigurationRedirectAllRequestsToEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_rule: Option<Vec<S3BucketWebsiteConfigurationRoutingRuleEl>>,
    dynamic: S3BucketWebsiteConfigurationDynamic,
}
struct S3BucketWebsiteConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketWebsiteConfigurationData>,
}
#[derive(Clone)]
pub struct S3BucketWebsiteConfiguration(Rc<S3BucketWebsiteConfiguration_>);
impl S3BucketWebsiteConfiguration {
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
    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = Some(v.into());
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
    #[doc = "Set the field `routing_rules`.\n"]
    pub fn set_routing_rules(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().routing_rules = Some(v.into());
        self
    }
    #[doc = "Set the field `error_document`.\n"]
    pub fn set_error_document(
        self,
        v: impl Into<BlockAssignable<S3BucketWebsiteConfigurationErrorDocumentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().error_document = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.error_document = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `index_document`.\n"]
    pub fn set_index_document(
        self,
        v: impl Into<BlockAssignable<S3BucketWebsiteConfigurationIndexDocumentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().index_document = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.index_document = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redirect_all_requests_to`.\n"]
    pub fn set_redirect_all_requests_to(
        self,
        v: impl Into<BlockAssignable<S3BucketWebsiteConfigurationRedirectAllRequestsToEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redirect_all_requests_to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redirect_all_requests_to = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `routing_rule`.\n"]
    pub fn set_routing_rule(
        self,
        v: impl Into<BlockAssignable<S3BucketWebsiteConfigurationRoutingRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routing_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routing_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expected_bucket_owner", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `routing_rules` after provisioning.\n"]
    pub fn routing_rules(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website_domain` after provisioning.\n"]
    pub fn website_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.website_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website_endpoint` after provisioning.\n"]
    pub fn website_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.website_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error_document` after provisioning.\n"]
    pub fn error_document(&self) -> ListRef<S3BucketWebsiteConfigurationErrorDocumentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error_document", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `index_document` after provisioning.\n"]
    pub fn index_document(&self) -> ListRef<S3BucketWebsiteConfigurationIndexDocumentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.index_document", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_all_requests_to` after provisioning.\n"]
    pub fn redirect_all_requests_to(
        &self,
    ) -> ListRef<S3BucketWebsiteConfigurationRedirectAllRequestsToElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_all_requests_to", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routing_rule` after provisioning.\n"]
    pub fn routing_rule(&self) -> ListRef<S3BucketWebsiteConfigurationRoutingRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routing_rule", self.extract_ref()),
        )
    }
}
impl Referable for S3BucketWebsiteConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for S3BucketWebsiteConfiguration {}
impl ToListMappable for S3BucketWebsiteConfiguration {
    type O = ListRef<S3BucketWebsiteConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for S3BucketWebsiteConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_website_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildS3BucketWebsiteConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
}
impl BuildS3BucketWebsiteConfiguration {
    pub fn build(self, stack: &mut Stack) -> S3BucketWebsiteConfiguration {
        let out = S3BucketWebsiteConfiguration(Rc::new(S3BucketWebsiteConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketWebsiteConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                routing_rules: core::default::Default::default(),
                error_document: core::default::Default::default(),
                index_document: core::default::Default::default(),
                redirect_all_requests_to: core::default::Default::default(),
                routing_rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct S3BucketWebsiteConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketWebsiteConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl S3BucketWebsiteConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expected_bucket_owner", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `routing_rules` after provisioning.\n"]
    pub fn routing_rules(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website_domain` after provisioning.\n"]
    pub fn website_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.website_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `website_endpoint` after provisioning.\n"]
    pub fn website_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.website_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error_document` after provisioning.\n"]
    pub fn error_document(&self) -> ListRef<S3BucketWebsiteConfigurationErrorDocumentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error_document", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `index_document` after provisioning.\n"]
    pub fn index_document(&self) -> ListRef<S3BucketWebsiteConfigurationIndexDocumentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.index_document", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_all_requests_to` after provisioning.\n"]
    pub fn redirect_all_requests_to(
        &self,
    ) -> ListRef<S3BucketWebsiteConfigurationRedirectAllRequestsToElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_all_requests_to", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routing_rule` after provisioning.\n"]
    pub fn routing_rule(&self) -> ListRef<S3BucketWebsiteConfigurationRoutingRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routing_rule", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketWebsiteConfigurationErrorDocumentEl {
    key: PrimField<String>,
}
impl S3BucketWebsiteConfigurationErrorDocumentEl {}
impl ToListMappable for S3BucketWebsiteConfigurationErrorDocumentEl {
    type O = BlockAssignable<S3BucketWebsiteConfigurationErrorDocumentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketWebsiteConfigurationErrorDocumentEl {
    #[doc = ""]
    pub key: PrimField<String>,
}
impl BuildS3BucketWebsiteConfigurationErrorDocumentEl {
    pub fn build(self) -> S3BucketWebsiteConfigurationErrorDocumentEl {
        S3BucketWebsiteConfigurationErrorDocumentEl { key: self.key }
    }
}
pub struct S3BucketWebsiteConfigurationErrorDocumentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketWebsiteConfigurationErrorDocumentElRef {
    fn new(shared: StackShared, base: String) -> S3BucketWebsiteConfigurationErrorDocumentElRef {
        S3BucketWebsiteConfigurationErrorDocumentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketWebsiteConfigurationErrorDocumentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}
#[derive(Serialize)]
pub struct S3BucketWebsiteConfigurationIndexDocumentEl {
    suffix: PrimField<String>,
}
impl S3BucketWebsiteConfigurationIndexDocumentEl {}
impl ToListMappable for S3BucketWebsiteConfigurationIndexDocumentEl {
    type O = BlockAssignable<S3BucketWebsiteConfigurationIndexDocumentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketWebsiteConfigurationIndexDocumentEl {
    #[doc = ""]
    pub suffix: PrimField<String>,
}
impl BuildS3BucketWebsiteConfigurationIndexDocumentEl {
    pub fn build(self) -> S3BucketWebsiteConfigurationIndexDocumentEl {
        S3BucketWebsiteConfigurationIndexDocumentEl {
            suffix: self.suffix,
        }
    }
}
pub struct S3BucketWebsiteConfigurationIndexDocumentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketWebsiteConfigurationIndexDocumentElRef {
    fn new(shared: StackShared, base: String) -> S3BucketWebsiteConfigurationIndexDocumentElRef {
        S3BucketWebsiteConfigurationIndexDocumentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketWebsiteConfigurationIndexDocumentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize)]
pub struct S3BucketWebsiteConfigurationRedirectAllRequestsToEl {
    host_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
}
impl S3BucketWebsiteConfigurationRedirectAllRequestsToEl {
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
}
impl ToListMappable for S3BucketWebsiteConfigurationRedirectAllRequestsToEl {
    type O = BlockAssignable<S3BucketWebsiteConfigurationRedirectAllRequestsToEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketWebsiteConfigurationRedirectAllRequestsToEl {
    #[doc = ""]
    pub host_name: PrimField<String>,
}
impl BuildS3BucketWebsiteConfigurationRedirectAllRequestsToEl {
    pub fn build(self) -> S3BucketWebsiteConfigurationRedirectAllRequestsToEl {
        S3BucketWebsiteConfigurationRedirectAllRequestsToEl {
            host_name: self.host_name,
            protocol: core::default::Default::default(),
        }
    }
}
pub struct S3BucketWebsiteConfigurationRedirectAllRequestsToElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketWebsiteConfigurationRedirectAllRequestsToElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketWebsiteConfigurationRedirectAllRequestsToElRef {
        S3BucketWebsiteConfigurationRedirectAllRequestsToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketWebsiteConfigurationRedirectAllRequestsToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_name` after provisioning.\n"]
    pub fn host_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_name", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}
#[derive(Serialize)]
pub struct S3BucketWebsiteConfigurationRoutingRuleElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_error_code_returned_equals: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_prefix_equals: Option<PrimField<String>>,
}
impl S3BucketWebsiteConfigurationRoutingRuleElConditionEl {
    #[doc = "Set the field `http_error_code_returned_equals`.\n"]
    pub fn set_http_error_code_returned_equals(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_error_code_returned_equals = Some(v.into());
        self
    }
    #[doc = "Set the field `key_prefix_equals`.\n"]
    pub fn set_key_prefix_equals(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_prefix_equals = Some(v.into());
        self
    }
}
impl ToListMappable for S3BucketWebsiteConfigurationRoutingRuleElConditionEl {
    type O = BlockAssignable<S3BucketWebsiteConfigurationRoutingRuleElConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketWebsiteConfigurationRoutingRuleElConditionEl {}
impl BuildS3BucketWebsiteConfigurationRoutingRuleElConditionEl {
    pub fn build(self) -> S3BucketWebsiteConfigurationRoutingRuleElConditionEl {
        S3BucketWebsiteConfigurationRoutingRuleElConditionEl {
            http_error_code_returned_equals: core::default::Default::default(),
            key_prefix_equals: core::default::Default::default(),
        }
    }
}
pub struct S3BucketWebsiteConfigurationRoutingRuleElConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketWebsiteConfigurationRoutingRuleElConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketWebsiteConfigurationRoutingRuleElConditionElRef {
        S3BucketWebsiteConfigurationRoutingRuleElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketWebsiteConfigurationRoutingRuleElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_error_code_returned_equals` after provisioning.\n"]
    pub fn http_error_code_returned_equals(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_error_code_returned_equals", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_prefix_equals` after provisioning.\n"]
    pub fn key_prefix_equals(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_prefix_equals", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketWebsiteConfigurationRoutingRuleElRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_redirect_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace_key_prefix_with: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace_key_with: Option<PrimField<String>>,
}
impl S3BucketWebsiteConfigurationRoutingRuleElRedirectEl {
    #[doc = "Set the field `host_name`.\n"]
    pub fn set_host_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_name = Some(v.into());
        self
    }
    #[doc = "Set the field `http_redirect_code`.\n"]
    pub fn set_http_redirect_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_redirect_code = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `replace_key_prefix_with`.\n"]
    pub fn set_replace_key_prefix_with(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replace_key_prefix_with = Some(v.into());
        self
    }
    #[doc = "Set the field `replace_key_with`.\n"]
    pub fn set_replace_key_with(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replace_key_with = Some(v.into());
        self
    }
}
impl ToListMappable for S3BucketWebsiteConfigurationRoutingRuleElRedirectEl {
    type O = BlockAssignable<S3BucketWebsiteConfigurationRoutingRuleElRedirectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketWebsiteConfigurationRoutingRuleElRedirectEl {}
impl BuildS3BucketWebsiteConfigurationRoutingRuleElRedirectEl {
    pub fn build(self) -> S3BucketWebsiteConfigurationRoutingRuleElRedirectEl {
        S3BucketWebsiteConfigurationRoutingRuleElRedirectEl {
            host_name: core::default::Default::default(),
            http_redirect_code: core::default::Default::default(),
            protocol: core::default::Default::default(),
            replace_key_prefix_with: core::default::Default::default(),
            replace_key_with: core::default::Default::default(),
        }
    }
}
pub struct S3BucketWebsiteConfigurationRoutingRuleElRedirectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketWebsiteConfigurationRoutingRuleElRedirectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketWebsiteConfigurationRoutingRuleElRedirectElRef {
        S3BucketWebsiteConfigurationRoutingRuleElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketWebsiteConfigurationRoutingRuleElRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_name` after provisioning.\n"]
    pub fn host_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_name", self.base))
    }
    #[doc = "Get a reference to the value of field `http_redirect_code` after provisioning.\n"]
    pub fn http_redirect_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_redirect_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `replace_key_prefix_with` after provisioning.\n"]
    pub fn replace_key_prefix_with(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replace_key_prefix_with", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `replace_key_with` after provisioning.\n"]
    pub fn replace_key_with(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replace_key_with", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct S3BucketWebsiteConfigurationRoutingRuleElDynamic {
    condition: Option<DynamicBlock<S3BucketWebsiteConfigurationRoutingRuleElConditionEl>>,
    redirect: Option<DynamicBlock<S3BucketWebsiteConfigurationRoutingRuleElRedirectEl>>,
}
#[derive(Serialize)]
pub struct S3BucketWebsiteConfigurationRoutingRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<S3BucketWebsiteConfigurationRoutingRuleElConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<Vec<S3BucketWebsiteConfigurationRoutingRuleElRedirectEl>>,
    dynamic: S3BucketWebsiteConfigurationRoutingRuleElDynamic,
}
impl S3BucketWebsiteConfigurationRoutingRuleEl {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<S3BucketWebsiteConfigurationRoutingRuleElConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl Into<BlockAssignable<S3BucketWebsiteConfigurationRoutingRuleElRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redirect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3BucketWebsiteConfigurationRoutingRuleEl {
    type O = BlockAssignable<S3BucketWebsiteConfigurationRoutingRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketWebsiteConfigurationRoutingRuleEl {}
impl BuildS3BucketWebsiteConfigurationRoutingRuleEl {
    pub fn build(self) -> S3BucketWebsiteConfigurationRoutingRuleEl {
        S3BucketWebsiteConfigurationRoutingRuleEl {
            condition: core::default::Default::default(),
            redirect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3BucketWebsiteConfigurationRoutingRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketWebsiteConfigurationRoutingRuleElRef {
    fn new(shared: StackShared, base: String) -> S3BucketWebsiteConfigurationRoutingRuleElRef {
        S3BucketWebsiteConfigurationRoutingRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketWebsiteConfigurationRoutingRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<S3BucketWebsiteConfigurationRoutingRuleElConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<S3BucketWebsiteConfigurationRoutingRuleElRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3BucketWebsiteConfigurationDynamic {
    error_document: Option<DynamicBlock<S3BucketWebsiteConfigurationErrorDocumentEl>>,
    index_document: Option<DynamicBlock<S3BucketWebsiteConfigurationIndexDocumentEl>>,
    redirect_all_requests_to:
        Option<DynamicBlock<S3BucketWebsiteConfigurationRedirectAllRequestsToEl>>,
    routing_rule: Option<DynamicBlock<S3BucketWebsiteConfigurationRoutingRuleEl>>,
}
