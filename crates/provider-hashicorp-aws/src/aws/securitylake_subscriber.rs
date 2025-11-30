use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SecuritylakeSubscriberData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriber_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriber_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<SecuritylakeSubscriberSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriber_identity: Option<Vec<SecuritylakeSubscriberSubscriberIdentityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecuritylakeSubscriberTimeoutsEl>,
    dynamic: SecuritylakeSubscriberDynamic,
}

struct SecuritylakeSubscriber_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecuritylakeSubscriberData>,
}

#[derive(Clone)]
pub struct SecuritylakeSubscriber(Rc<SecuritylakeSubscriber_>);

impl SecuritylakeSubscriber {
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

    #[doc = "Set the field `access_type`.\n"]
    pub fn set_access_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_type = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `subscriber_description`.\n"]
    pub fn set_subscriber_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subscriber_description = Some(v.into());
        self
    }

    #[doc = "Set the field `subscriber_name`.\n"]
    pub fn set_subscriber_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subscriber_name = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl Into<BlockAssignable<SecuritylakeSubscriberSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `subscriber_identity`.\n"]
    pub fn set_subscriber_identity(
        self,
        v: impl Into<BlockAssignable<SecuritylakeSubscriberSubscriberIdentityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subscriber_identity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subscriber_identity = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SecuritylakeSubscriberTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `access_type` after provisioning.\n"]
    pub fn access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_share_arn` after provisioning.\n"]
    pub fn resource_share_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_share_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_share_name` after provisioning.\n"]
    pub fn resource_share_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_share_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_bucket_arn` after provisioning.\n"]
    pub fn s3_bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscriber_description` after provisioning.\n"]
    pub fn subscriber_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscriber_endpoint` after provisioning.\n"]
    pub fn subscriber_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscriber_name` after provisioning.\n"]
    pub fn subscriber_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscriber_status` after provisioning.\n"]
    pub fn subscriber_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `subscriber_identity` after provisioning.\n"]
    pub fn subscriber_identity(&self) -> ListRef<SecuritylakeSubscriberSubscriberIdentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subscriber_identity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecuritylakeSubscriberTimeoutsElRef {
        SecuritylakeSubscriberTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for SecuritylakeSubscriber {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SecuritylakeSubscriber {}

impl ToListMappable for SecuritylakeSubscriber {
    type O = ListRef<SecuritylakeSubscriberRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SecuritylakeSubscriber_ {
    fn extract_resource_type(&self) -> String {
        "aws_securitylake_subscriber".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecuritylakeSubscriber {
    pub tf_id: String,
}

impl BuildSecuritylakeSubscriber {
    pub fn build(self, stack: &mut Stack) -> SecuritylakeSubscriber {
        let out = SecuritylakeSubscriber(Rc::new(SecuritylakeSubscriber_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecuritylakeSubscriberData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_type: core::default::Default::default(),
                region: core::default::Default::default(),
                subscriber_description: core::default::Default::default(),
                subscriber_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                source: core::default::Default::default(),
                subscriber_identity: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecuritylakeSubscriberRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeSubscriberRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SecuritylakeSubscriberRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_type` after provisioning.\n"]
    pub fn access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_share_arn` after provisioning.\n"]
    pub fn resource_share_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_share_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_share_name` after provisioning.\n"]
    pub fn resource_share_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_share_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_bucket_arn` after provisioning.\n"]
    pub fn s3_bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscriber_description` after provisioning.\n"]
    pub fn subscriber_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscriber_endpoint` after provisioning.\n"]
    pub fn subscriber_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscriber_name` after provisioning.\n"]
    pub fn subscriber_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscriber_status` after provisioning.\n"]
    pub fn subscriber_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `subscriber_identity` after provisioning.\n"]
    pub fn subscriber_identity(&self) -> ListRef<SecuritylakeSubscriberSubscriberIdentityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subscriber_identity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecuritylakeSubscriberTimeoutsElRef {
        SecuritylakeSubscriberTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriberSourceElAwsLogSourceResourceEl {
    source_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_version: Option<PrimField<String>>,
}

impl SecuritylakeSubscriberSourceElAwsLogSourceResourceEl {
    #[doc = "Set the field `source_version`.\n"]
    pub fn set_source_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_version = Some(v.into());
        self
    }
}

impl ToListMappable for SecuritylakeSubscriberSourceElAwsLogSourceResourceEl {
    type O = BlockAssignable<SecuritylakeSubscriberSourceElAwsLogSourceResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriberSourceElAwsLogSourceResourceEl {
    #[doc = ""]
    pub source_name: PrimField<String>,
}

impl BuildSecuritylakeSubscriberSourceElAwsLogSourceResourceEl {
    pub fn build(self) -> SecuritylakeSubscriberSourceElAwsLogSourceResourceEl {
        SecuritylakeSubscriberSourceElAwsLogSourceResourceEl {
            source_name: self.source_name,
            source_version: core::default::Default::default(),
        }
    }
}

pub struct SecuritylakeSubscriberSourceElAwsLogSourceResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeSubscriberSourceElAwsLogSourceResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecuritylakeSubscriberSourceElAwsLogSourceResourceElRef {
        SecuritylakeSubscriberSourceElAwsLogSourceResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriberSourceElAwsLogSourceResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_version", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crawler_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_arn: Option<PrimField<String>>,
}

impl SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesEl {
    #[doc = "Set the field `crawler_arn`.\n"]
    pub fn set_crawler_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.crawler_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `database_arn`.\n"]
    pub fn set_database_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `table_arn`.\n"]
    pub fn set_table_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesEl {
    type O = BlockAssignable<SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesEl {}

impl BuildSecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesEl {
    pub fn build(self) -> SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesEl {
        SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesEl {
            crawler_arn: core::default::Default::default(),
            database_arn: core::default::Default::default(),
            table_arn: core::default::Default::default(),
        }
    }
}

pub struct SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesElRef {
        SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_arn` after provisioning.\n"]
    pub fn crawler_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.crawler_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_arn` after provisioning.\n"]
    pub fn database_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
}

impl SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderEl {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderEl {
    type O = BlockAssignable<SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderEl {}

impl BuildSecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderEl {
    pub fn build(self) -> SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderEl {
        SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderEl {
            location: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}

pub struct SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderElRef {
        SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriberSourceElCustomLogSourceResourceEl {
    source_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_version: Option<PrimField<String>>,
}

impl SecuritylakeSubscriberSourceElCustomLogSourceResourceEl {
    #[doc = "Set the field `source_version`.\n"]
    pub fn set_source_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_version = Some(v.into());
        self
    }
}

impl ToListMappable for SecuritylakeSubscriberSourceElCustomLogSourceResourceEl {
    type O = BlockAssignable<SecuritylakeSubscriberSourceElCustomLogSourceResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriberSourceElCustomLogSourceResourceEl {
    #[doc = ""]
    pub source_name: PrimField<String>,
}

impl BuildSecuritylakeSubscriberSourceElCustomLogSourceResourceEl {
    pub fn build(self) -> SecuritylakeSubscriberSourceElCustomLogSourceResourceEl {
        SecuritylakeSubscriberSourceElCustomLogSourceResourceEl {
            source_name: self.source_name,
            source_version: core::default::Default::default(),
        }
    }
}

pub struct SecuritylakeSubscriberSourceElCustomLogSourceResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeSubscriberSourceElCustomLogSourceResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecuritylakeSubscriberSourceElCustomLogSourceResourceElRef {
        SecuritylakeSubscriberSourceElCustomLogSourceResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriberSourceElCustomLogSourceResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(
        &self,
    ) -> ListRef<SecuritylakeSubscriberSourceElCustomLogSourceResourceElAttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(
        &self,
    ) -> ListRef<SecuritylakeSubscriberSourceElCustomLogSourceResourceElProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_version", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeSubscriberSourceElDynamic {
    aws_log_source_resource:
        Option<DynamicBlock<SecuritylakeSubscriberSourceElAwsLogSourceResourceEl>>,
    custom_log_source_resource:
        Option<DynamicBlock<SecuritylakeSubscriberSourceElCustomLogSourceResourceEl>>,
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriberSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_log_source_resource: Option<Vec<SecuritylakeSubscriberSourceElAwsLogSourceResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_log_source_resource:
        Option<Vec<SecuritylakeSubscriberSourceElCustomLogSourceResourceEl>>,
    dynamic: SecuritylakeSubscriberSourceElDynamic,
}

impl SecuritylakeSubscriberSourceEl {
    #[doc = "Set the field `aws_log_source_resource`.\n"]
    pub fn set_aws_log_source_resource(
        mut self,
        v: impl Into<BlockAssignable<SecuritylakeSubscriberSourceElAwsLogSourceResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_log_source_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_log_source_resource = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `custom_log_source_resource`.\n"]
    pub fn set_custom_log_source_resource(
        mut self,
        v: impl Into<BlockAssignable<SecuritylakeSubscriberSourceElCustomLogSourceResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_log_source_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_log_source_resource = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SecuritylakeSubscriberSourceEl {
    type O = BlockAssignable<SecuritylakeSubscriberSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriberSourceEl {}

impl BuildSecuritylakeSubscriberSourceEl {
    pub fn build(self) -> SecuritylakeSubscriberSourceEl {
        SecuritylakeSubscriberSourceEl {
            aws_log_source_resource: core::default::Default::default(),
            custom_log_source_resource: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SecuritylakeSubscriberSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeSubscriberSourceElRef {
    fn new(shared: StackShared, base: String) -> SecuritylakeSubscriberSourceElRef {
        SecuritylakeSubscriberSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriberSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_log_source_resource` after provisioning.\n"]
    pub fn aws_log_source_resource(
        &self,
    ) -> ListRef<SecuritylakeSubscriberSourceElAwsLogSourceResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_log_source_resource", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_log_source_resource` after provisioning.\n"]
    pub fn custom_log_source_resource(
        &self,
    ) -> ListRef<SecuritylakeSubscriberSourceElCustomLogSourceResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_log_source_resource", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriberSubscriberIdentityEl {
    external_id: PrimField<String>,
    principal: PrimField<String>,
}

