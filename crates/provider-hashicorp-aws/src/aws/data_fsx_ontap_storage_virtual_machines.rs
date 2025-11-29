use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataFsxOntapStorageVirtualMachinesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataFsxOntapStorageVirtualMachinesFilterEl>>,
    dynamic: DataFsxOntapStorageVirtualMachinesDynamic,
}

struct DataFsxOntapStorageVirtualMachines_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataFsxOntapStorageVirtualMachinesData>,
}

#[derive(Clone)]
pub struct DataFsxOntapStorageVirtualMachines(Rc<DataFsxOntapStorageVirtualMachines_>);

impl DataFsxOntapStorageVirtualMachines {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataFsxOntapStorageVirtualMachinesFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for DataFsxOntapStorageVirtualMachines {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataFsxOntapStorageVirtualMachines { }

impl ToListMappable for DataFsxOntapStorageVirtualMachines {
    type O = ListRef<DataFsxOntapStorageVirtualMachinesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataFsxOntapStorageVirtualMachines_ {
    fn extract_datasource_type(&self) -> String {
        "aws_fsx_ontap_storage_virtual_machines".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachines {
    pub tf_id: String,
}

impl BuildDataFsxOntapStorageVirtualMachines {
    pub fn build(self, stack: &mut Stack) -> DataFsxOntapStorageVirtualMachines {
        let out = DataFsxOntapStorageVirtualMachines(Rc::new(DataFsxOntapStorageVirtualMachines_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataFsxOntapStorageVirtualMachinesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataFsxOntapStorageVirtualMachinesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachinesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataFsxOntapStorageVirtualMachinesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataFsxOntapStorageVirtualMachinesFilterEl {
    name: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl DataFsxOntapStorageVirtualMachinesFilterEl { }

impl ToListMappable for DataFsxOntapStorageVirtualMachinesFilterEl {
    type O = BlockAssignable<DataFsxOntapStorageVirtualMachinesFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachinesFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildDataFsxOntapStorageVirtualMachinesFilterEl {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachinesFilterEl {
        DataFsxOntapStorageVirtualMachinesFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataFsxOntapStorageVirtualMachinesFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFsxOntapStorageVirtualMachinesFilterElRef {
    fn new(shared: StackShared, base: String) -> DataFsxOntapStorageVirtualMachinesFilterElRef {
        DataFsxOntapStorageVirtualMachinesFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachinesFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataFsxOntapStorageVirtualMachinesDynamic {
    filter: Option<DynamicBlock<DataFsxOntapStorageVirtualMachinesFilterEl>>,
}
