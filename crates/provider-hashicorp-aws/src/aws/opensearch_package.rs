use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OpensearchPackageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_description: Option<PrimField<String>>,
    package_name: PrimField<String>,
    package_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_source: Option<Vec<OpensearchPackagePackageSourceEl>>,
    dynamic: OpensearchPackageDynamic,
}
struct OpensearchPackage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpensearchPackageData>,
}
#[derive(Clone)]
pub struct OpensearchPackage(Rc<OpensearchPackage_>);
impl OpensearchPackage {
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
    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_version = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `package_description`.\n"]
    pub fn set_package_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().package_description = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `package_source`.\n"]
    pub fn set_package_source(
        self,
        v: impl Into<BlockAssignable<OpensearchPackagePackageSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().package_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.package_source = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `available_package_version` after provisioning.\n"]
    pub fn available_package_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_package_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `package_description` after provisioning.\n"]
    pub fn package_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_id` after provisioning.\n"]
    pub fn package_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_name` after provisioning.\n"]
    pub fn package_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_type` after provisioning.\n"]
    pub fn package_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_source` after provisioning.\n"]
    pub fn package_source(&self) -> ListRef<OpensearchPackagePackageSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.package_source", self.extract_ref()),
        )
    }
}
impl Referable for OpensearchPackage {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OpensearchPackage {}
impl ToListMappable for OpensearchPackage {
    type O = ListRef<OpensearchPackageRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OpensearchPackage_ {
    fn extract_resource_type(&self) -> String {
        "aws_opensearch_package".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOpensearchPackage {
    pub tf_id: String,
    #[doc = ""]
    pub package_name: PrimField<String>,
    #[doc = ""]
    pub package_type: PrimField<String>,
}
impl BuildOpensearchPackage {
    pub fn build(self, stack: &mut Stack) -> OpensearchPackage {
        let out = OpensearchPackage(Rc::new(OpensearchPackage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OpensearchPackageData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                engine_version: core::default::Default::default(),
                id: core::default::Default::default(),
                package_description: core::default::Default::default(),
                package_name: self.package_name,
                package_type: self.package_type,
                region: core::default::Default::default(),
                package_source: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct OpensearchPackageRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchPackageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OpensearchPackageRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `available_package_version` after provisioning.\n"]
    pub fn available_package_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_package_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `package_description` after provisioning.\n"]
    pub fn package_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_id` after provisioning.\n"]
    pub fn package_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_name` after provisioning.\n"]
    pub fn package_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_type` after provisioning.\n"]
    pub fn package_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_source` after provisioning.\n"]
    pub fn package_source(&self) -> ListRef<OpensearchPackagePackageSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.package_source", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OpensearchPackagePackageSourceEl {
    s3_bucket_name: PrimField<String>,
    s3_key: PrimField<String>,
}
impl OpensearchPackagePackageSourceEl {}
impl ToListMappable for OpensearchPackagePackageSourceEl {
    type O = BlockAssignable<OpensearchPackagePackageSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchPackagePackageSourceEl {
    #[doc = ""]
    pub s3_bucket_name: PrimField<String>,
    #[doc = ""]
    pub s3_key: PrimField<String>,
}
impl BuildOpensearchPackagePackageSourceEl {
    pub fn build(self) -> OpensearchPackagePackageSourceEl {
        OpensearchPackagePackageSourceEl {
            s3_bucket_name: self.s3_bucket_name,
            s3_key: self.s3_key,
        }
    }
}
pub struct OpensearchPackagePackageSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchPackagePackageSourceElRef {
    fn new(shared: StackShared, base: String) -> OpensearchPackagePackageSourceElRef {
        OpensearchPackagePackageSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchPackagePackageSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_key", self.base))
    }
}
#[derive(Serialize, Default)]
struct OpensearchPackageDynamic {
    package_source: Option<DynamicBlock<OpensearchPackagePackageSourceEl>>,
}