impl SecuritylakeSubscriberSubscriberIdentityEl {}

impl ToListMappable for SecuritylakeSubscriberSubscriberIdentityEl {
    type O = BlockAssignable<SecuritylakeSubscriberSubscriberIdentityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriberSubscriberIdentityEl {
    #[doc = ""]
    pub external_id: PrimField<String>,
    #[doc = ""]
    pub principal: PrimField<String>,
}

impl BuildSecuritylakeSubscriberSubscriberIdentityEl {
    pub fn build(self) -> SecuritylakeSubscriberSubscriberIdentityEl {
        SecuritylakeSubscriberSubscriberIdentityEl {
            external_id: self.external_id,
            principal: self.principal,
        }
    }
}

pub struct SecuritylakeSubscriberSubscriberIdentityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeSubscriberSubscriberIdentityElRef {
    fn new(shared: StackShared, base: String) -> SecuritylakeSubscriberSubscriberIdentityElRef {
        SecuritylakeSubscriberSubscriberIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriberSubscriberIdentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriberTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl SecuritylakeSubscriberTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for SecuritylakeSubscriberTimeoutsEl {
    type O = BlockAssignable<SecuritylakeSubscriberTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriberTimeoutsEl {}

impl BuildSecuritylakeSubscriberTimeoutsEl {
    pub fn build(self) -> SecuritylakeSubscriberTimeoutsEl {
        SecuritylakeSubscriberTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct SecuritylakeSubscriberTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecuritylakeSubscriberTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecuritylakeSubscriberTimeoutsElRef {
        SecuritylakeSubscriberTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriberTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeSubscriberDynamic {
    source: Option<DynamicBlock<SecuritylakeSubscriberSourceEl>>,
    subscriber_identity: Option<DynamicBlock<SecuritylakeSubscriberSubscriberIdentityEl>>,
}
