use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DatasyncLocationEfsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_point_arn: Option<PrimField<String>>,
    efs_file_system_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_access_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_transit_encryption: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subdirectory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_config: Option<Vec<DatasyncLocationEfsEc2ConfigEl>>,
    dynamic: DatasyncLocationEfsDynamic,
}
struct DatasyncLocationEfs_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatasyncLocationEfsData>,
}
#[derive(Clone)]
pub struct DatasyncLocationEfs(Rc<DatasyncLocationEfs_>);
impl DatasyncLocationEfs {
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
    #[doc = "Set the field `access_point_arn`.\n"]
    pub fn set_access_point_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_point_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `file_system_access_role_arn`.\n"]
    pub fn set_file_system_access_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().file_system_access_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `in_transit_encryption`.\n"]
    pub fn set_in_transit_encryption(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().in_transit_encryption = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `subdirectory`.\n"]
    pub fn set_subdirectory(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subdirectory = Some(v.into());
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
    #[doc = "Set the field `ec2_config`.\n"]
    pub fn set_ec2_config(
        self,
        v: impl Into<BlockAssignable<DatasyncLocationEfsEc2ConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ec2_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ec2_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_point_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `efs_file_system_arn` after provisioning.\n"]
    pub fn efs_file_system_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.efs_file_system_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_access_role_arn` after provisioning.\n"]
    pub fn file_system_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_access_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `in_transit_encryption` after provisioning.\n"]
    pub fn in_transit_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_transit_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subdirectory", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ec2_config` after provisioning.\n"]
    pub fn ec2_config(&self) -> ListRef<DatasyncLocationEfsEc2ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ec2_config", self.extract_ref()),
        )
    }
}
impl Referable for DatasyncLocationEfs {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DatasyncLocationEfs {}
impl ToListMappable for DatasyncLocationEfs {
    type O = ListRef<DatasyncLocationEfsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DatasyncLocationEfs_ {
    fn extract_resource_type(&self) -> String {
        "aws_datasync_location_efs".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDatasyncLocationEfs {
    pub tf_id: String,
    #[doc = ""]
    pub efs_file_system_arn: PrimField<String>,
}
impl BuildDatasyncLocationEfs {
    pub fn build(self, stack: &mut Stack) -> DatasyncLocationEfs {
        let out = DatasyncLocationEfs(Rc::new(DatasyncLocationEfs_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatasyncLocationEfsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_point_arn: core::default::Default::default(),
                efs_file_system_arn: self.efs_file_system_arn,
                file_system_access_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                in_transit_encryption: core::default::Default::default(),
                region: core::default::Default::default(),
                subdirectory: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                ec2_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DatasyncLocationEfsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatasyncLocationEfsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DatasyncLocationEfsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_point_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `efs_file_system_arn` after provisioning.\n"]
    pub fn efs_file_system_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.efs_file_system_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_access_role_arn` after provisioning.\n"]
    pub fn file_system_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_access_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `in_transit_encryption` after provisioning.\n"]
    pub fn in_transit_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_transit_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subdirectory", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ec2_config` after provisioning.\n"]
    pub fn ec2_config(&self) -> ListRef<DatasyncLocationEfsEc2ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ec2_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DatasyncLocationEfsEc2ConfigEl {
    security_group_arns: SetField<PrimField<String>>,
    subnet_arn: PrimField<String>,
}
impl DatasyncLocationEfsEc2ConfigEl {}
impl ToListMappable for DatasyncLocationEfsEc2ConfigEl {
    type O = BlockAssignable<DatasyncLocationEfsEc2ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatasyncLocationEfsEc2ConfigEl {
    #[doc = ""]
    pub security_group_arns: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnet_arn: PrimField<String>,
}
impl BuildDatasyncLocationEfsEc2ConfigEl {
    pub fn build(self) -> DatasyncLocationEfsEc2ConfigEl {
        DatasyncLocationEfsEc2ConfigEl {
            security_group_arns: self.security_group_arns,
            subnet_arn: self.subnet_arn,
        }
    }
}
pub struct DatasyncLocationEfsEc2ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatasyncLocationEfsEc2ConfigElRef {
    fn new(shared: StackShared, base: String) -> DatasyncLocationEfsEc2ConfigElRef {
        DatasyncLocationEfsEc2ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatasyncLocationEfsEc2ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_group_arns` after provisioning.\n"]
    pub fn security_group_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_arns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_arn` after provisioning.\n"]
    pub fn subnet_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_arn", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatasyncLocationEfsDynamic {
    ec2_config: Option<DynamicBlock<DatasyncLocationEfsEc2ConfigEl>>,
}
