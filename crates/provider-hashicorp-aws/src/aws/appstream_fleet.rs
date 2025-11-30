use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct AppstreamFleetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disconnect_timeout_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_default_internet_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_disconnect_timeout_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_name: Option<PrimField<String>>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_sessions_per_instance: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_user_duration_in_seconds: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_view: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_capacity: Option<Vec<AppstreamFleetComputeCapacityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_join_info: Option<Vec<AppstreamFleetDomainJoinInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<AppstreamFleetVpcConfigEl>>,
    dynamic: AppstreamFleetDynamic,
}

struct AppstreamFleet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppstreamFleetData>,
}

#[derive(Clone)]
pub struct AppstreamFleet(Rc<AppstreamFleet_>);

impl AppstreamFleet {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `disconnect_timeout_in_seconds`.\n"]
    pub fn set_disconnect_timeout_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().disconnect_timeout_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_default_internet_access`.\n"]
    pub fn set_enable_default_internet_access(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_default_internet_access = Some(v.into());
        self
    }

    #[doc = "Set the field `fleet_type`.\n"]
    pub fn set_fleet_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fleet_type = Some(v.into());
        self
    }

    #[doc = "Set the field `iam_role_arn`.\n"]
    pub fn set_iam_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `idle_disconnect_timeout_in_seconds`.\n"]
    pub fn set_idle_disconnect_timeout_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_disconnect_timeout_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `image_arn`.\n"]
    pub fn set_image_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `image_name`.\n"]
    pub fn set_image_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_name = Some(v.into());
        self
    }

    #[doc = "Set the field `max_sessions_per_instance`.\n"]
    pub fn set_max_sessions_per_instance(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_sessions_per_instance = Some(v.into());
        self
    }

    #[doc = "Set the field `max_user_duration_in_seconds`.\n"]
    pub fn set_max_user_duration_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_user_duration_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `stream_view`.\n"]
    pub fn set_stream_view(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stream_view = Some(v.into());
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

    #[doc = "Set the field `compute_capacity`.\n"]
    pub fn set_compute_capacity(
        self,
        v: impl Into<BlockAssignable<AppstreamFleetComputeCapacityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_capacity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_capacity = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `domain_join_info`.\n"]
    pub fn set_domain_join_info(
        self,
        v: impl Into<BlockAssignable<AppstreamFleetDomainJoinInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domain_join_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domain_join_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(self, v: impl Into<BlockAssignable<AppstreamFleetVpcConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disconnect_timeout_in_seconds` after provisioning.\n"]
    pub fn disconnect_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disconnect_timeout_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_default_internet_access` after provisioning.\n"]
    pub fn enable_default_internet_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_default_internet_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fleet_type` after provisioning.\n"]
    pub fn fleet_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_disconnect_timeout_in_seconds` after provisioning.\n"]
    pub fn idle_disconnect_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_disconnect_timeout_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `image_arn` after provisioning.\n"]
    pub fn image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_sessions_per_instance` after provisioning.\n"]
    pub fn max_sessions_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_sessions_per_instance", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_user_duration_in_seconds` after provisioning.\n"]
    pub fn max_user_duration_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_user_duration_in_seconds", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stream_view` after provisioning.\n"]
    pub fn stream_view(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_view", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `compute_capacity` after provisioning.\n"]
    pub fn compute_capacity(&self) -> ListRef<AppstreamFleetComputeCapacityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_join_info` after provisioning.\n"]
    pub fn domain_join_info(&self) -> ListRef<AppstreamFleetDomainJoinInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_join_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<AppstreamFleetVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}

impl Referable for AppstreamFleet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for AppstreamFleet {}

impl ToListMappable for AppstreamFleet {
    type O = ListRef<AppstreamFleetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppstreamFleet_ {
    fn extract_resource_type(&self) -> String {
        "aws_appstream_fleet".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppstreamFleet {
    pub tf_id: String,
    #[doc = ""]
    pub instance_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppstreamFleet {
    pub fn build(self, stack: &mut Stack) -> AppstreamFleet {
        let out = AppstreamFleet(Rc::new(AppstreamFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppstreamFleetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                disconnect_timeout_in_seconds: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enable_default_internet_access: core::default::Default::default(),
                fleet_type: core::default::Default::default(),
                iam_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                idle_disconnect_timeout_in_seconds: core::default::Default::default(),
                image_arn: core::default::Default::default(),
                image_name: core::default::Default::default(),
                instance_type: self.instance_type,
                max_sessions_per_instance: core::default::Default::default(),
                max_user_duration_in_seconds: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                stream_view: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                compute_capacity: core::default::Default::default(),
                domain_join_info: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppstreamFleetRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamFleetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl AppstreamFleetRef {
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

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disconnect_timeout_in_seconds` after provisioning.\n"]
    pub fn disconnect_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disconnect_timeout_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_default_internet_access` after provisioning.\n"]
    pub fn enable_default_internet_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_default_internet_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fleet_type` after provisioning.\n"]
    pub fn fleet_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_disconnect_timeout_in_seconds` after provisioning.\n"]
    pub fn idle_disconnect_timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_disconnect_timeout_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `image_arn` after provisioning.\n"]
    pub fn image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_sessions_per_instance` after provisioning.\n"]
    pub fn max_sessions_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_sessions_per_instance", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_user_duration_in_seconds` after provisioning.\n"]
    pub fn max_user_duration_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_user_duration_in_seconds", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `stream_view` after provisioning.\n"]
    pub fn stream_view(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stream_view", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `compute_capacity` after provisioning.\n"]
    pub fn compute_capacity(&self) -> ListRef<AppstreamFleetComputeCapacityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_join_info` after provisioning.\n"]
    pub fn domain_join_info(&self) -> ListRef<AppstreamFleetDomainJoinInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_join_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<AppstreamFleetVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AppstreamFleetComputeCapacityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_sessions: Option<PrimField<f64>>,
}

impl AppstreamFleetComputeCapacityEl {
    #[doc = "Set the field `desired_instances`.\n"]
    pub fn set_desired_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.desired_instances = Some(v.into());
        self
    }

    #[doc = "Set the field `desired_sessions`.\n"]
    pub fn set_desired_sessions(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.desired_sessions = Some(v.into());
        self
    }
}

impl ToListMappable for AppstreamFleetComputeCapacityEl {
    type O = BlockAssignable<AppstreamFleetComputeCapacityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamFleetComputeCapacityEl {}

impl BuildAppstreamFleetComputeCapacityEl {
    pub fn build(self) -> AppstreamFleetComputeCapacityEl {
        AppstreamFleetComputeCapacityEl {
            desired_instances: core::default::Default::default(),
            desired_sessions: core::default::Default::default(),
        }
    }
}

pub struct AppstreamFleetComputeCapacityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamFleetComputeCapacityElRef {
    fn new(shared: StackShared, base: String) -> AppstreamFleetComputeCapacityElRef {
        AppstreamFleetComputeCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamFleetComputeCapacityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available` after provisioning.\n"]
    pub fn available(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.available", self.base))
    }

    #[doc = "Get a reference to the value of field `desired_instances` after provisioning.\n"]
    pub fn desired_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_instances", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `desired_sessions` after provisioning.\n"]
    pub fn desired_sessions(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_sessions", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `in_use` after provisioning.\n"]
    pub fn in_use(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.in_use", self.base))
    }

    #[doc = "Get a reference to the value of field `running` after provisioning.\n"]
    pub fn running(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.running", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamFleetDomainJoinInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit_distinguished_name: Option<PrimField<String>>,
}

