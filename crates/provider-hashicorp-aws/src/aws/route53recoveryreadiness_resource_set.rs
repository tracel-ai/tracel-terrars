use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct Route53recoveryreadinessResourceSetData {
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
    resource_set_name: PrimField<String>,
    resource_set_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Vec<Route53recoveryreadinessResourceSetResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Route53recoveryreadinessResourceSetTimeoutsEl>,
    dynamic: Route53recoveryreadinessResourceSetDynamic,
}

struct Route53recoveryreadinessResourceSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Route53recoveryreadinessResourceSetData>,
}

#[derive(Clone)]
pub struct Route53recoveryreadinessResourceSet(Rc<Route53recoveryreadinessResourceSet_>);

impl Route53recoveryreadinessResourceSet {
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

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        self,
        v: impl Into<BlockAssignable<Route53recoveryreadinessResourceSetResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resources = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Route53recoveryreadinessResourceSetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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

    #[doc = "Get a reference to the value of field `resource_set_name` after provisioning.\n"]
    pub fn resource_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_set_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_set_type` after provisioning.\n"]
    pub fn resource_set_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_set_type", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<Route53recoveryreadinessResourceSetResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53recoveryreadinessResourceSetTimeoutsElRef {
        Route53recoveryreadinessResourceSetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for Route53recoveryreadinessResourceSet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Route53recoveryreadinessResourceSet {}

impl ToListMappable for Route53recoveryreadinessResourceSet {
    type O = ListRef<Route53recoveryreadinessResourceSetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Route53recoveryreadinessResourceSet_ {
    fn extract_resource_type(&self) -> String {
        "aws_route53recoveryreadiness_resource_set".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53recoveryreadinessResourceSet {
    pub tf_id: String,
    #[doc = ""]
    pub resource_set_name: PrimField<String>,
    #[doc = ""]
    pub resource_set_type: PrimField<String>,
}

impl BuildRoute53recoveryreadinessResourceSet {
    pub fn build(self, stack: &mut Stack) -> Route53recoveryreadinessResourceSet {
        let out =
            Route53recoveryreadinessResourceSet(Rc::new(Route53recoveryreadinessResourceSet_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(Route53recoveryreadinessResourceSetData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    id: core::default::Default::default(),
                    resource_set_name: self.resource_set_name,
                    resource_set_type: self.resource_set_type,
                    tags: core::default::Default::default(),
                    tags_all: core::default::Default::default(),
                    resources: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53recoveryreadinessResourceSetRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53recoveryreadinessResourceSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl Route53recoveryreadinessResourceSetRef {
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

    #[doc = "Get a reference to the value of field `resource_set_name` after provisioning.\n"]
    pub fn resource_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_set_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_set_type` after provisioning.\n"]
    pub fn resource_set_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_set_type", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<Route53recoveryreadinessResourceSetResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53recoveryreadinessResourceSetTimeoutsElRef {
        Route53recoveryreadinessResourceSetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
}

impl
    Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceEl
{
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
}

impl ToListMappable for Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceEl {
    type O =
        BlockAssignable<
            Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceEl
{}

impl BuildRoute53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceEl {
    pub fn build(
        self,
    ) -> Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceEl {
        Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceEl {
            arn: core::default::Default::default(),
        }
    }
}

pub struct Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceElRef {
        Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_set_id: Option<PrimField<String>>,
}

impl
    Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceEl
{
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }

    #[doc = "Set the field `record_set_id`.\n"]
    pub fn set_record_set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.record_set_id = Some(v.into());
        self
    }
}

