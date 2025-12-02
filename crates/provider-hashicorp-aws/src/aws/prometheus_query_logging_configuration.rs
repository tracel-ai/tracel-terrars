use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct PrometheusQueryLoggingConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    workspace_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<Vec<PrometheusQueryLoggingConfigurationDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PrometheusQueryLoggingConfigurationTimeoutsEl>,
    dynamic: PrometheusQueryLoggingConfigurationDynamic,
}
struct PrometheusQueryLoggingConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PrometheusQueryLoggingConfigurationData>,
}
#[derive(Clone)]
pub struct PrometheusQueryLoggingConfiguration(Rc<PrometheusQueryLoggingConfiguration_>);
impl PrometheusQueryLoggingConfiguration {
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
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl Into<BlockAssignable<PrometheusQueryLoggingConfigurationDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PrometheusQueryLoggingConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<PrometheusQueryLoggingConfigurationDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusQueryLoggingConfigurationTimeoutsElRef {
        PrometheusQueryLoggingConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for PrometheusQueryLoggingConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for PrometheusQueryLoggingConfiguration {}
impl ToListMappable for PrometheusQueryLoggingConfiguration {
    type O = ListRef<PrometheusQueryLoggingConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for PrometheusQueryLoggingConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_prometheus_query_logging_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildPrometheusQueryLoggingConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub workspace_id: PrimField<String>,
}
impl BuildPrometheusQueryLoggingConfiguration {
    pub fn build(self, stack: &mut Stack) -> PrometheusQueryLoggingConfiguration {
        let out =
            PrometheusQueryLoggingConfiguration(Rc::new(PrometheusQueryLoggingConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(PrometheusQueryLoggingConfigurationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    region: core::default::Default::default(),
                    workspace_id: self.workspace_id,
                    destination: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct PrometheusQueryLoggingConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusQueryLoggingConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl PrometheusQueryLoggingConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<PrometheusQueryLoggingConfigurationDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusQueryLoggingConfigurationTimeoutsElRef {
        PrometheusQueryLoggingConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsEl {
    log_group_arn: PrimField<String>,
}
impl PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsEl {}
impl ToListMappable for PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsEl {
    type O = BlockAssignable<PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsEl {
    #[doc = ""]
    pub log_group_arn: PrimField<String>,
}
impl BuildPrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsEl {
    pub fn build(self) -> PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsEl {
        PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsEl {
            log_group_arn: self.log_group_arn,
        }
    }
}
pub struct PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsElRef {
        PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_group_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrometheusQueryLoggingConfigurationDestinationElFiltersEl {
    qsp_threshold: PrimField<f64>,
}
impl PrometheusQueryLoggingConfigurationDestinationElFiltersEl {}
impl ToListMappable for PrometheusQueryLoggingConfigurationDestinationElFiltersEl {
    type O = BlockAssignable<PrometheusQueryLoggingConfigurationDestinationElFiltersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrometheusQueryLoggingConfigurationDestinationElFiltersEl {
    #[doc = ""]
    pub qsp_threshold: PrimField<f64>,
}
impl BuildPrometheusQueryLoggingConfigurationDestinationElFiltersEl {
    pub fn build(self) -> PrometheusQueryLoggingConfigurationDestinationElFiltersEl {
        PrometheusQueryLoggingConfigurationDestinationElFiltersEl {
            qsp_threshold: self.qsp_threshold,
        }
    }
}
pub struct PrometheusQueryLoggingConfigurationDestinationElFiltersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusQueryLoggingConfigurationDestinationElFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrometheusQueryLoggingConfigurationDestinationElFiltersElRef {
        PrometheusQueryLoggingConfigurationDestinationElFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrometheusQueryLoggingConfigurationDestinationElFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `qsp_threshold` after provisioning.\n"]
    pub fn qsp_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qsp_threshold", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrometheusQueryLoggingConfigurationDestinationElDynamic {
    cloudwatch_logs:
        Option<DynamicBlock<PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsEl>>,
    filters: Option<DynamicBlock<PrometheusQueryLoggingConfigurationDestinationElFiltersEl>>,
}
#[derive(Serialize)]
pub struct PrometheusQueryLoggingConfigurationDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs: Option<Vec<PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<Vec<PrometheusQueryLoggingConfigurationDestinationElFiltersEl>>,
    dynamic: PrometheusQueryLoggingConfigurationDestinationElDynamic,
}
impl PrometheusQueryLoggingConfigurationDestinationEl {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl Into<BlockAssignable<PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        mut self,
        v: impl Into<BlockAssignable<PrometheusQueryLoggingConfigurationDestinationElFiltersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrometheusQueryLoggingConfigurationDestinationEl {
    type O = BlockAssignable<PrometheusQueryLoggingConfigurationDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrometheusQueryLoggingConfigurationDestinationEl {}
impl BuildPrometheusQueryLoggingConfigurationDestinationEl {
    pub fn build(self) -> PrometheusQueryLoggingConfigurationDestinationEl {
        PrometheusQueryLoggingConfigurationDestinationEl {
            cloudwatch_logs: core::default::Default::default(),
            filters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrometheusQueryLoggingConfigurationDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusQueryLoggingConfigurationDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrometheusQueryLoggingConfigurationDestinationElRef {
        PrometheusQueryLoggingConfigurationDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrometheusQueryLoggingConfigurationDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> ListRef<PrometheusQueryLoggingConfigurationDestinationElCloudwatchLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<PrometheusQueryLoggingConfigurationDestinationElFiltersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filters", self.base))
    }
}
#[derive(Serialize)]
pub struct PrometheusQueryLoggingConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl PrometheusQueryLoggingConfigurationTimeoutsEl {
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
impl ToListMappable for PrometheusQueryLoggingConfigurationTimeoutsEl {
    type O = BlockAssignable<PrometheusQueryLoggingConfigurationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrometheusQueryLoggingConfigurationTimeoutsEl {}
impl BuildPrometheusQueryLoggingConfigurationTimeoutsEl {
    pub fn build(self) -> PrometheusQueryLoggingConfigurationTimeoutsEl {
        PrometheusQueryLoggingConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct PrometheusQueryLoggingConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusQueryLoggingConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PrometheusQueryLoggingConfigurationTimeoutsElRef {
        PrometheusQueryLoggingConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrometheusQueryLoggingConfigurationTimeoutsElRef {
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
struct PrometheusQueryLoggingConfigurationDynamic {
    destination: Option<DynamicBlock<PrometheusQueryLoggingConfigurationDestinationEl>>,
}