impl AppstreamFleetDomainJoinInfoEl {
    #[doc = "Set the field `directory_name`.\n"]
    pub fn set_directory_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.directory_name = Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_distinguished_name`.\n"]
    pub fn set_organizational_unit_distinguished_name(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.organizational_unit_distinguished_name = Some(v.into());
        self
    }
}

impl ToListMappable for AppstreamFleetDomainJoinInfoEl {
    type O = BlockAssignable<AppstreamFleetDomainJoinInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamFleetDomainJoinInfoEl {}

impl BuildAppstreamFleetDomainJoinInfoEl {
    pub fn build(self) -> AppstreamFleetDomainJoinInfoEl {
        AppstreamFleetDomainJoinInfoEl {
            directory_name: core::default::Default::default(),
            organizational_unit_distinguished_name: core::default::Default::default(),
        }
    }
}

pub struct AppstreamFleetDomainJoinInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamFleetDomainJoinInfoElRef {
    fn new(shared: StackShared, base: String) -> AppstreamFleetDomainJoinInfoElRef {
        AppstreamFleetDomainJoinInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamFleetDomainJoinInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `organizational_unit_distinguished_name` after provisioning.\n"]
    pub fn organizational_unit_distinguished_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organizational_unit_distinguished_name", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppstreamFleetVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<ListField<PrimField<String>>>,
}

impl AppstreamFleetVpcConfigEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }
}

impl ToListMappable for AppstreamFleetVpcConfigEl {
    type O = BlockAssignable<AppstreamFleetVpcConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamFleetVpcConfigEl {}

impl BuildAppstreamFleetVpcConfigEl {
    pub fn build(self) -> AppstreamFleetVpcConfigEl {
        AppstreamFleetVpcConfigEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

pub struct AppstreamFleetVpcConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamFleetVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> AppstreamFleetVpcConfigElRef {
        AppstreamFleetVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamFleetVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppstreamFleetDynamic {
    compute_capacity: Option<DynamicBlock<AppstreamFleetComputeCapacityEl>>,
    domain_join_info: Option<DynamicBlock<AppstreamFleetDomainJoinInfoEl>>,
    vpc_config: Option<DynamicBlock<AppstreamFleetVpcConfigEl>>,
}
