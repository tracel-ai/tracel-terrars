use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigtableGcPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    column_family: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gc_rules: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_warnings: Option<PrimField<bool>>,
    instance_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    table: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<Vec<BigtableGcPolicyMaxAgeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_version: Option<Vec<BigtableGcPolicyMaxVersionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigtableGcPolicyTimeoutsEl>,
    dynamic: BigtableGcPolicyDynamic,
}
struct BigtableGcPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigtableGcPolicyData>,
}
#[derive(Clone)]
pub struct BigtableGcPolicy(Rc<BigtableGcPolicy_>);
impl BigtableGcPolicy {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `gc_rules`.\nSerialized JSON string for garbage collection policy. Conflicts with \"mode\", \"max_age\" and \"max_version\"."]
    pub fn set_gc_rules(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gc_rules = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_warnings`.\nAllows ignoring warnings when updating the GC policy. This can be used\n\t\t\t\tto increase the gc policy on replicated clusters. Doing this may make clusters be\n\t\t\t\tinconsistent for a longer period of time, before using this make sure you understand\n\t\t\t\tthe risks listed at https://cloud.google.com/bigtable/docs/garbage-collection#increasing"]
    pub fn set_ignore_warnings(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_warnings = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nNOTE: 'gc_rules' is more flexible, and should be preferred over this field for new resources. This field may be deprecated in the future. If multiple policies are set, you should choose between UNION OR INTERSECTION."]
    pub fn set_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mode = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(self, v: impl Into<BlockAssignable<BigtableGcPolicyMaxAgeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().max_age = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.max_age = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `max_version`.\n"]
    pub fn set_max_version(
        self,
        v: impl Into<BlockAssignable<BigtableGcPolicyMaxVersionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().max_version = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.max_version = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigtableGcPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `column_family` after provisioning.\nThe name of the column family."]
    pub fn column_family(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.column_family", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gc_rules` after provisioning.\nSerialized JSON string for garbage collection policy. Conflicts with \"mode\", \"max_age\" and \"max_version\"."]
    pub fn gc_rules(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gc_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_warnings` after provisioning.\nAllows ignoring warnings when updating the GC policy. This can be used\n\t\t\t\tto increase the gc policy on replicated clusters. Doing this may make clusters be\n\t\t\t\tinconsistent for a longer period of time, before using this make sure you understand\n\t\t\t\tthe risks listed at https://cloud.google.com/bigtable/docs/garbage-collection#increasing"]
    pub fn ignore_warnings(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_warnings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_name` after provisioning.\nThe name of the Bigtable instance."]
    pub fn instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nNOTE: 'gc_rules' is more flexible, and should be preferred over this field for new resources. This field may be deprecated in the future. If multiple policies are set, you should choose between UNION OR INTERSECTION."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe name of the table."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> ListRef<BigtableGcPolicyMaxAgeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_age", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_version` after provisioning.\n"]
    pub fn max_version(&self) -> ListRef<BigtableGcPolicyMaxVersionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableGcPolicyTimeoutsElRef {
        BigtableGcPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigtableGcPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigtableGcPolicy {}
impl ToListMappable for BigtableGcPolicy {
    type O = ListRef<BigtableGcPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigtableGcPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_bigtable_gc_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigtableGcPolicy {
    pub tf_id: String,
    #[doc = "The name of the column family."]
    pub column_family: PrimField<String>,
    #[doc = "The name of the Bigtable instance."]
    pub instance_name: PrimField<String>,
    #[doc = "The name of the table."]
    pub table: PrimField<String>,
}
impl BuildBigtableGcPolicy {
    pub fn build(self, stack: &mut Stack) -> BigtableGcPolicy {
        let out = BigtableGcPolicy(Rc::new(BigtableGcPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigtableGcPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                column_family: self.column_family,
                deletion_policy: core::default::Default::default(),
                gc_rules: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_warnings: core::default::Default::default(),
                instance_name: self.instance_name,
                mode: core::default::Default::default(),
                project: core::default::Default::default(),
                table: self.table,
                max_age: core::default::Default::default(),
                max_version: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigtableGcPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableGcPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigtableGcPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_family` after provisioning.\nThe name of the column family."]
    pub fn column_family(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.column_family", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gc_rules` after provisioning.\nSerialized JSON string for garbage collection policy. Conflicts with \"mode\", \"max_age\" and \"max_version\"."]
    pub fn gc_rules(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gc_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_warnings` after provisioning.\nAllows ignoring warnings when updating the GC policy. This can be used\n\t\t\t\tto increase the gc policy on replicated clusters. Doing this may make clusters be\n\t\t\t\tinconsistent for a longer period of time, before using this make sure you understand\n\t\t\t\tthe risks listed at https://cloud.google.com/bigtable/docs/garbage-collection#increasing"]
    pub fn ignore_warnings(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_warnings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_name` after provisioning.\nThe name of the Bigtable instance."]
    pub fn instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nNOTE: 'gc_rules' is more flexible, and should be preferred over this field for new resources. This field may be deprecated in the future. If multiple policies are set, you should choose between UNION OR INTERSECTION."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe name of the table."]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> ListRef<BigtableGcPolicyMaxAgeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_age", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_version` after provisioning.\n"]
    pub fn max_version(&self) -> ListRef<BigtableGcPolicyMaxVersionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableGcPolicyTimeoutsElRef {
        BigtableGcPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigtableGcPolicyMaxAgeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
}
impl BigtableGcPolicyMaxAgeEl {
    #[doc = "Set the field `days`.\nNumber of days before applying GC policy."]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }
    #[doc = "Set the field `duration`.\nDuration before applying GC policy"]
    pub fn set_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.duration = Some(v.into());
        self
    }
}
impl ToListMappable for BigtableGcPolicyMaxAgeEl {
    type O = BlockAssignable<BigtableGcPolicyMaxAgeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableGcPolicyMaxAgeEl {}
impl BuildBigtableGcPolicyMaxAgeEl {
    pub fn build(self) -> BigtableGcPolicyMaxAgeEl {
        BigtableGcPolicyMaxAgeEl {
            days: core::default::Default::default(),
            duration: core::default::Default::default(),
        }
    }
}
pub struct BigtableGcPolicyMaxAgeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableGcPolicyMaxAgeElRef {
    fn new(shared: StackShared, base: String) -> BigtableGcPolicyMaxAgeElRef {
        BigtableGcPolicyMaxAgeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableGcPolicyMaxAgeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `days` after provisioning.\nNumber of days before applying GC policy."]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\nDuration before applying GC policy"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
}
#[derive(Serialize)]
pub struct BigtableGcPolicyMaxVersionEl {
    number: PrimField<f64>,
}
impl BigtableGcPolicyMaxVersionEl {}
impl ToListMappable for BigtableGcPolicyMaxVersionEl {
    type O = BlockAssignable<BigtableGcPolicyMaxVersionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableGcPolicyMaxVersionEl {
    #[doc = "Number of version before applying the GC policy."]
    pub number: PrimField<f64>,
}
impl BuildBigtableGcPolicyMaxVersionEl {
    pub fn build(self) -> BigtableGcPolicyMaxVersionEl {
        BigtableGcPolicyMaxVersionEl {
            number: self.number,
        }
    }
}
pub struct BigtableGcPolicyMaxVersionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableGcPolicyMaxVersionElRef {
    fn new(shared: StackShared, base: String) -> BigtableGcPolicyMaxVersionElRef {
        BigtableGcPolicyMaxVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableGcPolicyMaxVersionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `number` after provisioning.\nNumber of version before applying the GC policy."]
    pub fn number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number", self.base))
    }
}
#[derive(Serialize)]
pub struct BigtableGcPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl BigtableGcPolicyTimeoutsEl {
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
impl ToListMappable for BigtableGcPolicyTimeoutsEl {
    type O = BlockAssignable<BigtableGcPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableGcPolicyTimeoutsEl {}
impl BuildBigtableGcPolicyTimeoutsEl {
    pub fn build(self) -> BigtableGcPolicyTimeoutsEl {
        BigtableGcPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct BigtableGcPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableGcPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigtableGcPolicyTimeoutsElRef {
        BigtableGcPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableGcPolicyTimeoutsElRef {
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
struct BigtableGcPolicyDynamic {
    max_age: Option<DynamicBlock<BigtableGcPolicyMaxAgeEl>>,
    max_version: Option<DynamicBlock<BigtableGcPolicyMaxVersionEl>>,
}
