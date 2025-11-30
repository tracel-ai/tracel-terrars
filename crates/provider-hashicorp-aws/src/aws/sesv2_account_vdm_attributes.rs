use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct Sesv2AccountVdmAttributesData {
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
    region: Option<PrimField<String>>,
    vdm_enabled: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dashboard_attributes: Option<Vec<Sesv2AccountVdmAttributesDashboardAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardian_attributes: Option<Vec<Sesv2AccountVdmAttributesGuardianAttributesEl>>,
    dynamic: Sesv2AccountVdmAttributesDynamic,
}

struct Sesv2AccountVdmAttributes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Sesv2AccountVdmAttributesData>,
}

#[derive(Clone)]
pub struct Sesv2AccountVdmAttributes(Rc<Sesv2AccountVdmAttributes_>);

impl Sesv2AccountVdmAttributes {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `dashboard_attributes`.\n"]
    pub fn set_dashboard_attributes(
        self,
        v: impl Into<BlockAssignable<Sesv2AccountVdmAttributesDashboardAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dashboard_attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dashboard_attributes = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `guardian_attributes`.\n"]
    pub fn set_guardian_attributes(
        self,
        v: impl Into<BlockAssignable<Sesv2AccountVdmAttributesGuardianAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().guardian_attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.guardian_attributes = Some(d);
            }
        }
        self
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

    #[doc = "Get a reference to the value of field `vdm_enabled` after provisioning.\n"]
    pub fn vdm_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vdm_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dashboard_attributes` after provisioning.\n"]
    pub fn dashboard_attributes(
        &self,
    ) -> ListRef<Sesv2AccountVdmAttributesDashboardAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dashboard_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `guardian_attributes` after provisioning.\n"]
    pub fn guardian_attributes(&self) -> ListRef<Sesv2AccountVdmAttributesGuardianAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardian_attributes", self.extract_ref()),
        )
    }
}

impl Referable for Sesv2AccountVdmAttributes {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Sesv2AccountVdmAttributes {}

impl ToListMappable for Sesv2AccountVdmAttributes {
    type O = ListRef<Sesv2AccountVdmAttributesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Sesv2AccountVdmAttributes_ {
    fn extract_resource_type(&self) -> String {
        "aws_sesv2_account_vdm_attributes".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesv2AccountVdmAttributes {
    pub tf_id: String,
    #[doc = ""]
    pub vdm_enabled: PrimField<String>,
}

impl BuildSesv2AccountVdmAttributes {
    pub fn build(self, stack: &mut Stack) -> Sesv2AccountVdmAttributes {
        let out = Sesv2AccountVdmAttributes(Rc::new(Sesv2AccountVdmAttributes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Sesv2AccountVdmAttributesData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                vdm_enabled: self.vdm_enabled,
                dashboard_attributes: core::default::Default::default(),
                guardian_attributes: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Sesv2AccountVdmAttributesRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2AccountVdmAttributesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl Sesv2AccountVdmAttributesRef {
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vdm_enabled` after provisioning.\n"]
    pub fn vdm_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vdm_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dashboard_attributes` after provisioning.\n"]
    pub fn dashboard_attributes(
        &self,
    ) -> ListRef<Sesv2AccountVdmAttributesDashboardAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dashboard_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `guardian_attributes` after provisioning.\n"]
    pub fn guardian_attributes(&self) -> ListRef<Sesv2AccountVdmAttributesGuardianAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guardian_attributes", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Sesv2AccountVdmAttributesDashboardAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    engagement_metrics: Option<PrimField<String>>,
}

impl Sesv2AccountVdmAttributesDashboardAttributesEl {
    #[doc = "Set the field `engagement_metrics`.\n"]
    pub fn set_engagement_metrics(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.engagement_metrics = Some(v.into());
        self
    }
}

impl ToListMappable for Sesv2AccountVdmAttributesDashboardAttributesEl {
    type O = BlockAssignable<Sesv2AccountVdmAttributesDashboardAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2AccountVdmAttributesDashboardAttributesEl {}

impl BuildSesv2AccountVdmAttributesDashboardAttributesEl {
    pub fn build(self) -> Sesv2AccountVdmAttributesDashboardAttributesEl {
        Sesv2AccountVdmAttributesDashboardAttributesEl {
            engagement_metrics: core::default::Default::default(),
        }
    }
}

pub struct Sesv2AccountVdmAttributesDashboardAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2AccountVdmAttributesDashboardAttributesElRef {
    fn new(shared: StackShared, base: String) -> Sesv2AccountVdmAttributesDashboardAttributesElRef {
        Sesv2AccountVdmAttributesDashboardAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2AccountVdmAttributesDashboardAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `engagement_metrics` after provisioning.\n"]
    pub fn engagement_metrics(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engagement_metrics", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Sesv2AccountVdmAttributesGuardianAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    optimized_shared_delivery: Option<PrimField<String>>,
}

impl Sesv2AccountVdmAttributesGuardianAttributesEl {
    #[doc = "Set the field `optimized_shared_delivery`.\n"]
    pub fn set_optimized_shared_delivery(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.optimized_shared_delivery = Some(v.into());
        self
    }
}

impl ToListMappable for Sesv2AccountVdmAttributesGuardianAttributesEl {
    type O = BlockAssignable<Sesv2AccountVdmAttributesGuardianAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2AccountVdmAttributesGuardianAttributesEl {}

impl BuildSesv2AccountVdmAttributesGuardianAttributesEl {
    pub fn build(self) -> Sesv2AccountVdmAttributesGuardianAttributesEl {
        Sesv2AccountVdmAttributesGuardianAttributesEl {
            optimized_shared_delivery: core::default::Default::default(),
        }
    }
}

pub struct Sesv2AccountVdmAttributesGuardianAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Sesv2AccountVdmAttributesGuardianAttributesElRef {
    fn new(shared: StackShared, base: String) -> Sesv2AccountVdmAttributesGuardianAttributesElRef {
        Sesv2AccountVdmAttributesGuardianAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2AccountVdmAttributesGuardianAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `optimized_shared_delivery` after provisioning.\n"]
    pub fn optimized_shared_delivery(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.optimized_shared_delivery", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct Sesv2AccountVdmAttributesDynamic {
    dashboard_attributes: Option<DynamicBlock<Sesv2AccountVdmAttributesDashboardAttributesEl>>,
    guardian_attributes: Option<DynamicBlock<Sesv2AccountVdmAttributesGuardianAttributesEl>>,
}
