use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkflowmonitorMonitorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    monitor_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    scope_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_resource: Option<Vec<NetworkflowmonitorMonitorLocalResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_resource: Option<Vec<NetworkflowmonitorMonitorRemoteResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkflowmonitorMonitorTimeoutsEl>,
    dynamic: NetworkflowmonitorMonitorDynamic,
}
struct NetworkflowmonitorMonitor_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkflowmonitorMonitorData>,
}
#[derive(Clone)]
pub struct NetworkflowmonitorMonitor(Rc<NetworkflowmonitorMonitor_>);
impl NetworkflowmonitorMonitor {
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
    #[doc = "Set the field `local_resource`.\n"]
    pub fn set_local_resource(
        self,
        v: impl Into<BlockAssignable<NetworkflowmonitorMonitorLocalResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().local_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.local_resource = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remote_resource`.\n"]
    pub fn set_remote_resource(
        self,
        v: impl Into<BlockAssignable<NetworkflowmonitorMonitorRemoteResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_resource = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkflowmonitorMonitorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `monitor_arn` after provisioning.\n"]
    pub fn monitor_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitor_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitor_name` after provisioning.\n"]
    pub fn monitor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitor_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope_arn` after provisioning.\n"]
    pub fn scope_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkflowmonitorMonitorTimeoutsElRef {
        NetworkflowmonitorMonitorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkflowmonitorMonitor {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkflowmonitorMonitor {}
impl ToListMappable for NetworkflowmonitorMonitor {
    type O = ListRef<NetworkflowmonitorMonitorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkflowmonitorMonitor_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkflowmonitor_monitor".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkflowmonitorMonitor {
    pub tf_id: String,
    #[doc = ""]
    pub monitor_name: PrimField<String>,
    #[doc = ""]
    pub scope_arn: PrimField<String>,
}
impl BuildNetworkflowmonitorMonitor {
    pub fn build(self, stack: &mut Stack) -> NetworkflowmonitorMonitor {
        let out = NetworkflowmonitorMonitor(Rc::new(NetworkflowmonitorMonitor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkflowmonitorMonitorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                monitor_name: self.monitor_name,
                region: core::default::Default::default(),
                scope_arn: self.scope_arn,
                tags: core::default::Default::default(),
                local_resource: core::default::Default::default(),
                remote_resource: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkflowmonitorMonitorRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkflowmonitorMonitorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkflowmonitorMonitorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `monitor_arn` after provisioning.\n"]
    pub fn monitor_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitor_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitor_name` after provisioning.\n"]
    pub fn monitor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitor_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope_arn` after provisioning.\n"]
    pub fn scope_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkflowmonitorMonitorTimeoutsElRef {
        NetworkflowmonitorMonitorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkflowmonitorMonitorLocalResourceEl {
    identifier: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl NetworkflowmonitorMonitorLocalResourceEl {}
impl ToListMappable for NetworkflowmonitorMonitorLocalResourceEl {
    type O = BlockAssignable<NetworkflowmonitorMonitorLocalResourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkflowmonitorMonitorLocalResourceEl {
    #[doc = ""]
    pub identifier: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildNetworkflowmonitorMonitorLocalResourceEl {
    pub fn build(self) -> NetworkflowmonitorMonitorLocalResourceEl {
        NetworkflowmonitorMonitorLocalResourceEl {
            identifier: self.identifier,
            type_: self.type_,
        }
    }
}
pub struct NetworkflowmonitorMonitorLocalResourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkflowmonitorMonitorLocalResourceElRef {
    fn new(shared: StackShared, base: String) -> NetworkflowmonitorMonitorLocalResourceElRef {
        NetworkflowmonitorMonitorLocalResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkflowmonitorMonitorLocalResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkflowmonitorMonitorRemoteResourceEl {
    identifier: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl NetworkflowmonitorMonitorRemoteResourceEl {}
impl ToListMappable for NetworkflowmonitorMonitorRemoteResourceEl {
    type O = BlockAssignable<NetworkflowmonitorMonitorRemoteResourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkflowmonitorMonitorRemoteResourceEl {
    #[doc = ""]
    pub identifier: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildNetworkflowmonitorMonitorRemoteResourceEl {
    pub fn build(self) -> NetworkflowmonitorMonitorRemoteResourceEl {
        NetworkflowmonitorMonitorRemoteResourceEl {
            identifier: self.identifier,
            type_: self.type_,
        }
    }
}
pub struct NetworkflowmonitorMonitorRemoteResourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkflowmonitorMonitorRemoteResourceElRef {
    fn new(shared: StackShared, base: String) -> NetworkflowmonitorMonitorRemoteResourceElRef {
        NetworkflowmonitorMonitorRemoteResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkflowmonitorMonitorRemoteResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkflowmonitorMonitorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkflowmonitorMonitorTimeoutsEl {
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
impl ToListMappable for NetworkflowmonitorMonitorTimeoutsEl {
    type O = BlockAssignable<NetworkflowmonitorMonitorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkflowmonitorMonitorTimeoutsEl {}
impl BuildNetworkflowmonitorMonitorTimeoutsEl {
    pub fn build(self) -> NetworkflowmonitorMonitorTimeoutsEl {
        NetworkflowmonitorMonitorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkflowmonitorMonitorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkflowmonitorMonitorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkflowmonitorMonitorTimeoutsElRef {
        NetworkflowmonitorMonitorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkflowmonitorMonitorTimeoutsElRef {
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
struct NetworkflowmonitorMonitorDynamic {
    local_resource: Option<DynamicBlock<NetworkflowmonitorMonitorLocalResourceEl>>,
    remote_resource: Option<DynamicBlock<NetworkflowmonitorMonitorRemoteResourceEl>>,
}