impl ToListMappable for Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceEl {
    type O =
        BlockAssignable<
            Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceEl
{}

impl BuildRoute53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceEl {
    pub fn build(
        self,
    ) -> Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceEl {
        Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceEl {
            domain_name: core::default::Default::default(),
            record_set_id: core::default::Default::default(),
        }
    }
}

pub struct Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceElRef {
        Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `record_set_id` after provisioning.\n"]
    pub fn record_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_set_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElDynamic {
    nlb_resource: Option<
        DynamicBlock<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceEl>,
    >,
    r53_resource: Option<
        DynamicBlock<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceEl>,
    >,
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nlb_resource: Option<
        Vec<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    r53_resource: Option<
        Vec<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceEl>,
    >,
    dynamic: Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElDynamic,
}

impl Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceEl {
    #[doc = "Set the field `nlb_resource`.\n"]
    pub fn set_nlb_resource(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.nlb_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.nlb_resource = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `r53_resource`.\n"]
    pub fn set_r53_resource(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.r53_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.r53_resource = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceEl
{
    type O = BlockAssignable<
        Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceEl {
}

impl BuildRoute53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceEl {
    pub fn build(
        self,
    ) -> Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceEl {
        Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceEl {
            nlb_resource: core::default::Default::default(),
            r53_resource: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElRef {
        Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nlb_resource` after provisioning.\n"]
    pub fn nlb_resource(
        &self,
    ) -> ListRef<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElNlbResourceElRef>{
        ListRef::new(self.shared().clone(), format!("{}.nlb_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `r53_resource` after provisioning.\n"]
    pub fn r53_resource(
        &self,
    ) -> ListRef<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElR53ResourceElRef>{
        ListRef::new(self.shared().clone(), format!("{}.r53_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElDynamic {
    target_resource: Option<
        DynamicBlock<
            Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceEl {
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosted_zone_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_set_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_resource: Option<
        Vec<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceEl>,
    >,
    dynamic: Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElDynamic,
}

impl Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceEl {
    #[doc = "Set the field `hosted_zone_arn`.\n"]
    pub fn set_hosted_zone_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hosted_zone_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `record_set_id`.\n"]
    pub fn set_record_set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.record_set_id = Some(v.into());
        self
    }

    #[doc = "Set the field `record_type`.\n"]
    pub fn set_record_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.record_type = Some(v.into());
        self
    }

    #[doc = "Set the field `target_resource`.\n"]
    pub fn set_target_resource(
        mut self,
        v: impl Into<
            BlockAssignable<
                Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_resource = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceEl {
    type O = BlockAssignable<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSetResourcesElDnsTargetResourceEl {
    #[doc = ""]
    pub domain_name: PrimField<String>,
}

impl BuildRoute53recoveryreadinessResourceSetResourcesElDnsTargetResourceEl {
    pub fn build(self) -> Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceEl {
        Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceEl {
            domain_name: self.domain_name,
            hosted_zone_arn: core::default::Default::default(),
            record_set_id: core::default::Default::default(),
            record_type: core::default::Default::default(),
            target_resource: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElRef {
        Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_arn` after provisioning.\n"]
    pub fn hosted_zone_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `record_set_id` after provisioning.\n"]
    pub fn record_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_set_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `record_type` after provisioning.\n"]
    pub fn record_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.record_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_resource` after provisioning.\n"]
    pub fn target_resource(
        &self,
    ) -> ListRef<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElTargetResourceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_resource", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct Route53recoveryreadinessResourceSetResourcesElDynamic {
    dns_target_resource:
        Option<DynamicBlock<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceEl>>,
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSetResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    readiness_scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_target_resource:
        Option<Vec<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceEl>>,
    dynamic: Route53recoveryreadinessResourceSetResourcesElDynamic,
}

impl Route53recoveryreadinessResourceSetResourcesEl {
    #[doc = "Set the field `readiness_scopes`.\n"]
    pub fn set_readiness_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.readiness_scopes = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `dns_target_resource`.\n"]
    pub fn set_dns_target_resource(
        mut self,
        v: impl Into<BlockAssignable<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dns_target_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_target_resource = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for Route53recoveryreadinessResourceSetResourcesEl {
    type O = BlockAssignable<Route53recoveryreadinessResourceSetResourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSetResourcesEl {}

impl BuildRoute53recoveryreadinessResourceSetResourcesEl {
    pub fn build(self) -> Route53recoveryreadinessResourceSetResourcesEl {
        Route53recoveryreadinessResourceSetResourcesEl {
            readiness_scopes: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
            dns_target_resource: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Route53recoveryreadinessResourceSetResourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53recoveryreadinessResourceSetResourcesElRef {
    fn new(shared: StackShared, base: String) -> Route53recoveryreadinessResourceSetResourcesElRef {
        Route53recoveryreadinessResourceSetResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSetResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_id` after provisioning.\n"]
    pub fn component_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.component_id", self.base))
    }

    #[doc = "Get a reference to the value of field `readiness_scopes` after provisioning.\n"]
    pub fn readiness_scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.readiness_scopes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_target_resource` after provisioning.\n"]
    pub fn dns_target_resource(
        &self,
    ) -> ListRef<Route53recoveryreadinessResourceSetResourcesElDnsTargetResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_target_resource", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl Route53recoveryreadinessResourceSetTimeoutsEl {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for Route53recoveryreadinessResourceSetTimeoutsEl {
    type O = BlockAssignable<Route53recoveryreadinessResourceSetTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSetTimeoutsEl {}

impl BuildRoute53recoveryreadinessResourceSetTimeoutsEl {
    pub fn build(self) -> Route53recoveryreadinessResourceSetTimeoutsEl {
        Route53recoveryreadinessResourceSetTimeoutsEl {
            delete: core::default::Default::default(),
        }
    }
}

pub struct Route53recoveryreadinessResourceSetTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53recoveryreadinessResourceSetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Route53recoveryreadinessResourceSetTimeoutsElRef {
        Route53recoveryreadinessResourceSetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSetTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53recoveryreadinessResourceSetDynamic {
    resources: Option<DynamicBlock<Route53recoveryreadinessResourceSetResourcesEl>>,
}
