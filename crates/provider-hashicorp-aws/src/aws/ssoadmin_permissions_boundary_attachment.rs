use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SsoadminPermissionsBoundaryAttachmentData {
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
    instance_arn: PrimField<String>,
    permission_set_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions_boundary: Option<Vec<SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SsoadminPermissionsBoundaryAttachmentTimeoutsEl>,
    dynamic: SsoadminPermissionsBoundaryAttachmentDynamic,
}

struct SsoadminPermissionsBoundaryAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsoadminPermissionsBoundaryAttachmentData>,
}

#[derive(Clone)]
pub struct SsoadminPermissionsBoundaryAttachment(Rc<SsoadminPermissionsBoundaryAttachment_>);

impl SsoadminPermissionsBoundaryAttachment {
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

    #[doc = "Set the field `permissions_boundary`.\n"]
    pub fn set_permissions_boundary(
        self,
        v: impl Into<BlockAssignable<SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().permissions_boundary = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.permissions_boundary = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<SsoadminPermissionsBoundaryAttachmentTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `permission_set_arn` after provisioning.\n"]
    pub fn permission_set_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permission_set_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `permissions_boundary` after provisioning.\n"]
    pub fn permissions_boundary(
        &self,
    ) -> ListRef<SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions_boundary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SsoadminPermissionsBoundaryAttachmentTimeoutsElRef {
        SsoadminPermissionsBoundaryAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for SsoadminPermissionsBoundaryAttachment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SsoadminPermissionsBoundaryAttachment {}

impl ToListMappable for SsoadminPermissionsBoundaryAttachment {
    type O = ListRef<SsoadminPermissionsBoundaryAttachmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SsoadminPermissionsBoundaryAttachment_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssoadmin_permissions_boundary_attachment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsoadminPermissionsBoundaryAttachment {
    pub tf_id: String,
    #[doc = ""]
    pub instance_arn: PrimField<String>,
    #[doc = ""]
    pub permission_set_arn: PrimField<String>,
}

impl BuildSsoadminPermissionsBoundaryAttachment {
    pub fn build(self, stack: &mut Stack) -> SsoadminPermissionsBoundaryAttachment {
        let out = SsoadminPermissionsBoundaryAttachment(Rc::new(
            SsoadminPermissionsBoundaryAttachment_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(SsoadminPermissionsBoundaryAttachmentData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    id: core::default::Default::default(),
                    instance_arn: self.instance_arn,
                    permission_set_arn: self.permission_set_arn,
                    region: core::default::Default::default(),
                    permissions_boundary: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsoadminPermissionsBoundaryAttachmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminPermissionsBoundaryAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SsoadminPermissionsBoundaryAttachmentRef {
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

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `permission_set_arn` after provisioning.\n"]
    pub fn permission_set_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permission_set_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `permissions_boundary` after provisioning.\n"]
    pub fn permissions_boundary(
        &self,
    ) -> ListRef<SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions_boundary", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SsoadminPermissionsBoundaryAttachmentTimeoutsElRef {
        SsoadminPermissionsBoundaryAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceEl
{
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable
    for SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceEl
{
    type O = BlockAssignable<
        SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceEl
{
    #[doc = ""]
    pub name: PrimField<String>,
}

impl
    BuildSsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceEl
{
    pub fn build(
        self,
    ) -> SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceEl
    {
        SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceEl {
            name: self.name,
            path: core::default::Default::default(),
        }
    }
}

pub struct SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceElRef {
        SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElDynamic {
    customer_managed_policy_reference: Option<
        DynamicBlock<SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceEl>,
    >,
}

#[derive(Serialize)]
pub struct SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_policy_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_policy_reference: Option<
        Vec<SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceEl>,
    >,
    dynamic: SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElDynamic,
}

impl SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryEl {
    #[doc = "Set the field `managed_policy_arn`.\n"]
    pub fn set_managed_policy_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.managed_policy_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `customer_managed_policy_reference`.\n"]
    pub fn set_customer_managed_policy_reference(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customer_managed_policy_reference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_managed_policy_reference = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryEl {
    type O = BlockAssignable<SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminPermissionsBoundaryAttachmentPermissionsBoundaryEl {}

impl BuildSsoadminPermissionsBoundaryAttachmentPermissionsBoundaryEl {
    pub fn build(self) -> SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryEl {
        SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryEl {
            managed_policy_arn: core::default::Default::default(),
            customer_managed_policy_reference: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElRef {
        SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_policy_arn` after provisioning.\n"]
    pub fn managed_policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_policy_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `customer_managed_policy_reference` after provisioning.\n"]
    pub fn customer_managed_policy_reference(
        &self,
    ) -> ListRef<SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryElCustomerManagedPolicyReferenceElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_policy_reference", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SsoadminPermissionsBoundaryAttachmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl SsoadminPermissionsBoundaryAttachmentTimeoutsEl {
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
}

impl ToListMappable for SsoadminPermissionsBoundaryAttachmentTimeoutsEl {
    type O = BlockAssignable<SsoadminPermissionsBoundaryAttachmentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminPermissionsBoundaryAttachmentTimeoutsEl {}

impl BuildSsoadminPermissionsBoundaryAttachmentTimeoutsEl {
    pub fn build(self) -> SsoadminPermissionsBoundaryAttachmentTimeoutsEl {
        SsoadminPermissionsBoundaryAttachmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct SsoadminPermissionsBoundaryAttachmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminPermissionsBoundaryAttachmentTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsoadminPermissionsBoundaryAttachmentTimeoutsElRef {
        SsoadminPermissionsBoundaryAttachmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminPermissionsBoundaryAttachmentTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct SsoadminPermissionsBoundaryAttachmentDynamic {
    permissions_boundary:
        Option<DynamicBlock<SsoadminPermissionsBoundaryAttachmentPermissionsBoundaryEl>>,
}
