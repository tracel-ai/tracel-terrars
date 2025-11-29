use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallLoggingConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_monitoring_dashboard: Option<PrimField<bool>>,
    firewall_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_configuration: Option<Vec<NetworkfirewallLoggingConfigurationLoggingConfigurationEl>>,
    dynamic: NetworkfirewallLoggingConfigurationDynamic,
}

struct NetworkfirewallLoggingConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkfirewallLoggingConfigurationData>,
}

#[derive(Clone)]
pub struct NetworkfirewallLoggingConfiguration(Rc<NetworkfirewallLoggingConfiguration_>);

impl NetworkfirewallLoggingConfiguration {
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

    #[doc = "Set the field `enable_monitoring_dashboard`.\n"]
    pub fn set_enable_monitoring_dashboard(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_monitoring_dashboard = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `logging_configuration`.\n"]
    pub fn set_logging_configuration(
        self,
        v: impl Into<BlockAssignable<NetworkfirewallLoggingConfigurationLoggingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `enable_monitoring_dashboard` after provisioning.\n"]
    pub fn enable_monitoring_dashboard(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_monitoring_dashboard", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_arn` after provisioning.\n"]
    pub fn firewall_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> ListRef<NetworkfirewallLoggingConfigurationLoggingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

impl Referable for NetworkfirewallLoggingConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetworkfirewallLoggingConfiguration { }

impl ToListMappable for NetworkfirewallLoggingConfiguration {
    type O = ListRef<NetworkfirewallLoggingConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkfirewallLoggingConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkfirewall_logging_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkfirewallLoggingConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub firewall_arn: PrimField<String>,
}

impl BuildNetworkfirewallLoggingConfiguration {
    pub fn build(self, stack: &mut Stack) -> NetworkfirewallLoggingConfiguration {
        let out = NetworkfirewallLoggingConfiguration(Rc::new(NetworkfirewallLoggingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkfirewallLoggingConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enable_monitoring_dashboard: core::default::Default::default(),
                firewall_arn: self.firewall_arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                logging_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkfirewallLoggingConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallLoggingConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl NetworkfirewallLoggingConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_monitoring_dashboard` after provisioning.\n"]
    pub fn enable_monitoring_dashboard(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_monitoring_dashboard", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_arn` after provisioning.\n"]
    pub fn firewall_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> ListRef<NetworkfirewallLoggingConfigurationLoggingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
    log_destination: RecField<PrimField<String>>,
    log_destination_type: PrimField<String>,
    log_type: PrimField<String>,
}

impl NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl { }

impl ToListMappable for NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
    type O = BlockAssignable<NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
    #[doc = ""]
    pub log_destination: RecField<PrimField<String>>,
    #[doc = ""]
    pub log_destination_type: PrimField<String>,
    #[doc = ""]
    pub log_type: PrimField<String>,
}

impl BuildNetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
    pub fn build(self) -> NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
        NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl {
            log_destination: self.log_destination,
            log_destination_type: self.log_destination_type,
            log_type: self.log_type,
        }
    }
}

pub struct NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigElRef {
        NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination_type` after provisioning.\n"]
    pub fn log_destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_destination_type", self.base))
    }

    #[doc = "Get a reference to the value of field `log_type` after provisioning.\n"]
    pub fn log_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallLoggingConfigurationLoggingConfigurationElDynamic {
    log_destination_config: Option<
        DynamicBlock<NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallLoggingConfigurationLoggingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination_config: Option<Vec<NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl>>,
    dynamic: NetworkfirewallLoggingConfigurationLoggingConfigurationElDynamic,
}

impl NetworkfirewallLoggingConfigurationLoggingConfigurationEl {
    #[doc = "Set the field `log_destination_config`.\n"]
    pub fn set_log_destination_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            NetworkfirewallLoggingConfigurationLoggingConfigurationElLogDestinationConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_destination_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_destination_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for NetworkfirewallLoggingConfigurationLoggingConfigurationEl {
    type O = BlockAssignable<NetworkfirewallLoggingConfigurationLoggingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallLoggingConfigurationLoggingConfigurationEl {}

impl BuildNetworkfirewallLoggingConfigurationLoggingConfigurationEl {
    pub fn build(self) -> NetworkfirewallLoggingConfigurationLoggingConfigurationEl {
        NetworkfirewallLoggingConfigurationLoggingConfigurationEl {
            log_destination_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct NetworkfirewallLoggingConfigurationLoggingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallLoggingConfigurationLoggingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallLoggingConfigurationLoggingConfigurationElRef {
        NetworkfirewallLoggingConfigurationLoggingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallLoggingConfigurationLoggingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallLoggingConfigurationDynamic {
    logging_configuration: Option<DynamicBlock<NetworkfirewallLoggingConfigurationLoggingConfigurationEl>>,
}
