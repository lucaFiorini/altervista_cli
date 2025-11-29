pub use register_macro::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::ptr;
use std::sync::{Arc, LazyLock, RwLock};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;
use serde::ser::Error;

#[derive(Debug,Clone)]
pub struct AltervistaItem<T>(Arc<T>);
impl<T:HasRegistry<AltervistaId>> AltervistaItem<T>{
    pub fn get(id: &AltervistaId) -> Result<Self,String> {
        match T::get_registry().read().unwrap().get(id){
            None => Err(format!("Can't  find {} for id {}",std::any::type_name::<T>(),id)),
            Some(v) => Ok(Self(v.clone()))
        }
    }
}
impl<T> Deref for AltervistaItem<T>{
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Serialize for AltervistaItem<T>
where
    T:HasRegistry<AltervistaId>,
    T:'static
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self.0.get_registry_key() {
            None => {Err(S::Error::custom(format!("Could not find ID for {}",std::any::type_name::<Self>())))}
            Some(id) => {serializer.serialize_newtype_struct("foo",&id)}
        }
    }
}

impl<'de,T> Deserialize<'de> for AltervistaItem<T>
where
    T: HasRegistry<AltervistaId> + 'static,  // T needs to be deserializable and have a registry
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RegistryItemVisitor<T> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<'de,T> Visitor<'de> for RegistryItemVisitor<T>
        where
            T: HasRegistry<AltervistaId> + 'static,  // T must be deserializable and have a registry
        {
            type Value = AltervistaItem<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a valid id for {}", std::any::type_name::<T>())
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                // Expecting an AltervistaId (or a similar type) that we can use to look up the registry
                let id: AltervistaId = Deserialize::deserialize(deserializer)?;

                // Now we can try to fetch the object from the registry
                match T::get_registry().read().unwrap().get(&id) {
                    Some(arc_obj) => Ok(AltervistaItem(arc_obj.clone())),
                    None => Err(de::Error::custom(format!(
                        "Could not find {} for id {}",
                        std::any::type_name::<T>(),
                        id
                    ))),
                }
            }
        }

        deserializer.deserialize_newtype_struct("_", RegistryItemVisitor::<T> { _marker: std::marker::PhantomData })
    }
}

pub fn register_item<T,K>(key: K, val: T)
where
    T : HasRegistry<K>,
    K : Eq + Hash + Copy + 'static
{
    let mut reg = T::get_registry().write().unwrap();
    reg.insert(key, Arc::new(val));
}

pub type AltervistaId = u32;

pub trait HasRegistry<K> where
    K : Eq + Copy + Hash + 'static,
    Self: 'static
{
    fn get_registry() -> &'static LazyLock<RwLock<HashMap<K,Arc<Self>>>>;
    fn deserialize_and_register<'de,D>(deserializer: D) -> Result<Arc<Self>, D::Error>
        where
            Self: Deserialize<'de>,
            D: serde::Deserializer<'de>;
    fn deserialize_and_register_arr<'de,D>(deserializer: D) -> Result<Vec<Arc<Self>>, D::Error>
    where
        Self: Deserialize<'de> + HasRegistry<K>,
        D: serde::Deserializer<'de>
    {
        let mut vec : Vec<Self> = Vec::deserialize(deserializer)?;

        Ok(vec.drain(..).map(|val| {
            let key = val.get_raw_key();
            register_item(key,val);
            Self::get_from_reg(key).unwrap()
        }).collect())

    }


    fn get_registry_key(&self) -> Option<K> {
        let reg = Self::get_registry();

        for (id, v) in reg.read().unwrap().iter() {
            // Arc<T> stores T on the heap; pointer equality is valid
            if ptr::eq(&**v, self) {
                return Some(*id);
            }
        }
        None
    }
    fn get_raw_key(&self) -> K;
    fn get_from_reg(key : K) -> Option<Arc<Self>> {
        let binding = Self::get_registry().read().unwrap();
        let map : &HashMap<K,Arc<Self>> = binding.deref();
        match map.get(&key) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }
}